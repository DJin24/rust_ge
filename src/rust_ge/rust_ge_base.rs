extern crate sdl2;

use ::sdl2::EventPump;
use crate::rust_ge::rust_ge_engine::Engine;
use crate::rust_ge::rust_ge_event::{Key, Mouse_button, Posn};
use crate::rust_ge::sprites::Sprite;
use ::sdl2::event::Event;
use ::sdl2::keyboard::Keycode;
use ::sdl2::pixels::Color;
use ::std::time::Duration;
use crate::rust_ge::frame_rate::FrameRate;
use ::std::collections::HashSet;
use crate::rust_ge::sprites::Sprite;
use ::std::cell::RefCell;
use ::std::rc::Rc;

pub trait AbstractGame: Sized {
    fn new() -> Self;
    fn run(game: &mut Self) {
        let engine = Rc::new(Engine::new(60));
        game.set_engine(engine);
        let e = game.engine();
        e.run(game);
    }
    
    fn engine(&self) -> Rc<Engine>;
    fn set_engine(&mut self, engine: Rc<Engine>);

    //TODO Add sprite renderer and sprite set so we can draw stuff
    fn draw(&self, dt: Duration, sprites: &mut Vec<Sprite>) {}

    fn on_frame(&self, dt: f64) {}

    fn on_key(&self, key: Key) {}

    fn on_key_down(&self, key: Key) {}

    fn on_key_up(&self, key: Key) {}

    fn on_mouse_down(&self, mouse_button: Mouse_button, posn: Posn) {}

    fn on_mouse_up(&self, mouse_button: Mouse_button, posn: Posn) {}

    fn on_mouse_move(&self, posn: Posn) {}

    fn on_start(&self) {}

    fn on_quit(&self) {}

    fn handle_events(&self, event: Event) {
        match event {
            Event::TextInput { text, .. } => {
                for c in text.chars() {
                    if let Some(key) = Key::map_char(c) {
                        self.on_key(key);
                    }
                }
            }
            Event::KeyDown {
                keycode: Some(key_code),
                repeat: repeat,
                ..
            } => {
                if let Some(key) = Key::map_key(key_code) {
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
                if let Some(key) = Key::map_key(key_code) {
                    println!("{:?}", key);
                    self.on_key_up(key);
                };
            }
            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => {
                if let Some(mouse_button) = Mouse_button::map_button(mouse_btn) {
                    println!("{:?}", mouse_button);
                    self.on_mouse_down(mouse_button, Posn { x, y });
                }
            }
            Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } => {
                if let Some(mouse_button) = Mouse_button::map_button(mouse_btn) {
                    println!("{:?}", mouse_button);
                    self.on_mouse_up(mouse_button, Posn { x, y });
                }
            }
            Event::MouseMotion { x, y, .. } => self.on_mouse_move(Posn { x, y }),
            _ => {}
        }
    }
}
