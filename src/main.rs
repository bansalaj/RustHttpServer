// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Read, Write, BufReader};

fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut buffer = [0; 1024];

    match reader.read(&mut buffer){
        Ok(_) => {
            println!("Received a request!");
        }
        Err(e) => println!("Failed to read from connection: {}", e),
    }

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //

    //let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_client(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
