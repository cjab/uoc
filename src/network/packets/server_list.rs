use std::io::prelude::*;
use std::io::{Cursor};
use std::net::{Ipv4Addr};
use std::str;
use byteorder::{ReadBytesExt, LittleEndian, BigEndian};

use network::packets::{Packet, Error};

pub struct ServerList {
    pub size: u16,
    pub flags: u8,
    pub servers: Vec<ServerEntry>
}

impl ServerList {
    pub fn parse(data: &[u8]) -> Result<ServerList, Error> {
        let mut cursor = Cursor::new(data);
        let id = try!(cursor.read_u8());
        let size = try!(cursor.read_u16::<BigEndian>());
        let flags = try!(cursor.read_u8());
        let server_count = try!(cursor.read_u16::<BigEndian>());
        let servers: Vec<ServerEntry> = try!((0..server_count).map(|i| {
            let start = (6 + (i as u64 * ServerEntry::size())) as usize;
            let end = start + ServerEntry::size() as usize;
            ServerEntry::parse(&data[start..end])
        }).collect());

        Ok(ServerList {
            size: size,
            flags: flags,
            servers: servers
        })
    }

}

impl Packet for ServerList {
    fn packet_id() -> u8 { 0xa8 }

    fn bytes(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; 62];
        let mut cursor = Cursor::new(buffer);

        cursor.write(&[Self::packet_id()]);
        // cursor.write(&self.username.as_bytes());
        // cursor.set_position(31);
        // cursor.write(&self.password.as_bytes());
        // cursor.set_position(61);
        // cursor.write(&[0x00]);

        cursor.into_inner()
    }
}

pub struct ServerEntry {
    pub index: u16,
    pub name: String,
    pub percent_full: u8,
    pub timezone: u8,
    pub address: Ipv4Addr
}

impl ServerEntry {
    fn size() -> u64 { 40 }

    fn parse(data: &[u8]) -> Result<ServerEntry, Error> {
        let mut cursor = Cursor::new(data);
        let index = try!(cursor.read_u16::<BigEndian>());
        let name = try!(String::from_utf8(data[2..34].to_vec()));
        cursor.consume(32);
        let percent_full = try!(cursor.read_u8());
        let timezone = try!(cursor.read_u8());
        let address = Ipv4Addr::from(try!(cursor.read_u32::<LittleEndian>()));
        Ok(ServerEntry {
            index: index,
            name: name,
            percent_full: percent_full,
            timezone: timezone,
            address: address
        })
    }
}
