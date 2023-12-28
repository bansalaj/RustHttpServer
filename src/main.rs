
use std::io::BufRead;
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

impl HttpRequest {
    fn print_request(&self) {
        println!("method: {} , path: {}", self.method,self.path);

        for(key, value) in&self.headers {
            println!("{}: {}", key, value);
        }
    }
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

fn build_response(request: &HttpRequest,body: &str, directory: Option<&str>) -> String {

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
    } else if request.method == "POST" {
        if request.path.starts_with("/files/") {
            if let Some(dir) = directory {
                let relative_path = &request.path[7..];
                let file_path = std::path::Path::new(dir).join(relative_path);

                let result = std::fs::write(&file_path, body);

                return match result {
                    Ok(_) => "HTTP/1.1 201 Created\r\n\r\n".to_string(),
                    Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\n".to_string(),
                };
            }
        }
    }

    "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
}

fn handle_client(mut stream: TcpStream, directory: Option<&str>) {
    let mut reader = BufReader::new(&stream);
    let mut req_line = String::new();

    while let Ok(bytes_read) = reader.read_line(&mut req_line) {
        if bytes_read == 0 || req_line.ends_with("\r\n\r\n") {
            break;
        }        
    }

    // Parse the request lines and headers
    let request = parse_request(&req_line);
    request.print_request();

    let mut body = String::new();

    if request.method == "POST" {
        if let Some(content_length) = request.headers.get("Content-Length") {
            if let Ok(length) = content_length.parse::<usize>() {
                let mut body_buff = vec![0; length];
                reader.read_exact(&mut body_buff).unwrap();
                body = String::from_utf8(body_buff).unwrap();
            }
        }
    }

    let response = build_response(&request, &body, directory);

    stream.write_all(response.as_bytes()).unwrap();
    
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
