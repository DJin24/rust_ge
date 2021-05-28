extern crate sdl2;

use self::sdl2::event::Event;
use crate::rust_ge::rust_ge_base::AbstractGame;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use std::collections::HashSet;
use crate::rust_ge::sprites::Sprite;
use crate::rust_ge::frame_rate::FrameRate;

pub struct Engine<Game: AbstractGame + Sized> {
    game: Game
}

impl<Game: AbstractGame + Sized> Engine<Game> {
    pub fn new(game: Game) -> Self {
        let engine = Self {
            game
        };
        game.set_engine(&engine);
        engine
    }

    pub fn run(&self) {
        self.game.on_start();
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
            canvas.clear();
            let mut sprites = HashSet::<Sprite>::new(); // maybe &Sprite, though might be confusing with lifetimes
            self.game.draw(dt, &mut sprites);
            let surfaces = sprites.iter().map(|sprite| sprite.as_sdl_surface());
            let texture_creator = canvas.texture_creator();
            for surface in surfaces {
                match texture_creator.create_texture_from_surface(surface){
                    Err(_) => panic!("failed to create texture on window"),
                    Ok(_texture) => ()
                };
            }
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => self.game.handle_events(event),
                }
            }
            canvas.present();
            
            dt = frame_rate.wait_for_next_frame();
        }

        self.game.on_quit()
        
        
        
    }

    fn handle_events(e: Event) {}
}
