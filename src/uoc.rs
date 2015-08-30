extern crate byteorder;
extern crate argparse;
extern crate sdl2;

//mod tile_data;
mod index;
mod art;
mod texture;
mod color;

//use tile_data::TileData;
//use index::Index;

use texture::TextureData;
use art::ArtData;

use argparse::{ArgumentParser, StoreTrue, Store};

use sdl2::event::Event;
use sdl2::render::Texture;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;

use std::env;

struct Options {
    asset_type: String,
    index: usize,
}


fn get_land(index: usize) -> (Vec<u8>, u32, u32) {
    let art_data = match ArtData::new("data/") {
        Ok(art)  => art,
        Err(err) => panic!("Error: {:?}", err)
    };

    let land_tile = match art_data.get_land(index) {
        Ok(tile) => tile,
        Err(err) => panic!("Error: {:?}", err)
    };

    let mut tile_data = land_tile.as_rgb();
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

    let mut tile_data = static_tile.as_rgb();
    let width  = static_tile.width() as u32;
    let height = static_tile.height() as u32;
    (tile_data, width, height)
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

    let mut tile_data = tile.as_rgb();
    let width  = tile.width() as u32;
    let height = tile.width() as u32;
    (tile_data, width, height)
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

    let (mut asset_data, width, height) = match(options.asset_type.as_ref()) {
        "land"    => get_land(options.index),
        "texture" => get_texture(options.index),
        "static"  => get_static(options.index),
        _         => panic!("Unknown asset type!")
    };

    let mut ctx = sdl2::init().everything().unwrap();

    let window = match ctx.window("UOC", width * 5, height * 5).position_centered().opengl().build() {
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
        Err(err)    => panic!("Failed to convert surface: {}", err)
    };

    let mut drawer = renderer.drawer();
    let _ = drawer.clear();
    let _ = drawer.copy(&texture, None, None);
    let _ = drawer.present();

    let mut events = ctx.event_pump();

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
