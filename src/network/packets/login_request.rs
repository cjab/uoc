use std::io::prelude::*;
use std::io::{Cursor};

use network::packets::Packet;

pub struct LoginRequest<'a> {
    pub username: &'a str,
    pub password: &'a str
}

impl<'a> Packet for LoginRequest<'a> {
    fn packet_id() -> u8 { 0x80 }

    fn bytes(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; 62];
        let mut cursor = Cursor::new(buffer);

        cursor.write(&[Self::packet_id()]);
        cursor.write(&self.username.as_bytes());
        cursor.set_position(31);
        cursor.write(&self.password.as_bytes());
        cursor.set_position(61);
        cursor.write(&[0x00]);

        cursor.into_inner()
    }
}
