extern crate byteorder;
extern crate argparse;
extern crate sdl2;

mod tile_data;
mod index;
mod art;
mod anim;
mod texture;
mod color;

use tile_data::TileData;

use texture::TextureData;
use art::ArtData;
use anim::AnimationFile;

use argparse::{ArgumentParser, Store};

use sdl2::event::Event;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;

struct Options {
    asset_type: String,
    index: usize,
}


fn get_land(index: usize) -> (Vec<u8>, u32, u32) {
    let art_data = match ArtData::new("data/") {
        Ok(art)  => art,
        Err(err) => panic!("Error: {:?}", err)
    };

    let tile_data = match TileData::new("data/tiledata.mul") {
        Ok(data) => data,
        Err(err) => panic!("Error: {:?}", err)
    };

    let data = tile_data.get_land_tile(index).unwrap();
    println!("DATA: {:?}", data);

    let land_tile = match art_data.get_land(index) {
        Ok(tile) => tile,
        Err(err) => panic!("Error: {:?}", err)
    };

    let tile_data = land_tile.as_rgb();
    let width  = land_tile.width() as u32;
    let height = land_tile.height() as u32;
    (tile_data, width, height)
}


fn get_static(index: usize) -> (Vec<u8>, u32, u32) {
    let art_data = match ArtData::new("data/") {
        Ok(art)  => art,
        Err(err) => panic!("Error: {:?}", err)
    };

    let static_tile = match art_data.get_static(index) {
        Ok(tile) => tile,
        Err(err) => panic!("Error: {:?}", err)
    };

    let tile_data = match TileData::new("data/tiledata.mul") {
        Ok(data) => data,
        Err(err) => panic!("Error: {:?}", err)
    };

    let data = tile_data.get_land_tile(index).unwrap();
    println!("DATA: {:?}", data);

    let width  = static_tile.width() as u32;
    let height = static_tile.height() as u32;
    (static_tile.as_rgb(), width, height)
}


fn get_texture(index: usize) -> (Vec<u8>, u32, u32) {
    let texture_data = match TextureData::new("data/") {
        Ok(r) => r,
        Err(err) => panic!("{:?}", err)
    };

    let tile = match texture_data.get(index) {
        Ok(tile) => tile,
        Err(err) => panic!("Error: {:?}", err)
    };

    let tile_data = tile.as_rgb();
    let width  = tile.width() as u32;
    let height = tile.width() as u32;
    (tile_data, width, height)
}


fn get_animation(index: usize) -> (Vec<u8>, u32, u32) {
    let animation_file = match AnimationFile::new("data/") {
        Ok(file) => file,
        Err(err) => panic!("Error: {:?}", err)
    };

    println!("INDEX: {}", index);

    let animation = match animation_file.get_animation(index) {
        Ok(anim) => anim,
        Err(err) => panic!("Error: {:?}", err)
    };

    let frame = animation.get_frame(0).unwrap();
    let width  = frame.width() as u32;
    let height = frame.height() as u32;
    (frame.as_rgb(), width, height)
}



fn main() {
    let mut options = Options { asset_type: String::new(), index: 0 as usize };

    {
        let mut parser = ArgumentParser::new();
        parser.refer(&mut options.asset_type)
              .add_argument("asset type", Store, "Type of asset");
        parser.refer(&mut options.index)
              .add_argument("id", Store, "The id of the asset");
        parser.parse_args_or_exit();
    }

    let (mut asset_data, width, height) = match options.asset_type.as_ref() {
        "land"      => get_land(options.index),
        "texture"   => get_texture(options.index),
        "static"    => get_static(options.index),
        "animation" => get_animation(options.index),
        _           => panic!("Unknown asset type!")
    };

    let mut ctx = sdl2::init().unwrap();
    let mut video = ctx.video().unwrap();

    let window = match video.window("UOC", width * 5, height * 5).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("Failed to created window: {}", err)
    };

    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err)     => panic!("Failed to create renderer: {}", err)
    };

    let surface = match Surface::from_data(&mut asset_data[..], width, height, 3 * width, PixelFormatEnum::RGB24) {
        Ok(surface) => surface,
        Err(err)    => panic!("Failed to load surface: {}", err)
    };

    let texture = match renderer.create_texture_from_surface(&surface) {
        Ok(texture) => texture,
        Err(err)    => panic!("Failed to convert surface: {:?}", err)
    };

    let _ = renderer.clear();
    let _ = renderer.copy(&texture, None, None);
    let _ = renderer.present();

    let mut events = ctx.event_pump().unwrap();

    'event: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                _               => continue
            }
        }
    }
}


#[test]
fn it_works() {
}
