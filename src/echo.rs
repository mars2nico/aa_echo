// Synchronous echo server.
// Handle only one connection at a time.

use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 1024];
    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            break;
        }
        stream.write(&buf[..n])?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listner = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listner.incoming() {
        let stream = stream?;
        handle_client(stream)?;
    }

    Ok(())
}
