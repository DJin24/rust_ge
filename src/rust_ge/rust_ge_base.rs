extern crate sdl2;

use self::sdl2::EventPump;
use crate::rust_ge::frame_rate::FrameRate;
use crate::rust_ge::rust_ge_engine::Engine;
use crate::rust_ge::rust_ge_event::{Key, Mouse_button, Posn};
use crate::rust_ge::sprites::Sprite;
use ::sdl2::event::Event;
use ::sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::collections::HashSet;
use std::time::Duration;

pub trait AbstractGame {
    fn run(&self) {
        self.on_start();
        let mut frame_rate = FrameRate::new(5);

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut i = 0;
        let mut dt = Duration::from_secs(0);
        'running: loop {
            self.on_frame(dt.as_secs_f64());

            i = (i + 1) % 255;
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => self.handle_events(event),
                }
            }

            canvas.present();
            dt = frame_rate.wait_for_next_frame();
        }

        self.on_quit()
    }

    //TODO Add sprite renderer and sprite set so we can draw stuff
    fn draw(&self, sprites: &mut HashSet<Sprite>) {}

    fn on_frame(&self, dt: f64) {}

    fn on_key(&self, key: Key) {}

    fn on_key_down(&self, key: Key) {}

    fn on_key_up(&self, key: Key) {}

    fn on_mouse_down(&self, mouse_button: Mouse_button, posn: Posn) {}

    fn on_mouse_up(&self, mouse_button: Mouse_button, posn: Posn) {}

    fn on_mouse_move(&self, posn: Posn) {}

    fn on_start(&self) {}

    fn on_quit(&self) {}

    fn handle_events(&self, event: Event) {
        match event {
            Event::TextInput { text, .. } => {
                for c in text.chars() {
                    if let Some(key) = Key::map_char(c) {
                        self.on_key(key);
                    }
                }
            }
            Event::KeyDown {
                keycode: Some(key_code),
                repeat: repeat,
                ..
            } => {
                if let Some(key) = Key::map_key(key_code) {
                    println!("{:?}", key);
                    if !repeat {
                        self.on_key_down(key);
                    }
                    if !key.is_textual() {
                        self.on_key(key);
                    }
                };
            }
            Event::KeyUp {
                keycode: Some(key_code),
                ..
            } => {
                if let Some(key) = Key::map_key(key_code) {
                    println!("{:?}", key);
                    self.on_key_up(key);
                };
            }
            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => {
                if let Some(mouse_button) = Mouse_button::map_button(mouse_btn) {
                    println!("{:?}", mouse_button);
                    self.on_mouse_down(mouse_button, Posn { x, y });
                }
            }
            Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } => {
                if let Some(mouse_button) = Mouse_button::map_button(mouse_btn) {
                    println!("{:?}", mouse_button);
                    self.on_mouse_up(mouse_button, Posn { x, y });
                }
            }
            Event::MouseMotion { x, y, .. } => self.on_mouse_move(Posn { x, y }),
            _ => {}
        }
    }
}
