use std::io::{self, Read, Seek, SeekFrom};
use byteorder::{self, ReadBytesExt, LittleEndian};

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

        let size = WIDTH * WIDTH;
        let mut pixels = (0..size).fold(Vec::with_capacity(size), |mut pixels: Vec<_>, _| {
            pixels.push(Color::new());
            pixels
        });

        let mut line_width = 2;
        let mut x = HALF_WIDTH;

        for y in (0..HALF_WIDTH) {
            x -= 1;
            for i in (0..line_width) {
                let pos   = y * WIDTH + i;
                let pixel = try!(cursor.read_u16::<LittleEndian>().map(Color::parse));
                pixels[pos + x] = try!(pixel);
            }
            line_width += 2;
        }

        for y in (HALF_WIDTH..WIDTH) {
            line_width -= 2;
            for i in (0..line_width) {
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
        self.pixels.iter().fold(Vec::with_capacity(WIDTH * WIDTH), |mut data: Vec<u8>, pixel: &Color| {
            data.extend(pixel.as_rgb().iter());
            data
        })
    }
}
