use byteorder::{ReadBytesExt, LittleEndian};
use std::io::{self};

use anim::Error;
use color::Color;


const PALETTE_COLOR_COUNT: usize = 0x100;
const PALETTE_SIZE: usize = PALETTE_COLOR_COUNT * 2;



pub struct Animation {
    frames:  Vec<Frame>
}


impl Animation {

    pub fn parse(buf: &[u8]) -> Result<Self, Error> {
        let mut cursor = io::Cursor::new(buf);

        let palette: Vec<_> = try!((0..PALETTE_COLOR_COUNT).map(|_| {
            try!(cursor.read_u16::<LittleEndian>().map(Color::parse))
        }).collect());
        let frame_count = try!(cursor.read_i32::<LittleEndian>());
        let lookup: Vec<_> = try!((0..frame_count).map(|_| {
            cursor.read_i32::<LittleEndian>()
        }).collect());
        let frames: Vec<_> = try!(lookup.iter().map(|&offset| {
            let frame_start = (PALETTE_SIZE + offset as usize) as usize;
            println!("FRAME START: {}", frame_start);
            Frame::parse(&buf[frame_start..], &palette[..])
        }).collect());

        Ok(Animation { frames:  frames })
    }


    pub fn get_frame(&self, index: usize) -> Option<&Frame> {
        self.frames.get(index)
    }
}



pub struct Frame {
    center_x: i16,
    center_y: i16,
    width: i16,
    height: i16,
    pixels: Vec<Color>
}


impl Frame {

    pub fn parse(buf: &[u8], palette: &[Color]) -> Result<Self, Error> {
        let mut cursor = io::Cursor::new(buf);

        let center_x = try!(cursor.read_i16::<LittleEndian>());
        let center_y = try!(cursor.read_i16::<LittleEndian>());
        let width    = try!(cursor.read_i16::<LittleEndian>());
        let height   = try!(cursor.read_i16::<LittleEndian>());

        let size       = (width * height) as usize;
        let mut pixels = vec![Color::new(); size];

        println!("Size: {}x{} = {} | Center: ({}, {})", width, height, size, center_x, center_y);

        loop {
            let header = try!(cursor.read_i32::<LittleEndian>());
            if header == 0x7fff7fff { break; }

            let x_offset = ((((header >> 12) & 0x3ff) ^ 0x200) - 0x200) as i16;
            let y_offset = ((((header >> 22) & 0x3ff) ^ 0x200) - 0x200) as i16;
            //let x_offset = ((header >> 22) & 0x3ff) as i16;
            //let y_offset = ((header >> 12) & 0x3ff) as i16;
            let x_run    = 0x00000fff & header;

            let run_pixels: Vec<Color> = try!((0..x_run).map(|_| {
                cursor.read_u8().map(|c| palette[c as usize].clone())
            }).collect());

            let x     = center_x + x_offset;
            let y     = center_y + y_offset;
            let start = (x + (y * width)) as usize;

            println!("Offset: ({:4}, {:4}) | Start: ({:4}, {:4}) | Run Count: {:4}", x_offset, y_offset, x, y, x_run);

            for (i, pixel) in (0..run_pixels.len()).zip(run_pixels) {
                pixels[start + i] = pixel;
            }

        }

        Ok(Frame {
            center_x: center_x,
            center_y: center_y,
            width:    width,
            height:   height,
            pixels:   pixels
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
