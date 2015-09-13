use std::i32;
use std::fs::File;
use std::mem::size_of;
use std::io::{self, Read};

use byteorder::{ReadBytesExt, LittleEndian};



#[derive(Debug)]
pub struct IndexEntry {
    pub lookup: i32,
    pub length: i32,
    pub extra:  i32
}


impl IndexEntry {
    fn parse(buf: &[u8]) -> Result<IndexEntry, io::Error> {
        let mut cursor = io::Cursor::new(buf);
        Ok(IndexEntry {
            lookup: try!(cursor.read_i32::<LittleEndian>()),
            length: try!(cursor.read_i32::<LittleEndian>()),
            extra:  try!(cursor.read_i32::<LittleEndian>())
        })
    }

    pub fn lookup_undefined(&self) -> bool {
        self.lookup >= (i32::MAX - 1) as i32
    }
}



pub struct Index {
    entries: Vec<IndexEntry>
}


impl Index {

    pub fn new(path: &str) -> Result<Index, io::Error> {
        let mut file = try!(File::open(path));
        let mut buf: Vec<u8> = Vec::new();
        try!(file.read_to_end(&mut buf));
        Ok(try!(Self::parse(&buf[..])))
    }

    pub fn get(&self, i: usize) -> &IndexEntry {
        &self.entries[i]
    }

    fn parse(buf: &[u8]) -> Result<Index, io::Error> {
        let entries = try!(buf.chunks(size_of::<IndexEntry>())
                              .filter(|&e| e.len() == size_of::<IndexEntry>())
                              .map(IndexEntry::parse).collect());
        Ok(Index { entries: entries })
    }
}
