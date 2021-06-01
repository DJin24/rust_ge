//! This module contains structs and methods related to events, which right now is just user input
//!
//! The [`Posn`] struct will be moved to it's own file in the future as it is less related to
//! events and more so just geometry
extern crate sdl2;

use self::sdl2::keyboard::Keycode;
use self::sdl2::mouse::MouseButton;
use self::sdl2::rect::Point;
use crate::rust_ge::rust_ge_engine::{WINDOW_HEIGHT, WINDOW_WIDTH};
use sdl2::event::Event;
use std::ops::Add;

/// Representation of different buttons on a mouse
///
/// Supports left, middle, and right click
#[derive(Copy, Clone, Debug)]
pub enum Mouse_button {
    left,
    middle,
    right,
}

/// Mapping of sdl2 [`MouseButton`]s to this `Mouse_button` enum
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

/// Represents a point on the window
///
/// Currently, it can have negative coordinates, although that will be changed soon so that `x` and `y`
/// are `u32`
///
/// This struct is very similar to the [sdl2] [`Point`] struct. In the future, `Posn` might be replaced
/// entirely by `Point`, or at least just be a wrapper for it. `Point` does use signed values, which
/// doesn't really make sense for this coordinate system, but that's not a major issue.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Posn {
    pub x: i32,
    pub y: i32,
}


impl Default for Posn {
    /// Default `Posn` is at (0,0)
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
        Posn { x, y }
    }
}

impl From<Posn> for Point {
    fn from(p: Posn) -> Point {
        Point::new(p.x, p.y)
    }
}

impl From<Point> for Posn {
    fn from(p: Point) -> Posn {
        Posn { x: p.x, y: p.y }
    }
}

impl Posn {
    /// Creates a Posn with the given coordinates
    pub fn new(x: i32, y: i32) -> Self {
        Posn { x, y }
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
        Posn {
            x: i32::max(0, i32::min(self.x, WINDOW_WIDTH as i32)),
            y: i32::max(0, i32::min(self.y, WINDOW_HEIGHT as i32)),
        }
    }
}

/// Representation of a key input from a keyboard
#[derive(Copy, Clone, Debug)]
pub struct Key {
    pub key_type: Type,
    /// The ASCII value for a key with code type
    pub code: i32,
}

/// Further categorization of keys
///
/// Differentiates between Unicode representable keys (`code`), special keys that might be used
/// for game control (like the arrow keys, shift, control), and a generic "other" key type
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
    /// Returns a key of a given type
    /// [`Type::code`] keys get a code value of 0, which may not be desirable. To specify a code
    /// value, use the [new_key_from_code] method.
    fn new_key_from_type(key_type: Type) -> Key {
        Key { key_type, code: 0 }
    }

    /// Returns a key of `Type` `code`, with specified code value
    fn new_key_from_code(c: i32) -> Key {
        Key {
            key_type: Type::code,
            code: c,
        }
    }

    /// Returns a code type key from an valid ASCII i32 value
    ///
    /// Similar to `new_key_from_code`, except this takes a numerical character value instead of
    /// a char
    ///
    /// If `input` is not a valid ASCII value (in the range \[0,127\]), `map_ascii` will return `None`
    /// Otherwise, return type is a `Some(Key)` of type `Type::code`
    pub fn map_ascii(input: i32) -> Option<Key> {
        if input >= 0 && input < 128 {
            Some(Key::new_key_from_code(input))
        } else {
            None
        }
    }

    /// Returns a code type key from an ASCII character
    ///
    /// If `input` is not a valid ASCII character, `map_ascii` will return `None`
    /// Otherwise, return type is a `Some(Key)` of type `Type::code`
    pub fn map_char(input: char) -> Option<Key> {
        Key::map_ascii(input as i32)
    }

    /// Converts a sdl2 `Keycode` to a `Key`
    ///
    /// Returns `None` if `input` does not match an ASCII character value or one of the special key
    /// types in the `Type` enum.
    ///
    /// *Note: no physical keys will generate a `Key` with type `Type::other` currently*
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

    /// Checks if a `Key` represents an ASCII character
    ///
    /// More specifically, this checks if a `Key` has a code type. This makes no guarantees that the
    /// represented ASCII value is an alphanumeric value or even printable
    pub fn is_textual(&self) -> bool {
        self.key_type == Type::code
    }
}
