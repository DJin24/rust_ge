extern crate sdl2;

use self::sdl2::event::Event;
use crate::rust_ge::rust_ge_base::AbstractGame;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use std::collections::HashSet;
use crate::rust_ge::sprites::Sprite;

pub struct Engine {
    game: AbstractGame
}

impl Engine {
    pub fn run(&self) {
        //TODO move initialization of window and everything to the engine
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        
        loop {
            let mut sprites = HashSet::<Sprite>::new(); // maybe &Sprite, though might be confusing with lifetimes
            self.game.draw(&mut sprites);
            let surfaces = sprites.iter().map(|sprite| sprite.as_sdl_surface());
            let texture_creator = canvas.texture_creator();
            for surface in surfaces {
                match texture_creator.create_texture_from_surface(surface){
                    Err(_) => panic!("failed to create texture on window"),
                    Ok(_texture) => ()
                };
            }
            
        }
        
    }

    fn handle_events(e: Event) {}
}
