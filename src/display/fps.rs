extern crate time;

use time::OffsetDateTime;

pub struct FPSCounter {
    frames: i32,
    last_fps: i32,
    last_time: i128,
}

impl FPSCounter {
    pub fn new() -> FPSCounter {
        Self {
            frames: 0,
            last_fps: 0,
            last_time: 0,
        }
    }

    pub fn get_fps(&mut self) -> i32 {
        let current_time =
            (OffsetDateTime::now_utc() - OffsetDateTime::unix_epoch()).whole_nanoseconds();
        self.frames += 1;
        if current_time - self.last_time >= 1000000000 {
            self.last_fps = self.frames;
            debug!("fps: {}", self.frames);
            self.frames = 0;
            self.last_time =
                (OffsetDateTime::now_utc() - OffsetDateTime::unix_epoch()).whole_nanoseconds();
            return self.frames;
        }
        self.last_fps
    }
}
