use std::{io::{Read, Write, Error}, net::TcpListener};
use log::info;

fn main() -> Result<(), Error> {
    let ip:&str = "127.0.0.1"; // localhost
    let port:&str = "8080";
    let listener = TcpListener::bind(format!("{ip}:{port}")).unwrap();
    println!("Server started at: {}:{}.", ip, port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_connection(stream));
            } Err(e) => {
                info!("Failed: {}", e);
            }
        }        
    }
    Ok(())
}

fn handle_connection(mut stream: std::net::TcpStream) {
    // Read request
    let mut buf = [0; 1024];
    stream.read(&mut buf).expect("Failed to read");
    let request = String::from_utf8_lossy(&buf[..]);
    println!("Recieved request: {}", request);

    // Send response
    let body = r"
    TESTBODYTESTBODYTESTBODY
    TESTBODYTESTBODYTESTBODY
    TESTBODYTESTBODY
    TESTBODY";
    let length = body.len();
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{body}\r\n");
    stream.write(response.as_bytes())
        .expect("Failed to write");
}