// This version uses mini-redis lib, follow the tokio's guide
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() { 

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        match listener.accept().await {
            Ok((socket, _addr)) => {
                println!("Created a TcpListener");
                let handle = tokio::spawn(async move {
                    process(socket).await;
                });

            } Err(e) => {
                println!("Error occured when listening to Tcpstream");
            }
        }  
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    let mut db = HashMap::new();

    // Connection (mini-redis), handles parsing frames from the socket
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                println!("Received and set");
                Frame::Simple("OK".to_string())
            } Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    println!("Received and get");
                    Frame::Bulk(value.clone().into())
                } else {
                    println!("Unknown cmd");
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented! {:?}", cmd),

        };
        connection.write_frame(&response).await.unwrap();
    }
}
