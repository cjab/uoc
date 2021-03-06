mod land_tile;
mod static_tile;

use std::fs::File;
use std::path::Path;
use std::io::{self, Read, Seek, SeekFrom};

use index::Index;
use self::land_tile::LandTile;
use self::static_tile::StaticTile;

const STATIC_TILE_INDEX_OFFSET: usize = 0x4000;


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    UndefinedIndex,
    IncompleteTile,
    InvalidPath
}


impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}


pub struct ArtData {
    file:  File,
    index: Index
}


impl ArtData {

    pub fn new(path: &str) -> Result<ArtData, Error> {
        let base_path  = Path::new(path);
        let data_path  = base_path.join("art.mul");
        let index_path = base_path.join("artidx.mul");

        Ok(ArtData {
            file:  try!(File::open(data_path)),
            index: try!(Index::new(try!(index_path.to_str().ok_or(Error::InvalidPath))))
        })
    }

    pub fn get_land(&self, i: usize) -> Result<LandTile, Error> {
        let entry     = &self.index.get(i);
        let mut file  = &self.file;

        if entry.lookup_undefined() {
            return Err(Error::UndefinedIndex)
        }

        try!(file.seek(SeekFrom::Start(entry.lookup as u64)));
        let buf: Vec<u8> = try!(file.take(entry.length as u64).bytes().collect());
        Ok(try!(LandTile::parse(&buf[..])))
    }

    pub fn get_static(&self, i: usize) -> Result<StaticTile, Error> {
        let entry    = &self.index.get(STATIC_TILE_INDEX_OFFSET + i);
        let mut file = &self.file;

        if entry.lookup_undefined() {
            return Err(Error::UndefinedIndex)
        }

        try!(file.seek(SeekFrom::Start(entry.lookup as u64)));
        let buf: Vec<u8> = try!(file.take(entry.length as u64).bytes().collect());
        Ok(try!(StaticTile::parse(&buf[..])))
    }
}
