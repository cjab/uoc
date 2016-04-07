use std::io::{self, Read};
use byteorder::{ReadBytesExt, LittleEndian};
use tile_data::Tile;
use tile_data::BLOCK_HEADER_SIZE;
use tile_data::TILES_IN_BLOCK;

const NAME_LENGTH: u64 = 20;

#[derive(Debug)]
pub struct LandTile {
    pub flags:      i32,
    pub texture_id: i16,
    pub name:       String,
}


impl LandTile {
    pub fn new() -> LandTile {
        LandTile {
            flags:      0i32,
            texture_id: 0i16,
            name:       String::new()
        }
    }
}


impl Tile for LandTile {

    fn parse(buf: &[u8]) -> Result<Box<LandTile>, io::Error> {
        let mut cursor = io::Cursor::new(buf);
        let mut tile   = Self::new();

        tile.flags      = try!(cursor.read_i32::<LittleEndian>());
        tile.texture_id = try!(cursor.read_i16::<LittleEndian>());
        match cursor.take(NAME_LENGTH).read_to_string(&mut tile.name) {
            Ok(_) => (),
            Err(_) => ()
        }

        Ok(Box::new(tile))
    }


    fn size() -> usize { 26 }


    fn block_size() -> usize {
        (Self::size() * TILES_IN_BLOCK) + BLOCK_HEADER_SIZE
    }
}
