use crate::core::timer::FluxTimer;

pub struct FluxTimeManager {
    pub song_timer: FluxTimer,
    pub pause_timer: FluxTimer,
    pub unpause_timer: FluxTimer,
    pub paused: bool,
    pub paused_total: u64,
}

impl FluxTimeManager {
    pub fn new(speed: f64, offset: i64)  -> Self {
        Self {
            song_timer: FluxTimer::new(speed, offset),
            pause_timer: FluxTimer::new(speed, offset),
            unpause_timer: FluxTimer::new(speed, offset),
            paused: false,
            paused_total: 0,
        }
    }

    pub fn start(&mut self) {
        self.song_timer.start_with_speed();
        self.unpause_timer.start();
    }

    pub fn update_paused_total(&mut self) {
        self.paused_total += self.pause_timer.current_ms;
    }

    pub fn update(&mut self) {
        if self.paused {
            self.pause_timer.update();
            return;
        }
        self.song_timer.update_with_offset_and_speed(self.paused_total as i64);
        self.unpause_timer.update();
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.song_timer.set_speed(speed);
        self.pause_timer.set_speed(speed);
        self.unpause_timer.set_speed(speed);
    }

    pub fn reset(&mut self) {
        self.song_timer.reset();
        self.pause_timer.reset();
        self.unpause_timer.reset();
        self.paused = false;
        self.paused_total = 0;
    }
}