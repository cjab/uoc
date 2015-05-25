extern crate byteorder;

mod tiledata;

use std::fs::File;
use std::io::{Read};

use tiledata::{LandTile, parse_blocks};

fn main() {
    let mut buf: Vec<u8> = Vec::new();

    let mut file = match File::open("data/tiledata.mul") {
        Err(e)   => panic!("Failed to open tiledata.mul: {}", e),
        Ok(file) => file
    };

    match file.read_to_end(&mut buf) {
        Err(e)   => panic!("Failed to read tiledata.mul to buffer: {}", e),
        Ok(size) => size
    };

    let tiles: Vec<LandTile> = match parse_blocks(&buf[..], 16) {
        Err(e)    => panic!("Failed to parse land tile groups: {}", e),
        Ok(tiles) => tiles
    };

    for tile in tiles {
        println!("========");
        //println!("Tile {}", i);
        println!("-------");
        println!("Flags: {}", tile.flags);
        println!("Texture Id: {}", tile.texture_id);
        println!("Name: {}", tile.name);
    }
}


#[test]
fn it_works() {
}
