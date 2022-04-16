use std::thread;
use std::time::Duration;
use std::{convert::Infallible, io::prelude::*};

use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    //let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    thread::sleep(Duration::from_secs(5));
    let mut buffer = [0; 1024];
    stream.write(&buffer).await.unwrap();

    let response = "GREETINGS FROM THE SERVER\r\n";
    stream.write(response.as_bytes()).await.unwrap();

    return stream.flush().await.unwrap();
}
//
