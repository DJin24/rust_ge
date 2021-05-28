#[derive(Debug)]
pub struct FrameRate {
    last_frame: std::time::Instant,
    curr_fps: usize,
    target_rate: usize,
    target_dt: std::time::Duration,
}

impl FrameRate {
    pub fn new(target_rate: usize) -> Self {
        Self {
            last_frame: std::time::Instant::now(),
            curr_fps: target_rate,
            target_rate,
            target_dt: std::time::Duration::from_micros((1_000_000 / target_rate) as u64),
        }
    }
    pub fn wait_for_next_frame(&mut self) -> std::time::Duration {
        let now = std::time::Instant::now();
        let dt = now - self.last_frame;
        if dt > self.target_dt {
            self.curr_fps = (1_000_000u128
                / dt.as_micros())
                as usize;
            self.last_frame = now;
            dt
        }
        else {
            std::thread::sleep(self.target_dt - dt);
            self.curr_fps = self.target_rate;
            self.last_frame = std::time::Instant::now();
            self.target_dt
        }
    }
}

