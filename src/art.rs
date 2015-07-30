use byteorder::{self, ReadBytesExt, LittleEndian};
use std::io::{self, Read, Seek, SeekFrom};
use color::{Color};
use index::{Index};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::u32;

const LAND_TILE_WIDTH: usize = 44;


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ByteOrder(byteorder::Error),
    UndefinedIndex,
    IncompleteTile,
    InvalidPath
}


impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}


impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        Error::ByteOrder(err)
    }
}


pub struct LandTile {
    pub pixels: Vec<Color>
}


pub struct Art<'a> {
    path:  &'a str,
    index: Index
}


impl<'a> Art<'a> {

    pub fn new(path: &str) -> Result<Art, Error> {
        let base_path  = Path::new(path);
        let index_path = base_path.join("artidx.mul");

        Ok(Art {
            path:  try!(base_path.to_str().ok_or(Error::InvalidPath)),
            index: try!(Index::new(try!(index_path.to_str().ok_or(Error::InvalidPath))))
        })
    }

    pub fn get(&self, i: usize) -> Result<LandTile, Error> {
        let entry     = &self.index.entries[i];
        let data_path = Path::new(self.path).join("art.mul");
        let mut file  = try!(File::open(data_path));

        println!("Lookup: {} -- Length: {}", entry.lookup, entry.length);

        if entry.lookup >= (u32::MAX - 1) as u64 {
            return Err(Error::UndefinedIndex)
        }

        try!(file.seek(SeekFrom::Start(entry.lookup)));
        let buf: Vec<u8> = try!(file.take(entry.length).bytes().collect());
        Ok(try!(LandTile::parse(&buf[..])))
    }
}


impl LandTile {

    pub fn parse(buf: &[u8]) -> Result<LandTile, Error> {
        let mut cursor = io::Cursor::new(buf);
        println!("HERE: {}, BYTE COUNT: {}", buf.len(), pixel_count());
        let pixels: Vec<Color> = try!((0..pixel_count()).map(|_| {
            try!(cursor.read_u16::<LittleEndian>().map(Color::parse))
        }).collect());

        if pixels.len() < pixel_count() {
            Err(Error::IncompleteTile)
        } else {
            Ok(LandTile { pixels: pixels })
        }
    }

    pub fn as_rgb(&self) -> Vec<u8> {
        let mut pixels = self.pixels.iter();
        let mut rgb_data = Vec::new();

        for row in 0..LAND_TILE_WIDTH {
            for col in 0..LAND_TILE_WIDTH {
                if is_padding(row, col) {
                    rgb_data.push(0x00u8);
                    rgb_data.push(0xffu8);
                    rgb_data.push(0x00u8);
                } else {
                    let bytes = pixels.next().unwrap().as_rgb();
                    for byte in bytes.iter() {
                        rgb_data.push(*byte);
                    }
                }
            }
        }
        rgb_data
    }
}


fn pixel_count() -> usize {
    (0..(LAND_TILE_WIDTH / 2)).fold(0, |acc, i| { acc + ((i+1) * 2) }) * 2
//    1024
}


fn padding(row: usize) -> usize {
    let half_width = LAND_TILE_WIDTH / 2;
    let is_top     = row < half_width;
    let is_bottom  = row >= half_width;

    let total_padding: i32 = if (is_top) {
        LAND_TILE_WIDTH as i32 - (2 * (row as i32 + 1))
    } else {
        LAND_TILE_WIDTH as i32 - (2 * (row as i32 + 1)) + 1
    };
    total_padding.abs() as usize / 2
}


fn row_size(row: usize) -> usize {
    LAND_TILE_WIDTH - (padding(row) * 2)
}


fn is_padding(row: usize, col: usize) -> bool {
    let padding  = padding(row);
    let row_size = row_size(row);
    col < padding || col >= (padding + row_size)
}
