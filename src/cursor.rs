use std::path::PathBuf;

use nannou::prelude::*;

use crate::game::{PLAY_AREA_WIDTH, PLAY_AREA_HEIGHT};
pub struct FluxCursor {
    x: f32,
    y: f32,
    pub texture: wgpu::Texture,
    size: f32,
}

impl FluxCursor {
    pub fn new(app: &App, cursor_size: f32, cursor_path: PathBuf) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            size: cursor_size,
            texture: wgpu::Texture::from_path(app, cursor_path).expect("Failed to load cursor in data/cursor.png"),
        }
    }
    pub fn draw(&self, draw: Draw) {
        draw.texture(&self.texture).x_y(self.x, self.y).w_h(self.size, self.size);
    }
    pub fn cursor_move(&mut self, mp: Point2, sens: f32) {
        self.x = mp.x * sens;
        self.y = mp.y * sens;
    }
    pub fn change_cursor_size(&mut self, size: f32) {
        self.size = size;
    }

    pub fn lock_cursor_to_play_area(&mut self) {
        self.x = self.x.clamp(-(PLAY_AREA_WIDTH/2.0), PLAY_AREA_WIDTH/2.0);
        self.y = self.y.clamp(-(PLAY_AREA_HEIGHT/2.0), PLAY_AREA_HEIGHT/2.0);
    }
}