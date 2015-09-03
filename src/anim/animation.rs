use byteorder::{ReadBytesExt, LittleEndian};
use std::io::{self, Read, Seek, SeekFrom};

use anim::Error;
use color::Color;


const PALETTE_SIZE: usize = 0x100;



pub struct Animation {
    palette: Vec<Color>,
    frame_count: i32,
    lookup: Vec<i32>,
    frames: Vec<Frame>
}


impl Animation {

    pub fn parse(buf: &[u8]) -> Result<Self, Error> {
        let mut cursor = io::Cursor::new(buf);

        let palette = try!((0..PALETTE_SIZE).map(|_| {
            try!(cursor.read_u16::<LittleEndian>().map(Color::parse))
        }).collect());
        let frame_count = try!(cursor.read_i32::<LittleEndian>());
        let lookup: Vec<_> = try!((0..frame_count).map(|_| {
            cursor.read_i32::<LittleEndian>()
        }).collect());
        let frames: Vec<_> = try!(lookup.iter().map(|offset| {
            let frame_start = (0x200 + offset) as usize;
            Frame::parse(&buf[frame_start..])
        }).collect());

        Ok(Animation {
            palette:     palette,
            frame_count: frame_count,
            lookup:      lookup,
            frames:      frames
        })
    }


    pub fn as_rgb(&self) -> Vec<u8> {
        self.frames.iter().flat_map(Frame::as_rgb).collect()
    }
}



pub struct Frame {
    center_x: u16,
    center_y: u16,
    width: u16,
    height: u16,
    pixels: Vec<Color>
}


impl Frame {

    pub fn parse(buf: &[u8]) -> Result<Self, Error> {
        let mut cursor = io::Cursor::new(buf);

        let center_x = try!(cursor.read_u16::<LittleEndian>());
        let center_y = try!(cursor.read_u16::<LittleEndian>());
        let width    = try!(cursor.read_u16::<LittleEndian>());
        let height   = try!(cursor.read_u16::<LittleEndian>());

        let size   = (width * height) as usize;
        let pixels = (0..size).map(|_| Color::new()).collect();

        loop {
            let header   = try!(cursor.read_i32::<LittleEndian>());
            let x_offset = (((0xffc00000 & header) >> 22) ^ 0x200) - 0x200;
            let y_offset = (((0x003ff000 & header) >> 12) ^ 0x200) - 0x200;
            let x_run    = 0x00000fff & header;

            println!("x: {}, y: {}, run: {}", x_offset, y_offset, x_run);

            if header == 0x7fff7fff { break; }
        }

        Ok(Frame {
            center_x: center_x,
            center_y: center_y,
            width:    width,
            height:   height,
            pixels:   pixels
        })
    }


    pub fn as_rgb(&self) -> Vec<u8> {
        self.pixels.iter().flat_map(Color::as_rgb).collect()
    }
}
