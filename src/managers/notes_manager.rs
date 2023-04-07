use nannou::prelude::*;

use crate::{core::{config::FluxConfig, maploader::FluxMap, constants::MAX_AR_AD}, sets::{noteset::FluxNoteset, hitset::FluxHitset, cursorset::FluxCursorset}};

use super::{stats_manager::FluxStatsManager, audio_manager::FluxAudioManager, time_manager::FluxTimeManager};

#[derive(Clone, PartialEq)]
pub struct FluxNote {
    x: f32,
    y: f32,
    z: f32,
    index: u32,
    noteset_index: usize,
    hitset_index: usize,
    cursorset_index: usize,
    spawn_time: u64,
    ms: u64,
    hitsound_played: bool,
}

#[derive(Clone)]
pub struct FluxNotesManager {
    notes: Vec<FluxNote>,
    notes_to_render: Vec<FluxNote>,
    index: usize,
}

impl FluxNotesManager {
    pub fn new() -> Self {
        Self {
            notes: vec![],
            notes_to_render: vec![],
            index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.notes = vec![];
        self.notes_to_render = vec![];
        self.index = 0;
    }

    pub fn load_maps(&mut self, config: &FluxConfig, map: &FluxMap, noteset: &FluxNoteset, hitset: &FluxHitset, cursorset: &FluxCursorset) {
        for (i, note) in map.map_data.split(",").enumerate() {
            if i < 1 {
                continue; // ignore roblox id
            }

            let note_nnl = note.replace("\r", "").replace("\n", "");
            let note_data: Vec<&str> = note_nnl.split("|").collect();
            
            let note_ms: u64 = note_data[2].parse::<u64>().expect("Invalid map data.");
            
            self.notes.push(FluxNote {
                x: note_data[0].parse::<f32>().expect("Invalid map data."),
                y: note_data[1].parse::<f32>().expect("Invalid map data."),
                z: MAX_AR_AD as f32,
                ms: note_ms,
                index: i as u32,
                spawn_time: (note_ms as f32 - (config.note.approach_time * config.audio.speed as f32)) as u64,
                noteset_index: i % noteset.textures.len(),
                hitset_index: i % hitset.sounds.len(),
                cursorset_index: i % cursorset.textures.len(),
                hitsound_played: false,
            });
        }
    }

    pub fn update_index(&mut self, time_manager: FluxTimeManager) {
        if time_manager.paused {
            return;
        }
        
        if time_manager.song_timer.current_ms < self.notes[self.index].spawn_time {
            return;
        }

        if self.index + 1 >= self.notes.len() {
            return;
        }

        self.notes_to_render.push(self.notes[self.index].clone());

        self.index += 1;
        return;
    }

    pub fn move_notes(&mut self, app: &App, stats_manager: &mut FluxStatsManager, time_manager: &FluxTimeManager, config: &FluxConfig, audio_manager: &mut FluxAudioManager, hitset: &mut FluxHitset, cursorset: &mut FluxCursorset) {
        let mut remove: i32 = -1;
        let mut hit = false;

        for (i, mut note) in self.notes_to_render.iter_mut().enumerate() {
            let st = note.ms as f32 - config.note.approach_time;
            let t = (note.ms as i64 - time_manager.song_timer.current_ms as i64) as f32 / (note.ms as f32 - st);
            note.z = t as f32 * config.note.ad;
            if note.z < 0.5 {
                remove = i as i32;
            }
            if note.z <= 1.0 && !note.hitsound_played {
                audio_manager.play_hitsound(hitset);
                note.hitsound_played = true;
            }
            if note.z >= 0.5 && note.z <= 1.0 {
                if Rect::from_xy_wh(
                    Vec2::new((-note.x + 1.0) * ((config.misc.play_area_width/3.0)), (note.y - 1.0) * ((config.misc.play_area_height/3.0))),
                    Vec2::new((config.misc.play_area_width/3.0) * config.note.hitbox, (config.misc.play_area_height/3.0) * config.note.hitbox)
                ).contains(Vec2::new(app.mouse.x, app.mouse.y)) {
                    remove = i as i32;
                    hit = true;
                }
            }
        }
        if remove != -1 {
            if hit {
                stats_manager.notes_hit += 1;
            } else {
                stats_manager.notes_missed += 1;
            }
            cursorset.index = self.notes_to_render[remove as usize].cursorset_index;
            hitset.index = self.notes_to_render[remove as usize].hitset_index;
            stats_manager.note_total += 1;
            self.notes_to_render.remove(remove as usize);
        }
    }
    
    pub fn render(&mut self, app: &App, draw: Draw, noteset: &FluxNoteset, config: &FluxConfig, time_manager: &FluxTimeManager, notes_manager: &FluxNotesManager) {
        for (i, note) in self.notes_to_render.iter().rev().enumerate() {
            draw.texture(&noteset.textures[note.noteset_index])
                .width((config.misc.play_area_width/3.0) / note.z)
                .height((config.misc.play_area_height/3.0) / note.z)
                .x((-note.x + 1.0) * ((config.misc.play_area_width/3.0) / (note.z)))
                .y((note.y - 1.0) * ((config.misc.play_area_height/3.0) / (note.z)));

            if config.misc.debug {
                draw.text(
                    &format!("i: {}, a: {}, ms: {}, nms: {}, x: {}, y: {}, z: {:.1}, st: {:.2}", 
                        notes_manager.index, 
                        note.index, 
                        time_manager.song_timer.current_ms, 
                        note.ms, 
                        note.x, 
                        note.y, 
                        note.z, 
                        note.spawn_time))
                    .color(WHITE)
                    .x(10.0)
                    .y(-(app.window_rect().h() as f32 / 2.5) + (10.0 * i as f32))
                    .width(app.window_rect().w() as f32)
                    .left_justify();
            }
        }
    }
}