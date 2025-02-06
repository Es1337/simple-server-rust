use std::{collections::HashMap, io::{Error, Read, Write}, net::TcpListener, str};
mod http_request;
use http_request::HttpRequest;

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
                println!("Failed: {}", e);
            }
        }        
    }
    Ok(())
}

fn handle_connection(mut stream: std::net::TcpStream) {
    // Read request
    let mut buf = [0; 1024];
    stream.read(&mut buf).expect("Failed to read");
    let request = String::from_utf8_lossy(&buf[..]).into_owned();
    println!("Recieved request: {}", request);

    let parsed_request = parse_request(buf.as_mut_slice());
    println!("Parsed request: {:?}", parsed_request);

    // Send response
    let body = "TESTBODYTESTBODYTESTBODY \
    TESTBODYTESTBODYTESTBODY \
    TESTBODYTESTBODY \
    TESTBODY";
    let length = body.len();
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{body}\r\n");
    stream.write(response.as_bytes())
        .expect("Failed to write");

}

fn parse_request(request: &[u8]) -> HttpRequest {
    let mut buf = String::new();
    let mut method = String::new();
    let mut path = String::new();
    let mut version = String::new();

    let mut prev_byte = 0u8;
    for byte_ref in request {
        let byte = *byte_ref;
        
        if byte == b'\r' && version.is_empty() {
            prev_byte = byte;
            continue;
        }

        if prev_byte == b'\r' && byte == b'\n' {
            version = buf.clone();
            buf.clear();
            break;
        }

        if method.is_empty() && byte == b' ' {
            method = buf.clone();
            buf.clear();
        } else if path.is_empty() && byte == b' ' {
            path = buf.clone();
            buf.clear();
        }

        if byte == b' ' {
            continue;
        }

        buf.push(char::from(byte));
    };

    HttpRequest::new(method, path, version, HashMap::new(), String::new())
}

#[test]
fn should_parse_request() {
    assert_eq!(parse_request("GET / HTTP/1.1\r\n".as_bytes()), 
        HttpRequest::new("GET".to_string(), 
            "/".to_string(), 
            "HTTP/1.1".to_string(), 
            HashMap::new(), 
            String::new()));
}