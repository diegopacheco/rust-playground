use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    
    while match stream.read(&mut buffer) {
        Ok(n) if n == 0 => false,
        Ok(n) => {
            let message = String::from_utf8_lossy(&buffer[..n]);
            let numbers: Vec<i32> = message
                .trim()
                .split(' ')
                .filter_map(|s| s.parse().ok())
                .collect();
            
            if numbers.len() == 2 {
                let result = numbers[0] + numbers[1];
                let response = format!("Result: {} + {} = {}\n", numbers[0], numbers[1], result);
                stream.write(response.as_bytes()).unwrap();
            }
            true
        }
        Err(_) => {
            println!("Error reading from connection");
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    println!("Server listening on port 8888");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection established!");
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}