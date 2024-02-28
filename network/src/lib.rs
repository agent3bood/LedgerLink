use node::node::Node;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Error as IoError;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

pub struct Network<'a> {
    node: &'a Node,
}

impl Network<'_> {
    pub fn new(node: &Node) -> Network {
        Network { node }
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind("0.0.0.0:8080").await?;

        loop {
            let (mut stream, addr) = listener.accept().await?;
            let request = read_http_request(&mut stream, &addr).await?;

            task::spawn(async move {
                let response = handle_request(request).await;
                wite_http_response(&mut stream, &addr, response).await;
            });
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Payload {}

async fn read_http_request(stream: &mut TcpStream, addr: &SocketAddr) -> Result<Payload, IoError> {
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut byte = [0; 1];
    let mut buf = Vec::new();
    loop {
        let n = stream.read(&mut byte).await?;
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

    println!("Received request from {}: {:?}", addr, headers);

    let content_length = headers
        .get("Content-Length")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut buf = vec![0; content_length];
    stream.read_exact(&mut buf).await?;

    Ok(Payload {})
}

async fn handle_request(request: Payload) -> Result<Payload, IoError> {
    // Implementation details:
    // 1. Match the request method and path to the appropriate handler.
    // 2. Call the handler and pass the request data to it.
    // 3. Handle any errors that occur during the process.
    Ok(Payload {})
}

async fn wite_http_response(
    stream: &mut TcpStream,
    addr: &SocketAddr,
    response: Result<Payload, IoError>,
) {
    stream.write_all(b"HTTP/1.1 200 OK\r\n").await.unwrap();
    // Implementation details:
    // 1. Convert the response data into a raw byte stream.
    // 2. Write the raw byte stream to the TcpStream.
    // 3. Handle any errors that occur during the process.
}
