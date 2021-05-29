extern crate sdl2;

use self::sdl2::event::Event;
use crate::rust_ge::rust_ge_base::AbstractGame;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use std::collections::HashSet;
use crate::rust_ge::sprites::Sprite;
use crate::rust_ge::frame_rate::FrameRate;
use std::rc::Rc;
use ::sdl2::render::Canvas;
use ::sdl2::video::Window;

pub struct Engine {
    canvas: Canvas<Window>,
    frame_rate: FrameRate,
    event_pump: ::sdl2::EventPump
}

impl Engine {
    pub fn new(fr: usize) -> Self {
        let mut frame_rate = FrameRate::new(fr);

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        Self {
            canvas,
            frame_rate,
            event_pump
        }
    }

    pub fn run<Game: AbstractGame + Sized>(&mut self, game: &mut Game) {
        game.on_start();
        
        self.canvas.set_draw_color(Color::RGB(0, 255, 255));
        self.canvas.clear();
        self.canvas.present();
        let mut i = 0;
        let mut dt = Duration::from_secs(0);
        'running: loop {
            self.canvas.clear();
            let mut sprites = Vec::<Sprite>::new(); // maybe &Sprite, though might be confusing with lifetimes
            game.draw(dt, &mut sprites);
            let surfaces = sprites.iter().map(|sprite| sprite.as_sdl_surface());
            let texture_creator = self.canvas.texture_creator();
            for surface in surfaces {
                match texture_creator.create_texture_from_surface(surface){
                    Err(_) => panic!("failed to create texture on window"),
                    Ok(_texture) => ()
                };
            }
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => game.handle_events(event),
                }
            }
            self.canvas.present();
            
            dt = self.frame_rate.wait_for_next_frame();
        }
        game.on_quit()
    }
    
    fn handle_events(e: Event) {}
}