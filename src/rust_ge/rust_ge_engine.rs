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
        // move initialization
        loop {
            let sprites = HashSet::<Sprite>::new(); // maybe &Sprite, though might be confusing with lifetimes
            self.game.draw(&mut sprites);
            let surfaces = sprites.drain().map(|sprite| sprite.to_sdl_surface());
        }
        
    }

    fn handle_events(e: Event) {}
}
