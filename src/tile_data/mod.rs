mod land_tile;
mod static_tile;

use std::io::{self, Read, BufRead};
use std::fs::File;
pub use self::land_tile::LandTile;
pub use self::static_tile::StaticTile;

const BLOCK_HEADER_SIZE:     usize =  4;
const TILES_IN_BLOCK:        usize = 32;
const NUMBER_OF_LAND_BLOCKS: usize = 512;

pub struct TileData {
    file_path: String
}

impl TileData {

    pub fn new(path: &str) -> TileData {
        TileData { file_path: path.to_string() }
    }


    pub fn land_tiles(&self) -> Result<Vec<LandTile>, io::Error> {
        let mut file = try!(File::open(&self.file_path));
        let mut buf: Vec<u8> = Vec::new();
        try!(file.read_to_end(&mut buf));
        Ok(try!(parse_blocks(&buf[..], NUMBER_OF_LAND_BLOCKS)))
    }


    pub fn static_tiles(&self) -> Result<Vec<StaticTile>, io::Error> {
        let mut file = try!(File::open(&self.file_path));
        let mut buf: Vec<u8> = Vec::new();
        try!(file.read_to_end(&mut buf));
        let begin = LandTile::block_size() * NUMBER_OF_LAND_BLOCKS;
        print!("BEGINNING: {}", begin);
        Ok(try!(parse_blocks(&buf[begin..], NUMBER_OF_LAND_BLOCKS)))
    }
}

trait CanParse {
    fn size() -> usize;
    fn block_size() -> usize;
    fn parse(buf: &[u8]) -> Result<Self, io::Error>;
}


fn parse_tile_block<T: CanParse>(buf: &[u8]) -> Result<Vec<T>, io::Error> {
    let mut cursor = io::Cursor::new(buf);
    cursor.consume(BLOCK_HEADER_SIZE as usize); // Unknown block header

    let mut tile_data = Vec::new();
    try!(cursor.take((T::size() * TILES_IN_BLOCK) as u64).read_to_end(&mut tile_data));
    let tiles = try!(tile_data.chunks(T::size()).map(T::parse).collect());
    Ok(tiles)
}


fn parse_blocks<T: CanParse>(buf: &[u8], count: usize) -> Result<Vec<T>, io::Error> {
    let blocks: Vec<&[u8]> = buf.chunks(T::block_size()).take(count).collect();


    Ok(blocks.iter().flat_map(|block| {
        match parse_tile_block(&block[..]) {
            Ok(tile) => tile,
            Err(e)   => panic!("Failed to parse tile block: {}", e)
        }
    }).collect())
}
