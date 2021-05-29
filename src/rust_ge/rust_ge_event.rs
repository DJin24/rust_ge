extern crate sdl2;

use self::sdl2::keyboard::Keycode;
use self::sdl2::mouse::MouseButton;
use sdl2::event::Event;

#[derive(Copy, Clone, Debug)]
pub enum Mouse_button {
    left,
    middle,
    right,
}

pub fn map_button(input: MouseButton) -> Option<Mouse_button> {
    match input {
        MouseButton::Left => Some(Mouse_button::left),
        MouseButton::Middle => Some(Mouse_button::middle),
        MouseButton::Right => Some(Mouse_button::right),
        _ => None,
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Posn {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct Key {
    key_type: Type,
    code: i32,
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

    pub fn is_textual(&self) -> bool {
        self.key_type == Type::code
    }
}

pub fn map_key(input: Keycode) -> Option<Key> {
    let ascii_val = input as i32;

    if ascii_val >= 0 && ascii_val < 128 {
        Some(Key::new_key_from_code(input as i32))
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
