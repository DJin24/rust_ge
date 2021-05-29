use ::std::thread::sleep;
use ::std::time::{Duration, Instant};

#[derive(Debug)]
pub struct FrameRate {
    last_frame: Instant,
    curr_fps: usize,
    target_rate: usize,
    target_dt: Duration,
}

impl FrameRate {
    pub fn new(target_rate: usize) -> Self {
        Self {
            last_frame: Instant::now(),
            curr_fps: target_rate,
            target_rate,
            target_dt: Duration::from_micros((1_000_000 / target_rate) as u64),
        }
    }
    pub fn wait_for_next_frame(&mut self) -> Duration {
        let now = Instant::now();
        let dt = now - self.last_frame;
        if dt > self.target_dt {
            self.curr_fps = (1_000_000u128 / dt.as_micros()) as usize;
            self.last_frame = now;
            dt
        } else {
            sleep(self.target_dt - dt);
            self.curr_fps = self.target_rate;
            self.last_frame = Instant::now();
            self.target_dt
        }
    }
}
