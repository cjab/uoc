mod animation;

use self::animation::Animation;

use std::fs::File;
use std::path::Path;
use std::io::{self, Read, Seek, SeekFrom};

use index::Index;


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    UndefinedIndex,
    InvalidPath
}


impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}



pub struct AnimationFile {
    file:  File,
    index: Index
}


impl AnimationFile {

    pub fn new(path: &str) -> Result<AnimationFile, Error> {
        let base_path  = Path::new(path);
        let data_path  = base_path.join("anim.mul");
        let index_path = base_path.join("anim.idx");

        Ok(AnimationFile {
            file:  try!(File::open(data_path)),
            index: try!(Index::new(try!(index_path.to_str().ok_or(Error::InvalidPath))))
        })
    }


    pub fn get_animation(&self, i: usize) -> Result<Animation, Error> {
        let entry    = &self.index.get(i);
        let mut file = &self.file;

        if entry.lookup_undefined() {
            return Err(Error::UndefinedIndex)
        }
        println!("HERE {}", entry.lookup);

        try!(file.seek(SeekFrom::Start(entry.lookup as u64)));
        let buf: Vec<u8> = try!(file.take(entry.length as u64).bytes().collect());

        Ok(try!(Animation::parse(&buf)))
    }
}
