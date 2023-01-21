use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use bytes::BytesMut;
use tokio::time::{Duration, sleep};

pub struct Connection {
    stream: Option<TcpStream>, 
    buf: BytesMut,
}

#[derive(Debug)]
pub struct HttpRequest {
    method: String,
    uri: String,
    version: String,
    content_type: Option<String>,
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

#[derive(Debug)]
pub enum RequestError {
    UnknownLength,
    FailedToReadRequest,
    CouldNotParseToString,
    EmptyRequest,
    NoMethod,
    UriAbsent,
    NoVersion,
    NoRequestLine,
    NoHeaders,
    NoTcpStream,
    ConnectionClosed,
    PostTooLarge,
    Timeout,
}

impl Connection {
    pub async fn new(stream: TcpStream) -> Self  {
        let buf = BytesMut::with_capacity(4096);
        let stream = Some(stream);
        Connection { stream, buf }
    }

    async fn get_bytes(mut self) -> Result<Self, RequestError> {
        let mut check = false;
        loop {
            if self.buf.capacity() > 100000 { return Err(RequestError::PostTooLarge)  }
            match self.stream.as_mut().unwrap().read_buf(&mut self.buf).await {
                Ok(_) => {
                    tokio::select! {
                        _ = sleep(Duration::new(3, 0)) => { 
                            return Err(RequestError::Timeout) 
                        }
                        end = self.check_buf() => {
                            if end { 
                                if check || self.buf.len() > 2 && &self.buf[..3] == b"GET" { break }
                                check = true;
                                continue;
                            }
                        }
                    };
                },
                Err(_) => return Err(RequestError::ConnectionClosed)
            }
        }
        Ok(self)
    }

/*
    async fn get_length(&self) -> usize {
        let cursor = self.buf.len() - 15;
        let lines: Vec<&[u8]> = self.buf[cursor..].split(|x| &[*x] == b"\n" ).collect();
        for line in lines {
            let mut line = line.split(|x| *x == b' ').into_iter();
            if line.next().unwrap() == b"Content-Length" {
                return line.next().expect("GETIING LENGTH")[0] as usize
            }
        }
        0
    }
*/

    pub async fn read_connection(self) -> Result<Self, RequestError>  {
        let outer = sleep(Duration::new(5, 0));

        let inner = sleep(Duration::new(1, 0));
        tokio::select!{
            // Content-Length header == buf.len()
            _ = outer => {
                return Err(RequestError::Timeout)
            }
            _ = inner => {
                return Err(RequestError::Timeout)
            }
            read = self.get_bytes() => {
                return read
            }
        }
    }

    async fn check_buf(&self) -> bool {
        println!("{:?}", &self.buf[self.buf.len()-4..]);
        if self.buf.len() > 3 && &self.buf[self.buf.len()-4..] == b"\r\n\r\n" { 
            return true
        }
        false
    }

    pub async fn build_request(&mut self) -> Result<HttpRequest, RequestError> {

        let (request, body): (Vec<&str>, Option<&str>) = match std::str::from_utf8(&self.buf) {
                    Ok(request) => { 
                    // Separate request and body
                    let vec: Vec<&str> = request.split("\r\n\r\n").collect();
                    let mut iter = vec.into_iter();

                    // Split up request into components (headers and status line...)
                    if let Some(request) = iter.next() { ( request.split("\r\n").collect(), iter.next() ) }
                        else { return Err(RequestError::EmptyRequest) }
                },
                // Request was not valid utf8 (post data will be base64 encoded and compressed)
                Err(_) => return Err(RequestError::CouldNotParseToString),
        };

        // Turn the request into iterator to save internal position of parse
        let mut request_iter = request.iter();

        // Get request line (status line) and split into components
        let request_line: Vec<&str> = if let Some(request_line) = request_iter.next() { 
            request_line.split(' ').collect()
        } else { return Err(RequestError::EmptyRequest) };

        // Turn request line into iterator to save position of the parse
        let mut request_line_iter = request_line.into_iter();

        // Parse the request line
        let method = if let Some(method) = request_line_iter.next() { method.to_string() }
            else { return Err(RequestError::NoMethod) };
        let uri = if let Some(uri) = request_line_iter.next() { uri.to_string() }
            else { return Err(RequestError::UriAbsent) };
        let version = if let Some(version) = request_line_iter.next() { version.to_string() }
            else { return Err(RequestError::NoVersion) };

        // Map on the rest of the request to get the headers 
        let mut content_type: Option<String> = None;
        let headers: Vec<String> = request_iter 
                .map(|header| {
                    let mut split = header.split(' ').into_iter();
                    split.next().map(|name| if name == "Content-Type:" { content_type = split.next().map(|num| num.to_string() ) });
                    header.to_string()
                }).collect();

        let body: String = if let Some(body) = body { body.to_string() }
            else { "".to_string() };

        let stream = self.stream.take().unwrap();
        Ok(HttpRequest{method , uri, version, content_type, headers, body, stream })
    }

}

impl HttpRequest {
    
    pub async fn build(method: String, uri: String, version: String, content_type: Option<String>, headers: Vec<String>, body: String, stream: TcpStream ) -> HttpRequest {
        HttpRequest { method , uri, version, content_type, headers, body, stream }
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }

    pub fn headers(&self) -> Vec<String> {
        self.headers.clone()
    }

    pub async fn write(&mut self) -> tokio::io::Result<()> {
        let response = b"HTTP/1.1 200 OK\r\n";
        self.stream.write_all(response).await
    }

}


#[cfg(test)]
mod tests {
    use tokio::net::TcpListener;
    use super::Connection;

    #[tokio::test]
    async fn build_request() {
        let connection = TcpListener::bind("127.0.0.1:9000").await.expect("HERE");
        let (stream, _) = connection.accept().await.expect("CHILLI");
        let request = Connection::new(stream).await.read_connection().await.expect("Parsnips!!").build_request().await.expect("HUUUUHHH?");
        println!("{:?}", request);

        panic!("I Panicked!");
        
        // The request:- HttpRequest { method: "GET", uri: "/api", version: "HTTP/1.1", headers: ["Host: localhost:9000"], body: Some(""), stream: Take { inner: PollEvented { io: Some(TcpStream { addr: 127.0.0.1:9000, peer: 127.0.0.1:49164, fd: 10 }) }, limit_: 0 } })
    }


}

