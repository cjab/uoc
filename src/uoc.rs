extern crate byteorder;

//mod tile_data;
mod index;

//use tile_data::{TileData};
use index::{Index};

fn main() {
    //let mut tile_data = TileData::new("data/tiledata.mul");

    //let tiles = match tile_data.land_tiles() {
    //    Ok(tiles) => tiles,
    //    Err(e) => panic!("Failed to read land tiles: {}", e)
    //};

    //for tile in tiles {
    //    println!("========");
//  //      println!("Flags: {}", tile.flags);
//  //      println!("Texture Id: {}", tile.texture_id);
//  //      println!("Weight: {}", tile.weight);
    //    println!("Name: {}", tile.name);
    //}

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
    println!("DONE");
}


#[test]
fn it_works() {
}
