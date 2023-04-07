
#[derive(Clone)]
pub struct FluxNoteConfig {
    pub ar: f32, // approach rate
    pub ad: f32, // approach distance
    pub fs: u32, // fade steps TODO: Implement fade
    pub hitbox: f32,
    pub approach_time: f32,
}

#[derive(Clone)]
pub struct FluxCursorConfig {
    pub sens: f32,
    pub size: f32,
    pub edge_buffer: f32,
}

#[derive(Clone)]
pub struct FluxAudioConfig {
    pub volume: f64,
    pub speed: f64,
    pub offset: i64,
}

#[derive(Clone)]
pub struct FluxSetsConfig {
    pub hit: &'static str,
    pub cursor: &'static str,
    pub note: &'static str,
}

#[derive(Clone)]
pub struct FluxMiscConfig {
    pub debug: bool,
    pub play_area_width: f32,
    pub play_area_height: f32,
}

#[derive(Clone)]
pub struct FluxConfig {
    pub note: FluxNoteConfig,
    pub cursor: FluxCursorConfig,
    pub audio: FluxAudioConfig,
    pub sets: FluxSetsConfig,
    pub misc: FluxMiscConfig,
}
