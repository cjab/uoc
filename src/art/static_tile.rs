use std::io::{self, Seek, SeekFrom};
use byteorder::{ReadBytesExt, LittleEndian};

use art::Error::{self, IncompleteTile};
use color::Color;



pub struct StaticTile {
    width:  u16,
    height: u16,
    pixels: Vec<Color>
}


impl StaticTile {

    pub fn parse(buf: &[u8]) -> Result<Self, Error> {
        let mut cursor  = io::Cursor::new(buf);

        try!(cursor.read_u32::<LittleEndian>()); // Unknown Header
        let width  = try!(cursor.read_u16::<LittleEndian>());
        let height = try!(cursor.read_u16::<LittleEndian>());

        if width <= 0 || height <= 0 { return Err(IncompleteTile) }

        let lookup: Vec<_> = try!((0..height).map(|_| {
            cursor.read_u16::<LittleEndian>()
        }).collect());

        let size       = (width * height) as usize;
        let mut pixels = vec![Color::new(); size];

        for y in (0..height as usize) {
            let start = ((lookup[y as usize] + 4 + height) * 2) as u64;
            try!(cursor.seek(SeekFrom::Start(start)));

            let mut x: usize = 0;
            loop {
                let offset = try!(cursor.read_u16::<LittleEndian>());
                let run    = try!(cursor.read_u16::<LittleEndian>());

                if offset + run == 0 { break; }
                x += offset as usize;

                for _ in 0..run {
                    let pixel = try!(Color::parse(try!(cursor.read_u16::<LittleEndian>())));
                    pixels[y * width as usize + x] = pixel;
                    x += 1;
                }
            }
        }

        Ok(StaticTile {
            width:  width,
            height: height,
            pixels: pixels
        })
    }


    pub fn width(&self) -> usize {
        self.width as usize
    }


    pub fn height(&self) -> usize {
        self.height as usize
    }


    pub fn as_rgb(&self) -> Vec<u8> {
        self.pixels.iter().flat_map(Color::as_rgb).collect()
    }
}
