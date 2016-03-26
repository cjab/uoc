use std::io::prelude::*;
use std::io::{self};
use std::net::TcpStream;

use network::packets::{Packet, LoginSeed, LoginRequest, ServerList};

pub enum ClientState {
    Connected { stream: TcpStream, buffer: [u8; 4096] },
    Disconnected,
    Error(io::Error)
}

pub struct Client {
    state: ClientState
}

impl Client {
    pub fn connect(address: &str) -> Client {
        match TcpStream::connect(address) {
            Ok(stream) => {
                Client {
                    state: ClientState::Connected {
                        stream: stream,
                        buffer: [0; 4096]
                    }
                }
            },
            Err(e) => Client { state: ClientState::Error(e) }
        }
    }

    pub fn slice(&mut self) -> bool {
        match self.state {
            ClientState::Disconnected | ClientState::Error(_) => false,
            ClientState::Connected { ref stream, ref mut buffer } => {
                stream.take(4096).read(buffer);
                true
            }
        }
    }

    pub fn login(&mut self, username: &str, password: &str) -> Result<(), io::Error> {
        let seed_packet = LoginSeed;
        let login_packet = LoginRequest {
            username: username,
            password: password
        };
        let _ = match &mut self.state {
            &mut ClientState::Disconnected => (),
            &mut ClientState::Error(ref e) => (),
            &mut ClientState::Connected { ref mut stream, buffer: _ } => {
                stream.write(&seed_packet.bytes());
                stream.write(&login_packet.bytes());
            }
        };
        self.slice();
        let _ = match &self.state {
            &ClientState::Disconnected => (),
            &ClientState::Error(ref e) => (),
            &ClientState::Connected { ref stream, ref buffer } => {
                println!("{}", buffer[]);
            }
        };
        Ok(())
    }
}
