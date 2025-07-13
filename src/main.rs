use tokio::{
    io::{AsyncReadExt, AsyncWriteExt}, 
    net::{TcpListener, TcpStream}
};
// use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    println!("Logs from your program will appear here!");
  
    let listener = TcpListener::bind("127.0.0.1:6379").await.expect("Failed to create listener");
    
    loop {
        match listener.accept().await {
            Ok((socket, _addr)) => {
                println!("Accepting new connection)");
                let handle = tokio::spawn(async move {
                    process(socket).await;
                });
            } 
            Err(e) => {
                println!("Failed to accept new socket: error {:?}", e)
            }
        }
    }
}

// Process any incoming connections
async fn process(mut socket: TcpStream) {
    let mut buf = [0; 512];

    loop {
        let _bytes_read= match socket.read(&mut buf).await {
            
            Ok(0) => {
                println!("client closed connection");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                println!("Failed to read from socket; err = {:?}", e);
                break;
            }
        };

        // Doc why I would not want to panic! with expect() or unwrap() here.
        if let Err(e) = socket.write_all(b"+PONG\r\n").await {
            println!("Failed to write to socket; err = {:?}", e);
            break;
        }
    }
}
