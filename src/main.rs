mod rust_ge;
use crate::rust_ge::rust_ge_base::AbstractGame;

struct TestGame {}

impl AbstractGame for TestGame {

}

pub fn main() {
    let test_game = TestGame{};
    test_game.run();
}
