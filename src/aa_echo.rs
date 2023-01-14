// async/await non-blocking echo server

use std::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 1024];
    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        stream.write(&buf[..n]).await?;
    }
    Ok(())
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listner = TcpListener::bind("127.0.0.1:8080").await?;

    while let Ok((stream, _)) = listner.accept().await {
        tokio::spawn(async move {
            handle_client(stream).await
        });
    }

    Ok(())
}
