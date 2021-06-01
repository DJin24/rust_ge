extern crate sdl2;

use self::sdl2::keyboard::Keycode;
use self::sdl2::mouse::MouseButton;
use self::sdl2::rect::Point;
use crate::rust_ge::rust_ge_engine::{WINDOW_WIDTH, WINDOW_HEIGHT};
use sdl2::event::Event;
use std::ops::Add;


#[derive(Copy, Clone, Debug)]
pub enum Mouse_button {
    left,
    middle,
    right,
}

impl Mouse_button {
    pub fn map_button(input: MouseButton) -> Option<Mouse_button> {
        match input {
            MouseButton::Left => Some(Mouse_button::left),
            MouseButton::Middle => Some(Mouse_button::middle),
            MouseButton::Right => Some(Mouse_button::right),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Posn {
    pub x: i32,
    pub y: i32,
}

impl Default for Posn {
    fn default() -> Posn {
        Posn { x: 0, y: 0 }
    }
}

impl Add for Posn {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

}

impl From<(i32, i32)> for Posn {
    fn from((x, y): (i32, i32)) -> Posn {
        Posn {x, y}
    }
}

impl From<Posn> for Point {
    fn from(p: Posn) -> Point {
        Point::new(p.x, p.y)
    }
}

impl From<Point> for Posn {
    fn from(p: Point) -> Posn {
        Posn{ x: p.x, y: p.y}
    }
}

impl Posn {
    pub fn new(x: i32, y: i32) -> Self {
        Posn {x, y}
    }

    /// calculates Pythagorean distance between two `Posn`s
    pub fn dist(a: Posn, b: Posn) -> i32 {
        let dx = (a.x - b.x).abs();
        let dy = (a.y - b.y).abs();
        ((dx * dx + dy * dy) as f64).sqrt() as i32
    }

    /// limits the `Posn` coordinates to be within the defined window size
    ///
    /// Window size is defined by the [`WINDOW_HEIGHT`] and [`WINDOW_WIDTH`] constants
    pub fn bounded(&self) -> Self {
        Posn { x: i32::max(0, i32::min(self.x, WINDOW_WIDTH as i32)),
            y: i32::max( 0,i32::min(self.y, WINDOW_HEIGHT as i32))
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Key {
    pub key_type: Type,
    pub code: i32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Type {
    /// Indicates a key with an Unicode value, which can be gotten
    /// with Key::code() const.
    code,
    /// The up arrow key.
    up,
    /// The down arrow key.
    down,
    /// The left arrow key.
    left,
    /// The right arrow key.
    right,
    /// The shift key.
    shift,
    /// The control key
    control,
    /// The alt or option key
    alt,
    /// The command or meta key
    command,
    /// Any other, unknown or invalid key.
    other,
}

impl Key {
    fn new_key_from_type(key_type: Type) -> Key {
        Key { key_type, code: 0 }
    }

    fn new_key_from_code(c: i32) -> Key {
        Key {
            key_type: Type::code,
            code: c,
        }
    }

    pub fn map_ascii(input: i32) -> Option<Key> {
        if input >= 0 && input < 128 {
            Some(Key::new_key_from_code(input))
        } else {
            None
        }
    }

    pub fn map_char(input: char) -> Option<Key> {
        Key::map_ascii(input as i32)
    }

    pub fn map_key(input: Keycode) -> Option<Key> {
        if let Some(key) = Key::map_ascii(input as i32) {
            Some(key)
        } else {
            match input {
                Keycode::Right => Some(Key::new_key_from_type(Type::right)),
                Keycode::Left => Some(Key::new_key_from_type(Type::left)),
                Keycode::Down => Some(Key::new_key_from_type(Type::down)),
                Keycode::Up => Some(Key::new_key_from_type(Type::up)),
                Keycode::KpEnter => Some(Key::new_key_from_code('\r' as i32)),
                Keycode::LCtrl => Some(Key::new_key_from_type(Type::control)),
                Keycode::LShift => Some(Key::new_key_from_type(Type::shift)),
                Keycode::LAlt => Some(Key::new_key_from_type(Type::alt)),
                Keycode::LGui => Some(Key::new_key_from_type(Type::command)),
                Keycode::RCtrl => Some(Key::new_key_from_type(Type::control)),
                Keycode::RShift => Some(Key::new_key_from_type(Type::shift)),
                Keycode::RAlt => Some(Key::new_key_from_type(Type::alt)),
                Keycode::RGui => Some(Key::new_key_from_type(Type::command)),
                _ => None,
            }
        }
    }

    pub fn is_textual(&self) -> bool {
        self.key_type == Type::code
    }
}
