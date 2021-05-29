mod rust_ge;
use crate::rust_ge::rust_ge_base::AbstractGame;
use crate::rust_ge::sprites::Sprite;
use ::std::collections::HashSet;
use ::std::time::Duration;
use ::sdl2::pixels::Color;
use crate::rust_ge::rust_ge_engine::Engine;
use std::rc::Rc;

struct TestGame {
    engine: Option<Engine>
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
    
    fn set_engine(&mut self, engine: Engine) {
        self.engine = Some(engine);
    }
    
    fn engine(&mut self) -> Option<&mut Engine> {
        self.engine.as_mut()
    }

    fn on_quit(&self) {
        println!("QUIT");
    }
    
    fn draw(&self, dt: Duration, sprites: &mut Vec<Sprite>) {
        let sprite = Sprite::rectangle(0, 0, 16, 16, Color::BLACK);
        sprites.push(sprite);
    }
}

pub fn main() {
    let mut test_game = TestGame::new();
    AbstractGame::run(&mut test_game);
}
