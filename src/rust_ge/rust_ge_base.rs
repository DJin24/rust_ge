extern crate sdl2;

use self::sdl2::EventPump;
use crate::rust_ge::rust_ge_engine::Engine;
use crate::rust_ge::rust_ge_event::{map_key, Key, Mouse_button};
use ::sdl2::event::Event;
use ::sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use crate::rust_ge::frame_rate::FrameRate;
use std::collections::HashSet;
use crate::rust_ge::sprites::Sprite;

pub trait AbstractGame: Sized {
    fn new() -> Self;
    fn run(game: &mut Self) {
        let engine = Engine::new(60);
        game.set_engine(engine);
        let mut e = game.engine().unwrap();
        e.run(game);
    }
    
    fn engine(&mut self) -> Option<&mut Engine>;
    fn set_engine(&mut self, engine: Engine);

    //TODO Add sprite renderer and sprite set so we can draw stuff
    fn draw(&self, dt: Duration, sprites: &mut Vec<Sprite>) {}

    fn on_frame(&self, dt: f64) {}

    fn on_key(&self, key: Key) {}

    fn on_key_down(&self, key: Key) {}

    fn on_key_up(&self, key: Key) {}

    //TODO These functions, primarily the position stuff
    /*fn on_mouse_down(&self, mouse_button: Mouse_button, posn: Position) {}

    fn on_mouse_up(&self, mouse_button: Mouse_button, posn: Position) {}

    fn on_mouse_move(&self, mouse_button: Mouse_button, posn: Position) {}*/

    fn on_start(&self) {}

    fn on_quit(&self) {}

    fn handle_events(&self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(key_code),
                repeat: repeat,
                ..
            } => {
                if let Some(key) = map_key(key_code) {
                    println!("{:?}", key);
                    if !repeat {
                        self.on_key_down(key);
                    }
                    if !key.is_textual() {
                        self.on_key(key);
                    }
                };
            }
            Event::KeyUp {
                keycode: Some(key_code),
                ..
            } => {
                if let Some(key) = map_key(key_code) {
                    println!("{:?}", key);
                    self.on_key_up(key);
                };
            }
            //TODO Mouse events
            _ => {}
        }
    }
}
