mod rust_ge;
use crate::rust_ge::rust_ge_base::AbstractGame;
use crate::rust_ge::rust_ge_engine::Engine;
use crate::rust_ge::rust_ge_event::{Mouse_button, Posn};
use crate::rust_ge::sprites::Sprite;
use ::sdl2::pixels::Color;
use ::std::collections::HashSet;
use ::std::time::Duration;
use std::cell::RefCell;
use std::fmt::DebugList;
use std::path::Path;
use std::rc::Rc;

struct TestGame {
    engine: Option<Rc<Engine>>,

    click_loc: Posn,
    r_click_loc: Posn,
}

impl AbstractGame for TestGame {
    fn new() -> Self {
        return Self {
            engine: None,
            click_loc: Default::default(),
            r_click_loc: Default::default(),
        }
    }


    fn on_start(&mut self) {
        println!("STARTED");
    }

    fn set_engine(&mut self, engine: Rc<Engine>) {
        self.engine = Some(engine.clone());
    }

    fn engine(&self) -> Rc<Engine> {
        self.engine.as_ref().unwrap().clone()
    }

    fn on_mouse_down(&mut self, mouse_button: Mouse_button, posn: Posn) {
        match mouse_button {
            Mouse_button::left => self.click_loc = posn,
            Mouse_button::right => self.r_click_loc = posn,
            _ => ()
        }
    }

    fn draw(&mut self, dt: Duration, sprites: &mut Vec<Sprite>) {
        //let my_box = Sprite::filled_rectangle(self.click_loc.x, self.click_loc.y, 20, 20, Color::RED);
        let filled_box = Sprite::rectangle(self.r_click_loc.x, self.r_click_loc.y, 20, 20, Color::GREEN);
        let my_box = Sprite::filled_rectangle(self.click_loc.x, self.click_loc.y, 40, 40, Color::RED);
        sprites.push(my_box);
        sprites.push(filled_box);
    }

    fn on_quit(&mut self) {
        println!("QUITTED");
    }
}

pub fn main() {
    let mut test_game = TestGame::new();
    AbstractGame::run(&mut test_game);
}
