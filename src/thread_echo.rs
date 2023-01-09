// Multi threaded echo server.
// Spawn a thread for a each connection.

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
        stream.write(buf[..n].as_ref())?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listner = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listner.incoming() {
        let stream = stream?;
        std::thread::spawn(move || {
            handle_client(stream).unwrap();
        });
    }

    Ok(())
}
