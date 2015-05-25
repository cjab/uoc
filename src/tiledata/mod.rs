//extern crate byteorder;

use std::io::{self, Read, BufRead};
use byteorder::{ ReadBytesExt, LittleEndian };

const BLOCK_HEADER_SIZE: u64 =  4;
const LAND_TILE_SIZE:    u64 = 26;
const TILES_IN_BLOCK:    u64 = 32;
const NUMBER_OF_BLOCKS:  u64 = 16;
const NAME_LENGTH:       u64 = 20;
const LAND_TILE_BLOCK_SIZE: u64 = (TILES_IN_BLOCK * LAND_TILE_SIZE) + BLOCK_HEADER_SIZE;


pub struct LandTile {
    pub flags:      i32,
    pub texture_id: i16,
    pub name:       String,
}


impl LandTile {

    fn new() -> LandTile {
        LandTile {
            flags:      0i32,
            texture_id: 0i16,
            name:       String::new()
        }
    }
}

pub trait CanParse {
    fn size() -> usize;
    fn parse(buf: &[u8]) -> Result<Self, io::Error>;
}

impl CanParse for LandTile {

    fn parse(buf: &[u8]) -> Result<LandTile, io::Error> {
        let mut cursor = io::Cursor::new(buf);
        let mut tile   = Self::new();

        tile.flags      = try!(cursor.read_i32::<LittleEndian>());
        tile.texture_id = try!(cursor.read_i16::<LittleEndian>());
        match cursor.take(NAME_LENGTH).read_to_string(&mut tile.name) {
            Ok(count) => count,
            Err(e)    => panic!("Could not read land tile name: {}", e)
        };
        Ok(tile)
    }

    fn size() -> usize { 26 }
}

fn parse_tile_block<T: CanParse>(buf: &[u8]) -> Result<Vec<T>, io::Error> {
    let mut cursor = io::Cursor::new(buf);
    cursor.consume(BLOCK_HEADER_SIZE as usize); // Unknown group header

    let mut tile_data = Vec::new();
    try!(cursor.take((T::size() * TILES_IN_BLOCK as usize) as u64).read_to_end(&mut tile_data));
    let tiles = tile_data.chunks(T::size());

    Ok(tiles.map(|t| {
        match T::parse(t) {
            Ok(tile) => tile,
            Err(e)   => panic!("Failed to parse tile: {}", e)
        }
    }).collect())
}

pub fn parse_blocks<T: CanParse>(buf: &[u8], count: usize) -> Result<Vec<T>, io::Error> {
    let blocks = buf.chunks(T::size()).take(count);

    Ok(blocks.flat_map(|block| {
        match parse_tile_block(&block[..]) {
            Ok(tile) => tile,
            Err(e)   => panic!("Failed to parse tile block: {}", e)
        }
    }).collect())
}


pub struct StaticTile {
    pub flags:     i32,
    pub weight:     u8,
    pub quality:    u8,
        unknown1:  i16,
        unknown2:   u8,
    pub quantity:   u8,
    pub animation: i16,
        unknown3:   u8,
    pub hue:        u8,
        unknown4:   u8,
        unknown5:   u8,
    pub height:     u8,
    pub name:   String
}
