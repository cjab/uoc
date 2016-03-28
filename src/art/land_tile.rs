use std::io;
use byteorder::{ReadBytesExt, LittleEndian};

use art::Error;
use color::Color;


const WIDTH: usize = 44;
const HALF_WIDTH: usize = WIDTH / 2;



pub struct LandTile {
    pixels: Vec<Color>
}


impl LandTile {

    pub fn parse(buf: &[u8]) -> Result<Self, Error> {
        let mut cursor = io::Cursor::new(buf);

        let size       = WIDTH * WIDTH;
        let mut pixels = vec![Color::new(); size];

        let mut line_width = 2;
        let mut x          = HALF_WIDTH;

        for y in 0..HALF_WIDTH {
            x -= 1;
            for i in 0..line_width {
                let pos   = y * WIDTH + i;
                let pixel = try!(cursor.read_u16::<LittleEndian>().map(Color::parse));
                pixels[pos + x] = try!(pixel);
            }
            line_width += 2;
        }

        for y in HALF_WIDTH..WIDTH {
            line_width -= 2;
            for i in 0..line_width {
                let pos   = y * WIDTH + i;
                let pixel = try!(cursor.read_u16::<LittleEndian>().map(Color::parse));
                pixels[pos + x] = try!(pixel);
            }
            x += 1;
        }

        Ok(LandTile { pixels: pixels })
    }


    pub fn width(&self) -> usize {
        WIDTH
    }


    pub fn height(&self) -> usize {
        WIDTH
    }


    pub fn as_rgb(&self) -> Vec<u8> {
        self.pixels.iter().flat_map(Color::as_rgb).collect()
    }
}
