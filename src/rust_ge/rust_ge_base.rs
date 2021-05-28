extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use crate::rust_ge::frame_rate::FrameRate;

pub trait AbstractGame {
    fn run(&self) {
        let mut frame_rate = FrameRate::new(5);
    
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut i = 0;
        let mut _dt = Duration::from_secs(0);
        'running: loop {
            i = (i + 1) % 255;
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            // The rest of the game loop goes here...

            canvas.present();
            _dt = frame_rate.wait_for_next_frame();
        }
    }

    /*fn draw(&self, sprites: &Rust_ge_sprite_set);

    fn on_frame(&self, dt: f64);

    fn on_key(&self, key: Rust_ge_key);

    fn on_key_down(&self, key: Rust_ge_key);

    fn on_key_up(&self, key: Rust_ge_key);

    fn on_mouse_down(&self, mouse_button: Mouse_button, posn: Position);

    fn on_mouse_up(&self, mouse_button: Mouse_button, posn: Position);

    fn on_mouse_move(&self, mouse_button: Mouse_button, posn: Position);

    fn on_start(&self);

    fn on_quit(&self);*/
}