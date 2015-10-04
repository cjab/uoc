mod land_tile;
mod static_tile;

use std::io::{self, Read, Seek, SeekFrom};
use std::fs::File;
pub use self::land_tile::LandTile;
pub use self::static_tile::StaticTile;

const BLOCK_HEADER_SIZE:     usize =   4;
const TILES_IN_BLOCK:        usize =  32;
const NUMBER_OF_LAND_BLOCKS: usize = 512;


pub struct TileData {
    land_tiles:   Vec<LandTile>,
    static_tiles: Vec<StaticTile>
}


impl TileData {

    pub fn new(path: &str) -> Result<TileData, io::Error> {
        Ok(TileData {
            land_tiles:   try!(land_tiles(path)),
            static_tiles: try!(static_tiles(path))
        })
    }


    pub fn get_land_tile(&self, index: usize) -> Option<&LandTile> {
        self.land_tiles.get(index)
    }


    pub fn get_static_tile(&self, index: usize) -> Option<&StaticTile> {
        self.static_tiles.get(index)
    }
}


trait Tile {
    fn size() -> usize;
    fn block_size() -> usize;
    fn parse(buf: &[u8]) -> Result<Self, io::Error>;
}


fn land_tiles(file_path: &str) -> Result<Vec<LandTile>, io::Error> {
    let mut file = try!(File::open(file_path));

    let mut buf: Vec<u8> = vec![0u8; static_tiles_offset() as usize];
    try!(file.read(&mut buf));

    Ok(try!(parse_blocks(&buf, NUMBER_OF_LAND_BLOCKS)))
}


fn static_tiles(file_path: &str) -> Result<Vec<StaticTile>, io::Error> {
    let mut file = try!(File::open(file_path));
    try!(file.seek(SeekFrom::Start(static_tiles_offset())));

    let mut buf: Vec<u8> = Vec::new();
    try!(file.read_to_end(&mut buf));

    //FIXME: This should not be NUMBER_OF_LAND_BLOCKS
    Ok(try!(parse_blocks(&buf, NUMBER_OF_LAND_BLOCKS)))
}


fn static_tiles_offset() -> u64 {
    (LandTile::block_size() * NUMBER_OF_LAND_BLOCKS) as u64
}


fn parse_tile_block<T: Tile>(buf: &[u8]) -> Result<Vec<T>, io::Error> {
    Ok(try!(
        buf[BLOCK_HEADER_SIZE..].chunks(T::size())
                                .take(TILES_IN_BLOCK)
                                .map(T::parse).collect()
    ))
}


fn parse_blocks<T: Tile>(buf: &[u8], count: usize) -> Result<Vec<T>, io::Error> {
    let blocks: Vec<Vec<T>> = try!(
        buf.chunks(T::block_size()).take(count)
           .map(parse_tile_block).collect()
    );
    Ok(blocks.into_iter().flat_map(|b| b).collect())
}
