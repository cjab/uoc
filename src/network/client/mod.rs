use std::io::prelude::*;
use std::io::{self};
use std::net::TcpStream;

use network::packets::{Packet, LoginSeed, LoginRequest, ServerList};

pub struct Client {
    stream: TcpStream
}

impl Client {
    pub fn connect(address: &str) -> Result<Client, io::Error> {
        let mut stream = try!(TcpStream::connect(address));
        Ok(Client { stream: stream })
    }

    pub fn login(&mut self, username: &str, password: &str) -> Result<(), io::Error> {
        let seed_packet = LoginSeed;
        let login_packet = LoginRequest {
            username: username,
            password: password
        };
        self.stream.write(&seed_packet.bytes());
        self.stream.write(&login_packet.bytes());
        Ok(())
    }

    pub fn next_packet(&mut self) {
        let mut buffer = vec![0; 128];
        self.stream.read(&mut buffer[..]);
        let packet = match buffer[0] {
            0xa8 => {
                let p = ServerList::parse(&buffer[..]).unwrap();
                println!("list {} {}", p.size, p.flags);
                for s in p.servers {
                    println!("{} {} {} {} {}", s.index, s.name, s.percent_full, s.timezone, s.address);
                }
                ()
            },
            _ => ()
        };
        println!("READING {}", buffer[0]);
    }
}
