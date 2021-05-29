mod rust_ge;
use crate::rust_ge::rust_ge_base::AbstractGame;
use crate::rust_ge::sprites::Sprite;
use ::std::collections::HashSet;
use ::std::time::Duration;


struct TestGame {}

impl AbstractGame for TestGame {
    fn on_start(&self) {
        println!("STARTED");
    }

    fn on_quit(&self) {
        println!("QUIT");
    }
    
    fn draw(&self, dt: Duration, sprites: &mut HashSet<Sprite>) {
        
    }
}

pub fn main() {
    let test_game = TestGame{};
    test_game.run();
}
