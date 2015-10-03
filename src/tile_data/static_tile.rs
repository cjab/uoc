use std::io::{self, Read};
use byteorder::{ReadBytesExt, LittleEndian};
use tile_data::Tile;
use tile_data::BLOCK_HEADER_SIZE;
use tile_data::TILES_IN_BLOCK;

const NAME_LENGTH: u64 = 20;

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


impl StaticTile {
    pub fn new() -> StaticTile {
        StaticTile {
            flags:     0i32,
            weight:    0u8,
            quality:   0u8,
            unknown1:  0i16,
            unknown2:  0u8,
            quantity:  0u8,
            animation: 0i16,
            unknown3:  0u8,
            hue:       0u8,
            unknown4:  0u8,
            unknown5:  0u8,
            height:    0u8,
            name:      String::new()
        }
    }
}


impl Tile for StaticTile {

    fn parse(buf: &[u8]) -> Result<StaticTile, io::Error> {
        let mut cursor = io::Cursor::new(buf);
        let mut tile   = Self::new();

        tile.flags     = try!(cursor.read_i32::<LittleEndian>());
        tile.weight    = try!(cursor.read_u8());
        tile.quality   = try!(cursor.read_u8());
        tile.unknown1  = try!(cursor.read_i16::<LittleEndian>());
        tile.unknown2  = try!(cursor.read_u8());
        tile.quantity  = try!(cursor.read_u8());
        tile.animation = try!(cursor.read_i16::<LittleEndian>());
        tile.unknown3  = try!(cursor.read_u8());
        tile.hue       = try!(cursor.read_u8());
        tile.unknown4  = try!(cursor.read_u8());
        tile.unknown5  = try!(cursor.read_u8());
        tile.height    = try!(cursor.read_u8());
        try!(cursor.take(NAME_LENGTH).read_to_string(&mut tile.name));

        Ok(tile)
    }

    fn size() -> usize { 37 }

    fn block_size() -> usize {
        (Self::size() * TILES_IN_BLOCK) + BLOCK_HEADER_SIZE
    }
}
