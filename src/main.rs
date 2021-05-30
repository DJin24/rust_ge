mod rust_ge;
use crate::rust_ge::rust_ge_base::AbstractGame;
use crate::rust_ge::sprites::Sprite;
use ::std::collections::HashSet;
use ::std::time::Duration;
use ::sdl2::pixels::Color;
use crate::rust_ge::rust_ge_engine::Engine;
use std::rc::Rc;
use std::cell::RefCell;

struct TestGame {
    engine: Option<Rc<Engine>>
}

impl AbstractGame for TestGame {
    fn new() -> Self {
        return Self{
            engine: None
        }
    }

    fn on_start(&self) {
        println!("STARTED");
    }
    
    fn set_engine(&mut self, engine: Rc<Engine>) {
        self.engine = Some(engine.clone());
    }
    
    fn engine(&self) -> Rc<Engine> {
        self.engine.as_ref().unwrap().clone()
    }

    fn on_quit(&self) {
        println!("QUIT");
    }
    
    // Should create a rectangle in the middle of the screen
    fn draw(&self, dt: Duration, sprites: &mut Vec<Sprite>) {
        let sprite = Sprite::rectangle(0, 0, 16, 16, Color::BLACK);
        sprites.push(sprite);
    }
}

pub fn main() {
    let mut test_game = TestGame::new();
    AbstractGame::run(&mut test_game);
}
