
pub struct FluxStatsManager {
    pub notes_hit: usize,
    pub notes_missed: usize,
    pub note_total: usize,
}

impl FluxStatsManager {
    pub fn new() -> Self {
        Self {
            notes_hit: 0,
            notes_missed: 0,
            note_total: 0,
        }
    }

    pub fn reset(&mut self) {
        self.note_total = 0;
        self.notes_hit = 0;
        self.notes_missed = 0;
    }
}