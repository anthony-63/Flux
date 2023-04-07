use std::path::PathBuf;

use nannou::prelude::*;

use crate::{managers::{time_manager::FluxTimeManager, notes_manager::FluxNotesManager, audio_manager::FluxAudioManager, stats_manager::FluxStatsManager}, sets::{hitset::FluxHitset, cursorset::FluxCursorset, noteset::FluxNoteset}, ui::hud::FluxHud, Model};

use super::{cursor::FluxCursor, maploader::FluxMap, config::FluxConfig};
use super::constants::*;

pub struct FluxGame {
    pub map: FluxMap,
    pub config: FluxConfig,
    pub audio_manager: FluxAudioManager,
    pub notes_manager: FluxNotesManager,
    pub time_manager: FluxTimeManager,
    pub stats_manager: FluxStatsManager,
    pub cursorset: FluxCursorset,
    pub hitset: FluxHitset,
    pub noteset: FluxNoteset,
    pub cursor: FluxCursor,
}

impl FluxGame {
    pub fn new(app: &App, config: FluxConfig) -> Self {
        Self {
            map: FluxMap::empty(),
            audio_manager: FluxAudioManager::new(),
            notes_manager: FluxNotesManager::new(),
            time_manager: FluxTimeManager::new(config.clone().audio.speed, config.clone().audio.offset),
            stats_manager: FluxStatsManager::new(),
            hitset: FluxHitset::new(),
            cursorset: FluxCursorset::new(),
            noteset: FluxNoteset::new(),
            cursor: FluxCursor::new(config.clone().cursor.size),
            config,
        }
    }

    pub fn reset(&mut self) {
        self.time_manager.reset();
        self.audio_manager.reset();
        self.notes_manager.reset();
    }

    pub fn update_notes(&mut self, app: &App) {
        self.notes_manager.move_notes(
            app, 
            &mut self.stats_manager, 
            &self.time_manager, 
            &self.config, 
            &mut self.audio_manager, 
            &mut self.hitset, 
            &mut self.cursorset)
    }

    pub fn new_config(&mut self, config: FluxConfig) {
        self.config = config;
    }
    
    pub fn pause_game(&mut self) {
        self.audio_manager.pause_song();
        self.time_manager.pause_timer.start();
    }

    pub fn play_game(&mut self) {
        self.time_manager.update_paused_total();
        self.audio_manager.resume_song();
        self.time_manager.unpause_timer.start();
    }

    pub fn insert_map(&mut self, map: FluxMap) {
        self.map = map;
        self.notes_manager.load_maps(
            &self.config, 
            &self.map,
            &self.noteset, 
            &self.hitset, 
            &self.cursorset)
    }

    pub fn start_audio(&mut self) {
        self.audio_manager.play_song(&self.map, &self.config);
        self.time_manager.song_timer.start();
        self.time_manager.unpause_timer.start();
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.config.audio.speed = speed;
    }

    pub fn draw_before_loaded_map(&self, _draw: Draw) {
    }
    
    pub fn draw_play_game(&self, app: &App, draw: Draw) {
        FluxHud::draw(
            app, 
            draw.clone(), 
            &self.map, 
            &self.config, 
            &self.stats_manager, 
            &self.time_manager);

        self.notes_manager.clone().render(
            app, 
            draw.clone(), 
            &self.noteset, 
            &self.config, 
            &self.time_manager, 
            &self.notes_manager);
        
        self.cursor.draw(draw.clone(), self.cursorset.index);
    }
}