use std::io::{self, Read, Seek, SeekFrom, Cursor};
use byteorder::{self, ReadBytesExt, LittleEndian};

use art::{Art, Error};
use color::Color;



pub struct StaticTile {
    header: u32,
    width:  u16,
    height: u16,
    lookup: Vec<u16>,
    data:   Vec<OffsetRunData>
}


#[derive(Debug)]
struct OffsetRunData {
    offset: u16,
    run:    u16,
    pixels: Vec<Color>
}


impl OffsetRunData {

    pub fn parse(buf: &[u8]) -> Result<OffsetRunData, Error> {
        let mut cursor = io::Cursor::new(buf);
        let offset = try!(cursor.read_u16::<LittleEndian>());
        let run    = try!(cursor.read_u16::<LittleEndian>());
        let pixels = try!((1..2).map(|_| {
            Color::parse(try!(cursor.read_u16::<LittleEndian>()))
        }).collect());

        Ok(OffsetRunData {
            offset: offset,
            run:    run,
            pixels: pixels
        })
    }

    pub fn as_rgb(&self) -> Vec<u8> {
        (0..self.run).fold(Vec::new(), |mut run, i| {
            let pixel = &self.pixels[i as usize];
            run.extend(pixel.as_rgb().iter().map(|&p| p));
            run
        })
    }

    pub fn pixel_offset(&self) -> usize {
        self.offset as usize
    }

    pub fn byte_offset(&self) -> usize {
        (self.offset * 2) as usize
    }

    pub fn pixel_run(&self) -> usize {
        self.run as usize
    }

    pub fn byte_run(&self) -> usize {
        (self.run * 2) as usize
    }
}


impl StaticTile {

    pub fn parse(buf: &[u8]) -> Result<Self, Error> {
        let mut cursor  = io::Cursor::new(buf);

        let header = try!(cursor.read_u32::<LittleEndian>());
        let width  = try!(cursor.read_u16::<LittleEndian>());
        let height = try!(cursor.read_u16::<LittleEndian>());
        let lookup: Vec<u16> = try!((0..height).map(|_| {
            cursor.read_u16::<LittleEndian>()
        }).collect());

        let data_start = (8 as usize) + (height * 2) as usize;
        let data = try!((0..(height as usize)).map(|i: usize| -> Result<OffsetRunData, Error> {
            let offset = ((lookup[i] * 2) as usize);
            OffsetRunData::parse(&buf[offset..])
        }).collect());

        Ok(StaticTile {
            header: header,
            width:  width,
            height: height,
            lookup: lookup,
            data:   data
        })
    }

    pub fn width(&self) -> usize {
        self.width as usize
    }

    pub fn height(&self) -> usize {
        self.height as usize
    }

    pub fn as_rgb(&self) -> Vec<u8> {
        self.data.iter().fold(Vec::new(), |mut row: Vec<u8>, run: &OffsetRunData| {
            let run_pixels = run.as_rgb();
            println!("SIZE: {} -- {}", self.width, run_pixels.len() / 10);

            let left_padding = (0..run.pixel_offset()).fold(Vec::new(), |mut pad, _| {
                pad.extend(Color::new().as_rgb().iter().map(|&p| p));
                pad
            });

            println!("DATA: {:?}", self.data);
            println!("RunOffset: {:?}", run);
            println!("Width: {} | Offset: {} | Run: {}", self.width, run.offset, run.run);
            let right_padding_pixels = self.width - run.offset - run.run;
            let right_padding = (0..right_padding_pixels).fold(Vec::new(), |mut pad: Vec<u8>, _| {
                pad.extend(Color::new().as_rgb().iter().map(|p| *p));
                pad
            });

            row.extend(left_padding.iter().map(|&p| p));
            row.extend(run_pixels.iter().map(|&p| p));
            row.extend(right_padding.iter().map(|&p| p));
            row
        })
    }
}
