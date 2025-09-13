use http::ThreadPool;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

fn parse_info(stream: &mut TcpStream) {
    println!("accepted new connection");

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap_or(0);

    if bytes_read == 0 {
        return;
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let mut headers_and_body = request.split("\r\n\r\n");
    let headers_part = headers_and_body.next().unwrap_or("");
    let body_part = headers_and_body.next().unwrap_or("");

    let mut lines = headers_part.lines();
    let request_line = lines.next().unwrap_or("");
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }
    let method = parts[0];
    let path = parts[1];

    // Collect headers
    let headers: Vec<&str> = lines.collect();

    // Find content length if it's a POST
    let content_length = headers
        .iter()
        .find(|h| h.to_lowercase().starts_with("content-length:"))
        .and_then(|h| h.splitn(2, ':').nth(1))
        .and_then(|len| len.trim().parse::<usize>().ok())
        .unwrap_or(0);

    let mut body = body_part.to_string();

    while body.len() < content_length {
        let mut buf = vec![0; content_length - body.len()];
        let n = stream.read(&mut buf).unwrap_or(0);
        if n == 0 {
            break;
        }
        body.push_str(&String::from_utf8_lossy(&buf[..n]));
    }

    //Routing
    match (method, path) {
        ("GET", "/") => {
            respond_text(stream, 200, "OK", "Welcome to my tiny server!");
        }
        ("GET", p) if p.starts_with("/echo/") => {
            let msg = p.trim_start_matches("/echo/");
            respond_text(stream, 200, "OK", msg);
        }
        ("GET", "/user-agent") => {
            if let Some(ua) = headers
                .iter()
                .find(|h| h.to_lowercase().starts_with("user-agent:"))
            {
                let ua_value = ua.splitn(2, ':').nth(1).unwrap_or("").trim();
                respond_text(stream, 200, "OK", ua_value);
            } else {
                respond_text(stream, 400, "Bad Request", "No User-Agent found");
            }
        }
        ("GET", p) if p.starts_with("/files/") => {
            let filename = p.trim_start_matches("/files/");
            let file_path = Path::new("tmp").join(filename);
            if let Ok(contents) = fs::read_to_string(&file_path) {
                respond_bytes(stream, 200, "OK", contents.as_bytes());
            } else {
                respond_text(stream, 404, "Not Found", "File not found");
            }
        }
        ("POST", p) if p.starts_with("/files/") => {
            let filename = p.trim_start_matches("/files/");
            let file_path = Path::new("tmp").join(filename);
            if let Err(e) = fs::write(&file_path, body) {
                eprintln!("file write error: {e}");
                respond_text(stream, 500, "Internal Server Error", "Could not write file");
            } else {
                respond_text(stream, 201, "Created", "");
            }
        }
        _ => {
            respond_text(stream, 404, "Not Found", "Unknown route");
        }
    }
}

fn respond_text(stream: &mut TcpStream, code: u16, status: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        code,
        status,
        body.len(),
        body
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn respond_bytes(stream: &mut TcpStream, code: u16, status: &str, body: &[u8]) {
    let headers = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
        code,
        status,
        body.len()
    );
    stream.write_all(headers.as_bytes()).unwrap();
    stream.write_all(body).unwrap();
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                pool.execute(move || parse_info(&mut stream));
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}
