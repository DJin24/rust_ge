mod rust_ge;

use crate::rust_ge::rust_ge_base::Abstract_game;

struct TestGame {}

impl Abstract_game for TestGame {}

pub fn main() {
    let testGame = TestGame {};
    testGame.run();
}
