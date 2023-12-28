
// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Read, Write, BufReader};
use std::thread;
use clap::{App, Arg};


struct HttpRequest {
    method: String,
    path: String,
    headers: std::collections::HashMap<String, String>,
}


fn parse_request(request_str: &str) -> HttpRequest {
    let lines : Vec<&str> = request_str.lines().collect();
    let mut headers = std::collections::HashMap::new();

    // Parse the request line
    let (method, path) = if let Some(first_line) = lines.get(0) {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        (parts.get(0).unwrap_or(&"").to_string(), parts.get(1).unwrap_or(&"").to_string())
    } else {
        (String::new(), String::new())
    };

    // Parse headers
    for line in &lines[1..] {
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    HttpRequest{
        method,
        path,
        headers
    }
}

fn build_response(request: &HttpRequest, directory: Option<&str>) -> String {

    if request.method == "GET" {
        if request.path == "/" {
            return "HTTP/1.1 200 OK\r\n\r\n".to_string();
        } else if request.path.starts_with("/echo/") {
            let content = &request.path[6..];
            return format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                content.len(),
                content
            );
        } else if request.path == "/user-agent" {
            if let Some(user_agent) = request.headers.get("User-Agent") {
                return format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    user_agent.len(),
                    user_agent
                );
            }
        } else if request.path.starts_with("/files/") {
            if let Some(dir) = directory {
                let relative_path = &request.path[7..];
                let file_path = std::path::Path::new(dir).join(relative_path);

                if file_path.exists() && file_path.is_file() {
                    if let Ok(contents) = std::fs::read_to_string(&file_path) {
                        return format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-length: {}\r\n\r\n{}",
                            contents.len(),
                            contents
                        );
                    }
                }
            }
        }
    }

    "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
}

fn handle_client(mut stream: TcpStream, directory: Option<&str>) {
    let mut reader = BufReader::new(&stream);
    let mut buffer = [0; 1024];

    if let Ok(_) = reader.read(&mut buffer) {
        let request = parse_request(&String::from_utf8_lossy(&buffer));
        let response = build_response(&request, directory);

        stream.write_all(response.as_bytes()).unwrap();
    }
    stream.flush().unwrap();
}

fn open_connection(ipaddr: &str, directory: Option<String>) {
    let listener = TcpListener::bind(ipaddr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                let client_addr = _stream.peer_addr().unwrap();
                println!("Accepted new connection {}", client_addr);
                let dir_clone = directory.clone();
                // spawn a new thread for each connection
                thread::spawn(move || {
                    handle_client(_stream, dir_clone.as_deref());
                });
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
    let matches = App::new("Http Server")
        .version("1.0")
        .author("Ajit Bansal")
        .about("simple Rust http server")
        .arg(Arg::with_name("directory")
                .long("directory")
                .value_name("DIRECTORY")
                .help("sets a custom directory")
                .takes_value(true))
        .get_matches();

    let directory = matches.value_of("directory").map(String::from);

    open_connection("127.0.0.1:4221", directory)
}
