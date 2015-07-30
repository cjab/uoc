use byteorder::{ReadBytesExt, LittleEndian};
use std::io::{self, Read};
use std::mem::{size_of};
use std::fs::File;
use std::path::{Path};

pub struct IndexEntry {
    pub lookup: u64,
    pub length: u64,
    pub extra:  i32
}

pub struct Index {
    pub entries: Vec<IndexEntry>
}

impl IndexEntry {
    fn parse(buf: &[u8]) -> Result<IndexEntry, io::Error> {
        let mut cursor = io::Cursor::new(buf);
        Ok(IndexEntry {
            lookup: try!(cursor.read_i32::<LittleEndian>()) as u64,
            length: try!(cursor.read_i32::<LittleEndian>()) as u64,
            extra:  try!(cursor.read_i32::<LittleEndian>())
        })
    }
}

impl Index {

    pub fn new(path: &str) -> Result<Index, io::Error> {
        let mut file = try!(File::open(path));
        let mut buf: Vec<u8> = Vec::new();
        try!(file.read_to_end(&mut buf));
        Ok(try!(Self::parse(&buf[..])))
    }

    fn parse(buf: &[u8]) -> Result<Index, io::Error> {
        let entries_buf = buf.chunks(size_of::<IndexEntry>())
                             .filter(|&e| e.len() == size_of::<IndexEntry>());
        let entries = try!(entries_buf.map(IndexEntry::parse).collect());
        Ok(Index { entries: entries })
    }
}
