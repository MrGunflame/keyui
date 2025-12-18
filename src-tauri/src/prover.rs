use std::io::{self, PipeReader, PipeWriter, Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::sync::{LazyLock, OnceLock};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

static CHANNEL: OnceLock<(mpsc::UnboundedSender<(String, Value, oneshot::Sender<Message>)>)> =
    OnceLock::new();

#[tauri::command]
pub async fn send_msg(method: String, params: Value) -> Message {
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

    let (tx, mut rx) = mpsc::unbounded_channel::<(String, Value, oneshot::Sender<Message>)>();
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

    pub fn send<T>(&mut self, method: &str, req: &T) -> Result<(), io::Error>
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

        dbg!("TX", serde_json::from_str::<Value>(&buf).unwrap());

        let framed = format!("Content-Length: {}\r\n\r\n{}", buf.len(), buf);
        dbg!(&framed);

        self.tx.write_all(framed.as_bytes())?;
        Ok(())
    }

    pub fn recv(&mut self) -> Result<Message, ReadError> {
        loop {
            if self.buf.len() - self.bytes_init < 256 {
                self.buf.resize(self.buf.len() * 2, 0);
                debug_assert!(self.buf.len() - self.bytes_init >= 256);
            }

            let bytes_read = match self.rx.read(&mut self.buf[self.bytes_init..]) {
                Ok(0) => return Err(ReadError::Io(io::ErrorKind::UnexpectedEof.into())),
                Ok(n) => n,
                Err(err) => return Err(ReadError::Io(err)),
            };

            self.bytes_init += bytes_read;

            let (msg, bytes_consumed) = match parse_message(&self.buf[..self.bytes_init]) {
                Ok(v) => v,
                Err(ParseError::NeedMoreData) => continue,
                Err(err) => return Err(ReadError::Parse(err)),
            };

            let resp = serde_json::from_slice::<Response>(msg).map_err(ReadError::InvalidJson)?;
            dbg!(&resp);

            debug_assert!(self.bytes_init >= bytes_consumed);
            self.buf.copy_within(bytes_consumed.., 0);
            self.bytes_init -= bytes_consumed;

            let mut msg = Message::Null(Value::Null);
            if let Some(result) = resp.result {
                msg = Message::Result(result);
            }

            if let Some(err) = resp.error {
                msg = Message::Err(err);
            }

            return Ok(msg);
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum Message {
    #[serde(rename = "result")]
    Result(Value),
    #[serde(rename = "error")]
    Err(Value),
    #[serde(untagged)]
    Null(Value),
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

#[derive(Debug, Error)]
enum ReadError {
    #[error(transparent)]
    Io(io::Error),
    #[error(transparent)]
    Parse(ParseError),
    #[error("invalid json payload: {0}")]
    InvalidJson(serde_json::Error),
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("need more data")]
    NeedMoreData,
    #[error("unexpected token {found:?}, expected {expected:?}")]
    UnexpectedToken { expected: Vec<u8>, found: Vec<u8> },
    #[error("invalid string: {0}")]
    InvalidString(std::str::Utf8Error),
    #[error("invalid integer: {0}")]
    InvalidInt(std::num::ParseIntError),
}

fn parse_message(data: &[u8]) -> Result<(&[u8], usize), ParseError> {
    let mut parser = Parser {
        data,
        bytes_read: 0,
    };

    // We cannot parse the header until complete.
    if memchr::memmem::find(parser.data, b"\r\n\r\n").is_none() {
        return Err(ParseError::NeedMoreData);
    }

    parser.consume_lit(b"Content-Length: ")?;
    let payload_len = parser.consume_number::<usize>()?;
    parser.consume_lit(b"\r\n\r\n")?;

    if parser.data.len() < payload_len {
        return Err(ParseError::NeedMoreData);
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

    fn consume_lit(&mut self, lit: &[u8]) -> Result<(), ParseError> {
        let rem = self
            .data
            .strip_prefix(lit)
            .ok_or(ParseError::UnexpectedToken {
                expected: lit.to_vec(),
                found: self.data.to_vec(),
            })?;

        self.data = rem;
        self.bytes_read += lit.len();
        Ok(())
    }

    fn consume_number<T>(&mut self) -> Result<T, ParseError>
    where
        T: FromStr<Err = std::num::ParseIntError>,
    {
        let end = self
            .data
            .iter()
            .position(|b| !matches!(b, b'0'..=b'9'))
            .unwrap_or(self.data.len());
        let num_str = std::str::from_utf8(&self.data[..end]).map_err(ParseError::InvalidString)?;

        let num = num_str.parse::<T>().map_err(ParseError::InvalidInt)?;
        self.data = &self.data[end..];
        self.bytes_read += end;
        Ok(num)
    }
}
