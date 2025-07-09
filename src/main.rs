#![allow(unused_imports)]
use std::{io::Write, io::Read, net::TcpListener, net::TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let mut buf = [0; 512];
                
                loop {
                    let bytes_read = stream.read(&mut buf).unwrap();

                    if bytes_read == 0 {
                        return;
                    }
                    stream.write_all(b"+PONG\r\n").unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
