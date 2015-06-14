extern crate byteorder;

mod tile_data;
mod index;

use tile_data::{TileData};
use index::{Index};

fn main() {
    let tile_data = TileData::new("data/tiledata.mul");

    let land_tiles = match tile_data.land_tiles() {
        Ok(tiles) => tiles,
        Err(e) => panic!("Failed to read land tiles: {}", e)
    };

    for tile in land_tiles {
        println!("========");
        println!("Name: {}", tile.name);
    }

    let static_tiles = match tile_data.static_tiles() {
        Ok(tiles) => tiles,
        Err(e) => panic!("Failed to read static tiles: {}", e)
    };

    for tile in static_tiles {
        println!("========");
        println!("Name: {}", tile.name);
    }

    let index = match Index::new("data/texidx.mul") {
        Ok(index) => index,
        Err(e)    => panic!("Failed to read index: {}", e)
    };
    for entry in index.entries {
        println!("============");
        println!("Lookup: {}", entry.lookup);
        println!("Length: {}", entry.length);
        println!("Extra:  {}", entry.extra);
    }
}


#[test]
fn it_works() {
}
