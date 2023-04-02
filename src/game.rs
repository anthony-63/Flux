use std::io::Cursor;

use nannou::prelude::*;

use crate::{maploader::FluxMap, WIDTH, HEIGHT};
pub struct FluxGame {
    map: FluxMap,
}


impl FluxGame {
    pub fn new() -> Self {
        Self {
            map: FluxMap::empty(),
        }
    }
    pub fn draw_before_loaded_map(&self, draw: Draw) {
        draw.background().color(PURPLE);
    }
    pub fn insert_map(&mut self, map: FluxMap) {
        self.map = map;
    }
    pub fn play_map_audio(&self) {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        let cursor = Cursor::new(self.map.mp3_data.clone());
        sink.append(rodio::decoder::Decoder::new_mp3(cursor).unwrap());

        sink.sleep_until_end();
    }
    pub fn draw_play_game(&self, draw: Draw) {
        draw.background().color(BLUE);
        draw.text(&format!("{} - {}", self.map.artist, self.map.song_name)).color(WHITE).y(HEIGHT as f32 / 2.1).font_size(25).width(WIDTH as f32);
    }
}