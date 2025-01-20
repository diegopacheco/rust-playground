use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer);
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello from Rust server!";
    let _ = stream.write(response.as_bytes());
}

fn main() -> std::io::Result<()> {
    dbg!("Server started at http://127.0.0.1:8080");
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        if let Ok(s) = stream {
            handle_client(s);
        }
    }
    Ok(())
}