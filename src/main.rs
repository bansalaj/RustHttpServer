// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Read, Write, BufReader};


fn build_response(path: &str) -> String {
    if path == "/" {
        "HTTP/1.1 200 OK\r\n\r\n".to_string()
    } else if path.starts_with("/echo/") {
        let content = &path[6..];
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        )
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut buffer = [0; 1024];

    if let Ok(_) = reader.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer);
        let req_lines: Vec<&str> = request.lines().collect();

        if let Some(first_line) = req_lines.get(0) {
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() > 1 {
                let path = parts[1];

                let response = build_response(path);
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    }
    stream.flush().unwrap();
}

fn open_connection(ipaddr: &str) {
    let listener = TcpListener::bind(ipaddr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("Accepted new connection");
                handle_client(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    open_connection("127.0.0.1:4221")
}
