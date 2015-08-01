use index::{Index};
use byteorder::{ReadBytesExt, LittleEndian};
use color::{Color};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{self, Read, Seek, SeekFrom, Cursor};
use std::mem;
use std::f32;
use std::u32;


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    UndefinedIndex,
    IncompleteTile,
    InvalidPath
}


impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}


pub struct Texture {
    pub pixels: Vec<Color>
}


impl Texture {

    pub fn parse(buf: &[u8]) -> Result<Texture, io::Error> {
        let pixel_count = (buf.len() / 2);
        let mut cursor  = Cursor::new(buf);

        let pixels = try!((0..pixel_count).map(|_| {
            try!(cursor.read_u16::<LittleEndian>().map(Color::parse))
        }).collect());

        Ok(Texture { pixels: pixels })
    }

    pub fn as_rgb(&self) -> Vec<u8> {
        self.pixels.iter().fold(Vec::new(), |mut acc, pixel| {
            acc.extend(pixel.as_rgb());
            acc
        })
    }

    pub fn width(&self) -> u64 {
        (self.pixels.len() as f32).sqrt() as u64
    }

    pub fn height(&self) -> u64 {
        (self.pixels.len() as f32).sqrt() as u64
    }
}


pub struct TextureReader<'a> {
    path:  &'a str,
    index: Index
}


impl<'a> TextureReader<'a> {

    pub fn new(path: &str) -> Result<TextureReader, Error> {
        let base_path  = Path::new(path);
        let index_path = base_path.join("texidx.mul");

        Ok(TextureReader {
            path:  try!(base_path.to_str().ok_or(Error::InvalidPath)),
            index: try!(Index::new(try!(index_path.to_str().ok_or(Error::InvalidPath))))
        })
    }

    pub fn get(&self, i: usize) -> Result<Texture, Error> {
        let entry     = &self.index.entries[i];
        let data_path = Path::new(self.path).join("texmaps.mul");
        let mut file  = try!(File::open(data_path));

        if entry.lookup >= (u32::MAX - 1) as u64 {
            return Err(Error::UndefinedIndex)
        }

        try!(file.seek(SeekFrom::Start(entry.lookup)));
        let buf: Vec<u8> = try!(file.take(entry.length).bytes().collect());
        Ok(try!(Texture::parse(&buf[..])))
    }
}
