mod rust_ge;

use crate::rust_ge::rust_ge_base::AbstractGame;
use crate::rust_ge::rust_ge_event::{Mouse_button, Posn};
use std::collections::HashSet;
use crate::rust_ge::sprites::{Sprite};
use sdl2::pixels::Color;

struct TestGame {
    click_loc: Posn,
    r_click_loc: Posn,
}

impl AbstractGame for TestGame {
    fn on_start(&mut self) {
        println!("STARTED");
    }

    fn on_mouse_down(&mut self, mouse_button: Mouse_button, posn: Posn) {
        match mouse_button {
            Mouse_button::left => self.click_loc = posn,
            Mouse_button::right => self.r_click_loc = posn,
            _ => ()
        }
    }

    fn draw(&mut self, sprites: &mut Vec<Sprite>) {
        let my_box = Sprite::filled_rectangle(self.click_loc.x, self.click_loc.y, 20, 20, Color::MAGENTA);
        let filled_box = Sprite::rectangle(self.r_click_loc.x, self.r_click_loc.y, 20, 20, Color::GREEN);
        sprites.push(my_box);
        sprites.push(filled_box);
    }

    fn on_quit(&mut self) {
        println!("QUITTED");
    }
}

pub fn main() {
    let mut test_game = TestGame { click_loc: Default::default(), r_click_loc: Default::default() };
    test_game.run();
}
