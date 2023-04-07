use crate::{core::{config::FluxConfig, maploader::FluxMap}, managers::{stats_manager::FluxStatsManager, time_manager::FluxTimeManager}};
use nannou::prelude::*;
pub struct FluxHud;

impl FluxHud {
    pub fn draw(app: &App, draw: Draw, map: &FluxMap, config: &FluxConfig, stats: &FluxStatsManager, time_manager: &FluxTimeManager) {
        draw.text(&format!("{} - {}", map.artist, map.song_name))
            .color(WHITE).
            y(config.misc.play_area_height + 20.0)
            .font_size(25)
            .width(app.window_rect().w() as f32);
        
        
        draw.rect()
            .width(config.misc.play_area_width as f32)
            .height(config.misc.play_area_height as f32)
            .x_y(0.0, 0.0)
            .no_fill()
            .stroke(WHITE)
            .stroke_weight(2.0);
        
        draw.text(&format!("Notes:\n{}/{}", stats.notes_hit, stats.note_total))
            .width(config.misc.play_area_width as f32)
            .x(config.misc.play_area_width + 40.0)
            .left_justify()
            .y(-(config.misc.play_area_height / 2.0) + 100.0)
            .color(WHITE)
            .font_size(30);
        
        draw.text(&format!("Accuracy:\n{:.2}%", (stats.notes_hit as f32 / stats.note_total as f32) * 100.0))
            .width(config.misc.play_area_width as f32)
            .x(config.misc.play_area_width + 40.0)
            .left_justify()
            .y(-(config.misc.play_area_height / 2.0) + 200.0)
            .color(WHITE)
            .font_size(30);
        
        draw.text(&format!("Misses: {}", stats.notes_missed))
            .width(config.misc.play_area_width as f32)
            .x(config.misc.play_area_width + 40.0)
            .left_justify()
            .y(-(config.misc.play_area_height / 2.0) + 300.0)
            .color(WHITE)
            .font_size(30);

        if time_manager.paused {
            draw.text("PAUSED")
                .color(RED)
                .font_size(50)
                .width(app.window_rect().w());
        }
    }
}