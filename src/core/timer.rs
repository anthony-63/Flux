use std::time::UNIX_EPOCH;

pub struct FluxTimer {
    pub start_ms: u64,
    pub current_ms: u64,
    pub speed: f64,
    pub offset: i64,
}

impl FluxTimer {
    pub fn new(speed: f64, offset: i64) -> Self {
        Self {
            start_ms: 0,
            current_ms: 0,
            speed,
            offset,
        }
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    pub fn start_with_speed(&mut self) {
        self.start_ms = ((std::time::SystemTime::now().duration_since(UNIX_EPOCH).expect("Time traveler?").as_millis() as i64 + self.offset) as f64 * self.speed) as u64;
    }

    pub fn start(&mut self) {
        self.start_ms = (std::time::SystemTime::now().duration_since(UNIX_EPOCH).expect("Time traveler?").as_millis() as i64 + self.offset) as u64;
    }

    pub fn reset(&mut self) {
        self.start_ms = 0;
        self.current_ms = 0;
    }

    pub fn update(&mut self) {
        let current_time = std::time::SystemTime::now().duration_since(UNIX_EPOCH).expect("Time traveler?").as_millis() as f64 * self.speed;
        self.current_ms = current_time as u64 - self.start_ms;
    }

    pub fn update_with_offset_and_speed(&mut self, offset: i64) {
        let current_time = (std::time::SystemTime::now().duration_since(UNIX_EPOCH).expect("Time traveler?").as_millis() as f64 * self.speed) as i64 - offset;
        self.current_ms = current_time as u64 - self.start_ms;
    }
}