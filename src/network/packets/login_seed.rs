use std::io::prelude::*;

use network::packets::Packet;

pub struct LoginSeed;

impl Packet for LoginSeed {
    fn packet_id() -> u8 { 0xff }

    fn bytes(&self) -> Vec<u8> {
        vec![0x7f, 0x0c, 0x22, 0x38]
    }
}
