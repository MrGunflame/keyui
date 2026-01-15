use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::process::Stdio;
use std::str::FromStr;
use std::sync::OnceLock;

use bstr::BString;
use futures::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use futures::{pin_mut, select, FutureExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

static CHANNEL: OnceLock<mpsc::UnboundedSender<(String, Value, oneshot::Sender<Value>)>> =
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
    let (tx, rx) = mpsc::unbounded_channel::<(String, Value, oneshot::Sender<Value>)>();
    CHANNEL.set(tx).unwrap();

    std::thread::spawn(move || {
        let mut jar_path = std::env::current_dir().unwrap();
        jar_path.push("api.jar");

        async_io::block_on(async move {
            if let Err(err) = run_cmd(&jar_path, rx).await {
                eprintln!("Failed to start prover: {}", err);
                eprintln!(
                    "Please ensure java is available in $PATH and the api is placed at {}",
                    jar_path.to_string_lossy()
                );
                std::process::exit(1);
            }
        });
    });
}

async fn run_cmd(
    jar_path: impl AsRef<OsStr>,
    mut channel: mpsc::UnboundedReceiver<(String, Value, oneshot::Sender<Value>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = async_process::Command::new("java")
        .arg("-jar")
        .arg(&jar_path)
        .arg("--std")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut writer = Writer::new(stdin);
    let mut reader = Reader::new(stdout);

    let status_fut = child.status().fuse();
    pin_mut!(status_fut);

    let mut next_id: u64 = 0;

    // TODO: Timeouts
    let mut inflight = HashMap::new();

    loop {
        select! {
            res = status_fut => {
                let status = res?;
                if !status.success() {
                    return Err("process exited with non-zero status code".into());
                } else {
                    return Ok(());
                }
            }
            res = channel.recv().fuse() => {
                let Some((method, params, resp_tx)) = res else {
                    return Ok(());
                };

                let id = next_id;
                next_id += 1;

                writer.send(&method, params, id).await.unwrap();

                inflight.insert(id, resp_tx);
            }
            msg = reader.recv().fuse() => {
                let msg = msg.unwrap();

                match msg {
                    Message::Response { id, msg } => {
                        let Some(id) = id else {
                            continue;
                        };

                        let Some(resp_tx) = inflight.remove(&id) else {
                            continue;
                        };

                        resp_tx.send(msg).ok();
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Writer<W> {
    writer: W,
}

impl<W> Writer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub async fn send<T>(&mut self, method: &str, req: T, id: u64) -> Result<(), io::Error>
    where
        T: Serialize,
        W: AsyncWrite + Unpin,
    {
        let req = Request {
            jsonrpc: Cow::Borrowed("2.0"),
            method: Cow::Borrowed(method),
            id: Some(id),
            params: Some(req),
        };

        let buf = serde_json::to_string(&req).unwrap();

        dbg!("TX", serde_json::from_str::<Value>(&buf).unwrap());

        let framed = format!("Content-Length: {}\r\n\r\n{}", buf.len(), buf);
        dbg!(&framed);

        self.writer.write_all(framed.as_bytes()).await?;
        Ok(())
    }
}

#[derive(Debug)]
struct Reader<R> {
    reader: R,
    buf: Vec<u8>,
    bytes_init: usize,
}

impl<R> Reader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: vec![0; 256],
            bytes_init: 0,
        }
    }

    pub async fn recv(&mut self) -> Result<Message, ReadError>
    where
        R: AsyncRead + Unpin,
    {
        loop {
            match parse_message(&self.buf[..self.bytes_init]) {
                Ok((msg, bytes_consumed)) => {
                    let resp =
                        serde_json::from_slice::<Message>(msg).map_err(ReadError::InvalidJson)?;
                    dbg!(&resp);

                    debug_assert!(self.bytes_init >= bytes_consumed);
                    self.buf.copy_within(bytes_consumed.., 0);
                    self.bytes_init -= bytes_consumed;

                    return Ok(resp);
                }
                Err(ParseError::NeedMoreData) => {}
                Err(err) => return Err(ReadError::Parse(err)),
            }

            if self.buf.len() - self.bytes_init < 256 {
                self.buf.resize(self.buf.len() * 2, 0);
                debug_assert!(self.buf.len() - self.bytes_init >= 256);
            }

            let bytes_read = match self.reader.read(&mut self.buf[self.bytes_init..]).await {
                Ok(0) => return Err(ReadError::Io(io::ErrorKind::UnexpectedEof.into())),
                Ok(n) => n,
                Err(err) => return Err(ReadError::Io(err)),
            };

            self.bytes_init += bytes_read;

            dbg!(bstr::BStr::new(&self.buf[..self.bytes_init]));
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Message {
    /// Response to a previous [`Request`].
    Response {
        id: Option<u64>,
        #[serde(flatten)]
        msg: Value,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseMessage {
    #[serde(rename = "result")]
    Result(Value),
    #[serde(rename = "error")]
    Err(Value),
    #[serde(untagged)]
    Null(Value),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Request<'a, T> {
    jsonrpc: Cow<'static, str>,
    method: Cow<'a, str>,
    id: Option<u64>,
    params: Option<T>,
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
    #[error("unexpected token {found}, expected {expected}")]
    UnexpectedToken { expected: BString, found: BString },
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
                expected: BString::new(lit.to_vec()),
                found: BString::new(self.data.to_vec()),
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
