mod rust_ge;
use crate::rust_ge::rust_ge_base::AbstractGame;

struct TestGame {}

impl AbstractGame for TestGame {
    fn on_start(&self) {
        println!("STARTED");
    }

    fn on_quit(&self) {
        println!("QUITTED");
    }
}

pub fn main() {
    let test_game = TestGame{};
    test_game.run();
}
