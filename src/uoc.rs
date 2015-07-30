extern crate byteorder;
extern crate sdl2;

mod tile_data;
mod index;
mod art;
mod color;

use std::path::Path;

use tile_data::{TileData};
use index::{Index};

use art::{Art};

use sdl2::event::{Event};
use sdl2::surface::{Surface};
use sdl2::pixels::{PixelFormatEnum};

use std::fs::File;
use std::io::{self, Read};
use std::env;


fn main() {

    let index = env::args().last().unwrap().parse::<usize>().unwrap();


    let art = match Art::new("data/") {
        Ok(art)  => art,
        Err(err) => panic!("Error: {:?}", err)
    };

    let tile = match art.get(index) {
        Ok(tile) => tile,
        Err(err) => panic!("Error: {:?}", err)
    };

    let mut ctx = sdl2::init().everything().unwrap();

    let window = match ctx.window("UOC", 640, 640).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("Failed to created window: {}", err)
    };

    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err)     => panic!("Failed to create renderer: {}", err)
    };

    let mut tile_data = tile.as_rgb();
    let surface = match Surface::from_data(&mut tile_data[..], 44, 44, 3 * 44, PixelFormatEnum::RGB24) {
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
