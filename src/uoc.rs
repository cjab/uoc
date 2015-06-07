extern crate byteorder;

mod tile_data;

use tile_data::{TileData};

fn main() {
    let mut tile_data = TileData::new("data/tiledata.mul");

    let tiles = match tile_data.land_tiles() {
        Ok(tiles) => tiles,
        Err(e) => panic!("Failed to read land tiles: {}", e)
    };

    for tile in tiles {
        println!("========");
//        println!("Flags: {}", tile.flags);
//        println!("Texture Id: {}", tile.texture_id);
//        println!("Weight: {}", tile.weight);
        println!("Name: {}", tile.name);
    }
}


#[test]
fn it_works() {
}
