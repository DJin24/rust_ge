//! This is a sample game using the `rust_ge` game engine. By no means is it an
//! exhaustive example of all the engine's functionality.

mod rust_ge;
use crate::rust_ge::rust_ge_base::AbstractGame;
use crate::rust_ge::rust_ge_engine::{Engine, WINDOW_WIDTH, WINDOW_HEIGHT};
use crate::rust_ge::rust_ge_event::{Mouse_button, Posn, Key, Type};
use crate::rust_ge::sprites::{Sprite, ShapeTypes};
use ::sdl2::pixels::Color;
use ::std::collections::HashSet;
use ::std::time::Duration;
use std::cell::RefCell;
use std::fmt::DebugList;
use std::path::Path;
use std::rc::Rc;


struct TestGame {
    engine: Option<Rc<Engine>>,

    /// These fields define the state of the game
    target_loc: Posn,
    dest_loc: Posn,

}

impl AbstractGame for TestGame {



    fn new() -> Self {
        return Self {
            engine: None,
            target_loc: Default::default(),
            dest_loc: Default::default(),
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
            Mouse_button::left => self.target_loc = posn,
            Mouse_button::right => self.dest_loc = posn,
            _ => ()
        }
    }

    fn on_key(&mut self, key: Key) {
        let mut dx: i32 = 0;
        let mut dy: i32 = 0;
        match key.key_type {
            Type::down => {
                dy = 10;
            },
            Type::up => {
                dy = -10;
            },
            Type::right => {
                dx = 10;
            },
            Type::left => {
                dx = -10;
            }
            _ => ()
        }

        self.dest_loc = (self.dest_loc + Posn::from((dx, dy))).bounded()

    }

    fn draw(&mut self, dt: Duration, sprites: &mut Vec<Sprite>) {
        let mut target_color : Color = Color::RED;

        let test_sprite = Sprite::rectangle(self.target_loc.x - 20, self.target_loc.y - 20, 40, 40, Color::BLACK);

        if contains(test_sprite, self.dest_loc) {
            target_color = Color::WHITE;
        }

        //let my_box = Sprite::filled_rectangle(self.click_loc.x, self.click_loc.y, 20, 20, Color::RED);
        let user_box = Sprite::rectangle(self.dest_loc.x - 10, self.dest_loc.y - 10, 20, 20, Color::GREEN);
        let target_box = Sprite::filled_rectangle(self.target_loc.x - 20, self.target_loc.y - 20, 40, 40, target_color);

        add_dashed_line(self.dest_loc, self.target_loc, Color::CYAN, sprites);

        sprites.push(target_box);
        sprites.push(user_box);

    }

    fn on_quit(&mut self) {
        println!("QUITTED");
    }



}

fn add_dashed_line(p1: Posn, p2: Posn, color: Color, sprite_vec: &mut Vec<Sprite>) {
    const DASH_LENGTH : i32 = 20;

    //let mut segments : Vec<Sprite> = Vec::new();

    let length: i32 = Posn::dist(p1, p2);

    if length > 0 {
        let x_factor: i32 = (p2.x - p1.x).signum() * ((p1.x - p2.x).abs() * DASH_LENGTH) / length;
        let y_factor: i32 = (p2.y - p1.y).signum() * ((p1.y - p2.y).abs() * DASH_LENGTH) / length;

        let mut d: i32 = 0;
        let mut x: i32 = p1.x;
        let mut y: i32 = p1.y;

        while d < length {
            sprite_vec.push(Sprite::line(x, y, x + x_factor, y + y_factor, color));
            d += DASH_LENGTH * 2;
            x += x_factor * 2;
            y += y_factor * 2;
        }

        //sprite_vec.push(Sprite::line(x, y, p2.x, p2.y, color));
    }

    //segments
}

fn contains( bounds: Sprite, point: Posn) -> bool {
    // could also be
    //    bounds.shape_type() == ShapeTypes::Rect && point.x <= b_s.right() && point.x >= b_s.left && point.y <= b_s.bottom() && point.y >= b_s.top()
    // but not very readable
    if bounds.shape_type() == &ShapeTypes::Rect || bounds.shape_type() == &ShapeTypes::FilledRect {
        let b_s = &bounds.shape();
        point.x <= b_s.right() && point.x >= b_s.left() && point.y <= b_s.bottom() && point.y >= b_s.top()
    } else {
        false
    }
}

pub fn main() {
    let mut test_game = TestGame::new();
    AbstractGame::run(&mut test_game);
}
