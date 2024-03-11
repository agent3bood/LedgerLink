use k256::ecdsa::VerifyingKey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Error as IoError;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;
use utils::Utils;

pub struct Network {
    neighbors: HashMap<String, SocketAddr>,
    node: String,
    rx: tokio::sync::mpsc::Receiver<String>,
    node_tx: tokio::sync::mpsc::Sender<String>,
}

impl Network {
    pub fn new(
        node: String,
        rx: tokio::sync::mpsc::Receiver<String>,
        node_tx: tokio::sync::mpsc::Sender<String>,
    ) -> Network {
        Network {
            node,
            neighbors: HashMap::new(),
            rx,
            node_tx,
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("0.0.0.0:8080").await?;

        let network = Arc::new(self);
        //
        // TODO listen to rx
        //
        //
        loop {
            let (stream, addr) = listener.accept().await?;
            let connection = Connection {
                node: network.clone().node.clone(),
                node_tx: network.clone().node_tx.clone(),
                network: network.clone(),
                stream,
                addr,
                from_key: None,
                request: None,
                response: None,
            };
            task::spawn(async move {
                connection.process().await;
            });
        }
    }
}

struct Connection {
    node: String,
    node_tx: tokio::sync::mpsc::Sender<String>,
    network: Arc<Network>,
    stream: TcpStream,
    addr: SocketAddr,
    from_key: Option<VerifyingKey>,
    request: Option<Request>,
    response: Option<Response>,
}

impl Connection {
    async fn process(mut self) {
        self.read_request().await.unwrap_or(());
        self.handle_request().await.unwrap_or(());
        self.write_response().await.unwrap_or(());
    }
    async fn read_request(&mut self) -> Result<(), IoError> {
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut byte = [0; 1];
        let mut buf = Vec::new();
        loop {
            let n = self.stream.read(&mut byte).await?;
            if n == 0 {
                break;
            }
            buf.push(byte[0]);
            if buf.strip_suffix(b"\r\n\r\n").is_some() {
                buf.pop();
                buf.pop();
                buf.pop();
                buf.pop();
                break;
            }
        }

        for line in String::from_utf8(buf).unwrap().split("\r\n") {
            let mut parts = line.split(": ");
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            headers.insert(key.into(), value.into());
        }

        // TODO use logger
        println!("Received request from {}: {:?}", self.addr, headers);

        let content_length = headers
            .get("Content-Length")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut buf = vec![0; content_length];
        self.stream.read_exact(&mut buf).await?;

        let request = serde_json::from_slice(&buf).unwrap();

        self.request = Some(request);
        Ok(())
    }

    async fn handle_request(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let request = self.request.as_ref().unwrap();

        // to
        if request.to != self.node {
            self.response = Some(Response {
                status: 500,
                from: self.node.clone(),
                signature: "".into(),
                message: "Unknown recipient".into(),
            });
            return Ok(());
        }
        // from
        if !self.network.neighbors.contains_key(&request.from) {
            self.response = Some(Response {
                status: 500,
                from: self.node.clone(),
                signature: "".into(),
                message: "Unknown sender".into(),
            });
            return Ok(());
        }
        // signature
        let from_key = Utils::get_verifying_key(&request.from)?;
        let from_signature = Utils::decode_signature(&request.signature)?;
        if !Utils::verify_signature(&request.from, &from_signature, &from_key) {
            self.response = Some(Response {
                status: 500,
                from: self.node.clone(),
                signature: "".into(),
                message: "Invalid signature".into(),
            });
            return Ok(());
        }

        self.node_tx.send(request.message.clone()).await?;
        self.response = Some(Response {
            status: 200,
            from: self.node.clone(),
            signature: "".into(), // TODO
            message: "OK".into(),
        });

        Ok(())
    }

    async fn write_response(&mut self) -> Result<(), IoError> {
        match &self.response {
            Some(response) => {
                self.stream
                    .write_all(format!("{}", serde_json::to_string(response)?).as_bytes())
                    .await?;
            }
            None => {}
        };
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
enum Message {
    Request(Request),
    Response(Response),
}

#[derive(Deserialize, Serialize)]
struct Request {
    /// The recipient node public key
    to: String,
    /// The sender node public key
    from: String,
    /// THe sender node public key signed by the sender node private key
    signature: String,
    /// The message to be opened by the node
    message: String,
}

#[derive(Deserialize, Serialize)]
struct Response {
    /// The status code of the response
    status: usize,
    /// The sender node public key
    from: String,
    /// The sender node public key signed by the sender node private key
    signature: String,
    /// The message to be opened by the recipient node
    message: String,
}
