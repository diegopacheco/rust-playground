use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    match TcpStream::connect("127.0.0.1:8888") {
        Ok(mut stream) => {
            println!("Connected to server!");
            
            loop {
                let mut input = String::new();
                println!("Enter two numbers separated by space (or 'quit' to exit):");
                
                std::io::stdin().read_line(&mut input).unwrap();
                
                if input.trim().to_lowercase() == "quit" {
                    break;
                }
                
                stream.write(input.as_bytes()).unwrap();
                
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(n) => {
                        let response = String::from_utf8_lossy(&buffer[..n]);
                        print!("Server response: {}", response);
                    }
                    Err(e) => {
                        println!("Failed to receive response: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}