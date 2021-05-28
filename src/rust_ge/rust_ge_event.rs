extern crate sdl2;

use self::sdl2::keyboard::Keycode;
use self::sdl2::mouse::MouseButton;
use sdl2::event::Event;

enum Mouse_button {
    left,
    middle,
    right,
}

fn map_button(input: MouseButton) -> Option<Mouse_button> {
    match input {
        MouseButton::Left => Some(Mouse_button::left),
        MouseButton::Middle => Some(Mouse_button::middle),
        MouseButton::Right => Some(Mouse_button::right),
        _ => None,
    }
}

struct Key {
    key_type: Type,
    code: char,
}

enum Type {
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
    fn new(key_type: Type) -> Key {
        Key {
            key_type,
            code: '\0',
        }
    }

    fn new(c: char) -> Key {
        Key {
            key_type: Type::code,
            code: c,
        }
    }
}

fn map_key(input: Keycode) -> Option<Key> {}
