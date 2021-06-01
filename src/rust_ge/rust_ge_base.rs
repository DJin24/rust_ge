extern crate sdl2;


use crate::rust_ge::rust_ge_engine::Engine;
use crate::rust_ge::rust_ge_event::{Key, Mouse_button, Posn};
use crate::rust_ge::sprites::{ShapeTypes, Sprite};
use crate::rust_ge::frame_rate::FrameRate;
use ::sdl2::EventPump;
use ::sdl2::event::Event;
use ::sdl2::keyboard::Keycode;
use ::sdl2::pixels::Color;
use ::std::time::Duration;
use ::std::collections::HashSet;
use ::std::cell::RefCell;
use ::std::rc::Rc;

// Games have Rc's to their engines when they are run
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
    fn draw(&mut self, dt: Duration, sprites: &mut Vec<Sprite>) {}

    fn on_frame(&mut self, dt: Duration) {}

    fn on_key(&mut self, key: Key) {}

    fn on_key_down(&mut self, key: Key) {}

    fn on_key_up(&mut self, key: Key) {}

    fn on_mouse_down(&mut self, mouse_button: Mouse_button, posn: Posn) {}

    fn on_mouse_up(&mut self, mouse_button: Mouse_button, posn: Posn) {}

    fn on_mouse_move(&mut self, posn: Posn) {}

    fn on_start(&mut self) {}

    fn on_quit(&mut self) {}
}
