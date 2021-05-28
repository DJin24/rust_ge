extern crate sdl2;

use self::sdl2::event::Event;
use crate::rust_ge::rust_ge_base::Abstract_game;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub struct Engine {}

impl Engine {
    pub fn run(&self) {}

    fn handle_events(e: Event) {}
}
