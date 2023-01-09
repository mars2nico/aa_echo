// Non-blocking echo server using epoll.

use std::collections::HashMap;
use std::io::prelude::*;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

#[allow(dead_code)]
struct ClientState {
    stream: TcpStream,
    buf: [u8; 1024],
    buf_cursor: usize,
}

const SERVER: Token = Token(0);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(32);
    let mut listner = TcpListener::bind("127.0.0.1:8080".parse()?)?;
    let mut clients: HashMap<usize, ClientState> = HashMap::new();

    poll.registry().register(&mut listner, SERVER, Interest::READABLE)?;

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    /* Accept a new connection */
                    let (mut client, addr) = listner.accept()?;

                    // *CAUTION* This is ad-hoc code to make a token by the `port` then
                    //           have not considered for ordinary usecase.
                    let t = addr.port().into();

                    // *NOTE* The method reregistering each `Interest::*` doesnot work for some reason, so
                    //        modify initially set both; `READABLE` and `WRITABLE`.
                    poll.registry()
                        .register(&mut client, Token(t), Interest::READABLE | Interest::WRITABLE)?;
                    clients.insert(
                        t,
                        ClientState {
                            stream: client,
                            buf: [0u8; 1024],
                            buf_cursor: 0,
                        },
                    );
                }
                Token(t) => {
                    if let Some(client_state) = clients.get_mut(&t) {
                        /* Handle a client */
                        if event.is_readable() {
                            client_state.buf_cursor = client_state.stream.read(&mut client_state.buf)?;
                            if client_state.buf_cursor == 0 {
                                break;
                            }
                        }
                        if event.is_writable() {
                            client_state.stream.write(client_state.buf[..client_state.buf_cursor].as_ref())?;
                        }
                    }
                }
            }
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}
