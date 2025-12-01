use std::io::{self, PipeReader, PipeWriter, Read, Write};
use std::net::TcpStream;
use std::process::Command;
use std::str::FromStr;
use std::sync::{LazyLock, OnceLock};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::{mpsc, oneshot};

static CHANNEL: OnceLock<(mpsc::UnboundedSender<(String, Value, oneshot::Sender<Value>)>)> =
    OnceLock::new();

#[tauri::command]
pub async fn send_msg(method: String, params: Value) -> Value {
    let tx = CHANNEL.get().unwrap();

    let (resp_tx, resp_rx) = oneshot::channel();

    tx.send((method, params, resp_tx)).unwrap();
    let resp = resp_rx.await.unwrap();
    resp
}

pub fn start_prover() {
    let (stdin_rx, stdin_tx) = io::pipe().unwrap();
    let (stdout_rx, stdout_tx) = io::pipe().unwrap();

    std::thread::spawn(move || {
        let mut jar_path = std::env::current_dir().unwrap();
        jar_path.push("api.jar");

        let run_cmd = || -> Result<(), Box<dyn std::error::Error>> {
            let status = Command::new("java")
                .arg("-jar")
                .arg(&jar_path)
                .arg("--std")
                .stdin(stdin_rx)
                .stdout(stdout_tx)
                .spawn()?
                .wait()?;

            if !status.success() {
                Err("process exited with non-zero status code".into())
            } else {
                Ok(())
            }
        };

        if let Err(err) = run_cmd() {
            eprintln!("Failed to start prover: {}", err);
            eprintln!(
                "Please ensure java is available in $PATH and the api is placed at {}",
                jar_path.to_string_lossy()
            );
            std::process::exit(1);
        }
    });

    let mut conn = Connection::new(stdin_tx, stdout_rx);

    let (tx, mut rx) = mpsc::unbounded_channel::<(String, Value, oneshot::Sender<Value>)>();
    CHANNEL.set(tx).unwrap();

    std::thread::spawn(move || {
        async_io::block_on(async move {
            while let Some((method, params, tx)) = rx.recv().await {
                conn.send(&method, &params).unwrap();
                let resp = conn.recv().unwrap();
                tx.send(resp).unwrap();
            }
        });
    });
}

pub struct Connection {
    tx: PipeWriter,
    rx: PipeReader,
    buf: Vec<u8>,
    bytes_init: usize,
}

impl Connection {
    pub fn new(tx: PipeWriter, rx: PipeReader) -> Self {
        Self {
            tx,
            rx,
            buf: vec![0; 256],
            bytes_init: 0,
        }
    }

    pub fn send<T>(&mut self, method: &str, req: &T) -> Result<(), ()>
    where
        T: Serialize,
    {
        let req = Request {
            jsonrpc: "2.0",
            method,
            id: Some(1),
            params: Some(req),
        };

        let buf = serde_json::to_string(&req).unwrap();
        let framed = format!("Content-Length: {}\r\n\r\n{}", buf.len(), buf);
        self.tx.write_all(framed.as_bytes()).unwrap();
        Ok(())
    }

    pub fn recv(&mut self) -> Result<Value, io::Error> {
        loop {
            if self.buf.len() - self.bytes_init < 256 {
                self.buf.resize(self.buf.len() * 2, 0);
                debug_assert!(self.buf.len() - self.bytes_init >= 256);
            }

            let bytes_read = match self.rx.read(&mut self.buf[self.bytes_init..])? {
                0 => return Err(io::ErrorKind::UnexpectedEof.into()),
                n => n,
            };

            self.bytes_init += bytes_read;

            let (msg, bytes_consumed) = match parse_message(&self.buf[..self.bytes_init]) {
                Ok(v) => v,
                Err(Error::NeedMoreData) => continue,
            };

            let resp = serde_json::from_slice::<Response>(msg).unwrap();

            debug_assert!(self.bytes_init >= bytes_consumed);
            self.buf.copy_within(bytes_consumed.., 0);
            self.bytes_init -= bytes_consumed;

            return Ok(resp.result.unwrap_or(Value::Null));
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct Request<'a, T> {
    jsonrpc: &'static str,
    method: &'a str,
    id: Option<u64>,
    params: Option<&'a T>,
}

#[derive(Clone, Debug, Deserialize)]
struct Response {
    jsonrpc: String,
    id: Option<u64>,
    error: Option<Value>,
    result: Option<Value>,
}

#[derive(Copy, Clone, Debug)]
enum Error {
    NeedMoreData,
}

fn parse_message(data: &[u8]) -> Result<(&[u8], usize), Error> {
    let mut parser = Parser {
        data,
        bytes_read: 0,
    };

    // We cannot parse the head until complete.
    if memchr::memmem::find(parser.data, b"\r\n\r\n").is_none() {
        return Err(Error::NeedMoreData);
    }

    parser.consume_lit(b"Content-Length: ");
    let payload_len = parser.consume_number::<usize>();
    parser.consume_lit(b"\r\n\r\n");

    if parser.data.len() < payload_len {
        return Err(Error::NeedMoreData);
    }

    let payload = parser.advance(payload_len);

    Ok((payload, parser.bytes_read))
}

#[derive(Clone, Debug)]
struct Parser<'a> {
    data: &'a [u8],
    bytes_read: usize,
}

impl<'a> Parser<'a> {
    fn advance(&mut self, count: usize) -> &'a [u8] {
        let (data, rem) = self.data.split_at(count);
        self.data = rem;
        self.bytes_read += count;
        data
    }

    fn consume_lit(&mut self, lit: &[u8]) {
        let rem = self.data.strip_prefix(lit).unwrap();
        self.data = rem;
        self.bytes_read += lit.len();
    }

    fn consume_number<T>(&mut self) -> T
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
    {
        let end = self
            .data
            .iter()
            .position(|b| !matches!(b, b'0'..=b'9'))
            .unwrap_or(self.data.len());
        let num_str = std::str::from_utf8(&self.data[..end]).unwrap();

        let num = num_str.parse::<T>().unwrap();
        self.data = &self.data[end..];
        self.bytes_read += end;
        num
    }
}

struct Message {
    len: u64,
    method: String,
    data: String,
}
