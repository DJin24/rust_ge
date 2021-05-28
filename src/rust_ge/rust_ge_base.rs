extern crate sdl2;

use self::sdl2::EventPump;
use crate::rust_ge::rust_ge_engine::Engine;
use crate::rust_ge::rust_ge_event::{map_key, Key, Mouse_button};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub trait Abstract_game {
    fn run(&self) {
        self.on_start();

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
        'running: loop {
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

            //TODO Fix frame rate using Jackson's code
            let frame_time = Duration::new(0, 1_000_000_000u32 / 60);
            self.on_frame((frame_time.as_secs_f64()));

            canvas.present();
            ::std::thread::sleep(frame_time);
        }

        self.on_quit()
    }

    //TODO Add sprite renderer and sprite set so we can draw stuff
    //fn draw(&self, sprites: &Sprite_set) {}

    fn on_frame(&self, dt: f64) {}

    fn on_key(&self, key: Key) {}

    fn on_key_down(&self, key: Key) {}

    fn on_key_up(&self, key: Key) {}

    //TODO These functions, primarily the position stuff
    /*fn on_mouse_down(&self, mouse_button: Mouse_button, posn: Position) {}

    fn on_mouse_up(&self, mouse_button: Mouse_button, posn: Position) {}

    fn on_mouse_move(&self, mouse_button: Mouse_button, posn: Position) {}*/

    fn on_start(&self) {}

    fn on_quit(&self) {}

    fn handle_events(&self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(key_code),
                repeat: repeat,
                ..
            } => {
                if let Some(key) = map_key(key_code) {
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
                if let Some(key) = map_key(key_code) {
                    println!("{:?}", key);
                    self.on_key_up(key);
                };
            }
            //TODO Mouse events
            _ => {}
        }
    }
}
