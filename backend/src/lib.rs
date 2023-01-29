pub mod parse_connection;
pub mod sql;

use serde::Deserialize;
use tokio::{
    net::TcpStream,
    io::AsyncWriteExt,
};

#[derive(Debug)]
pub struct HttpRequest {
    method: String,
    uri: String,
    version: String,
    headers: Vec<String>,
    body: String,
    stream: TcpStream,
}

pub struct HttpResponse {
    status: u16,
    version: String,
    headers: Vec<String>,
    body: String,
}

#[derive(Deserialize)]
pub struct Report {
    key: usize,
    name: String,
    photo: String,
    content: String,
}

impl HttpRequest {
    
    pub async fn build(method: String, uri: String, version: String, headers: Vec<String>, body: String, stream: TcpStream ) -> HttpRequest {
        HttpRequest { method , uri, version, headers, body, stream }
    }

    pub fn method(&self) -> String {
        self.method.clone()
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }

    pub fn headers(&self) -> Vec<String> {
        self.headers.clone()
    }

    pub async fn write(&mut self) -> tokio::io::Result<()> {
        let response = b"HTTP/1.1 200 OK\r\n\r\n";
        self.stream.write_all(response).await
    }

}

impl HttpResponse {

    pub async fn new() -> HttpResponse {
        HttpResponse {
            status: 0,
            version: "".to_string(),
            headers: vec![],
            body: "".to_string(),
        }
    }

    pub async fn build(self, request: HttpRequest) -> HttpResponse {
        if request.method() == "GET" { self.get().await }
            else { self.post().await }
    }

    pub async fn get(self) -> HttpResponse {
        todo!()
    }

    pub async fn post(self) -> HttpResponse {
        todo!()
    }

}

