use std::{
    fs,
    io::{ prelude::*, BufReader },
    net::{ TcpListener, TcpStream },
};
use server_1::ThreadPool;

fn main() {
    let ip = "127.0.0.1:9001";
    println!("Connected to: {ip}");
    let listener = TcpListener::bind(ip).unwrap();

    let pool = ThreadPool::build(4).unwrap();

    for stream in listener.incoming().take(20) {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream) );
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename, content_type) = match request_line.as_ref() {
        "GET /api/report-demo.json HTTP/1.1" => {
            //println!("WORKING");
            ("HTTP/1.1 200 OK", "report-demo.json", "application/json")
        }
        "GET /api/jenny-report.json HTTP/1.1" => {
            //println!("WORKING");
            ("HTTP/1.1 200 OK", "jenny-report.json", "application/json")
        }
        "GET /api/jenny-photo.jpg HTTP/1.1" => {
            //println!("Got Jenny");
            ("HTTP/1.1 200 OK", "jenny-photo.jpg", "image/jpeg")
        }
        _ => {
            //println!("DOESNT");
            ("HTTP/1.1 404 Not Found", "404.html", "")
        }
    };
    let file = fs::read(filename).unwrap();
    let length = file.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n");
    let bytes: Vec<u8> = [response.as_bytes(), &file].concat();
    stream.write_all(&bytes).unwrap();
}
