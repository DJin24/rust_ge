mod rust_ge;

use crate::rust_ge::rust_ge_base::Abstract_game;

struct TestGame {}

impl Abstract_game for TestGame {
    fn on_start(&self) {
        println!("STARTED");
    }

    fn on_quit(&self) {
        println!("QUITTED");
    }
}

pub fn main() {
    let testGame = TestGame {};
    testGame.run();
}
