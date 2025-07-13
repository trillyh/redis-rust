// This version uses mini-redis lib, follow the tokio's guide


use tokio::net::{TcpListener};

#[tokio::main]
async fn main() { 

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _addr) = listener.accept().await.unwrap();
    }

}
