use std::{time::UNIX_EPOCH, path::{PathBuf, Path}, io::Cursor};

use kira::{manager::{AudioManager, backend::cpal::CpalBackend, AudioManagerSettings}, sound::static_sound::{StaticSoundData, StaticSoundSettings}, tween::Tween};
use nannou::prelude::*;

use crate::{maploader::FluxMap, WIDTH, HEIGHT, cursor::FluxCursor, CURSOR_PATH};


pub const PLAY_AREA_SIZE_DIVIDER: f32 = 2.5;
pub const PLAY_AREA_WIDTH: f32 = WIDTH as f32 / PLAY_AREA_SIZE_DIVIDER;
pub const PLAY_AREA_HEIGHT: f32 = PLAY_AREA_WIDTH;

#[derive(Clone, PartialEq)]
pub struct FluxNote {
    x: f32,
    y: f32,
    z: f32,
    i: u32,
    ti: usize,
    st: u64,
    ms: u64,
}

#[derive(Clone)]
pub struct FluxConfig {
    pub ar: f32, // approach rate
    pub ad: f32, // approach distance
    pub sens: f32,
    pub volume: f64,
    pub noteset: &'static str,
    pub cursor_size: f32, // cursor size
    pub fs: u32, // fade steps TODO: Implement fade
    pub offset: i64,
}

pub struct FluxGame {
    map: FluxMap,
    config: FluxConfig,
    currentms: u64,
    startms: u64,
    approach_time: f64,
    audio_manager: AudioManager,
    notes: Vec<FluxNote>,
    note_index: usize,
    noteset_textures: Vec<wgpu::Texture>,
    current_notes: Vec<FluxNote>,
    pub cursor: FluxCursor,
}

impl FluxGame {
    pub fn new(app: &App, config: FluxConfig) -> Self {
        Self {
            map: FluxMap::empty(),
            currentms: 0,
            startms: 0,
            notes: vec![],
            note_index: 0,
            audio_manager: AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).expect("Failed to create audio manager"),
            config: config.clone(),
            approach_time: (config.ad as f64 / config.ar as f64) * 1000.0,
            current_notes: vec![],
            noteset_textures: vec![],
            cursor: FluxCursor::new(app, config.cursor_size,PathBuf::from(CURSOR_PATH)),
        }
    }

    pub fn reset(&mut self) {
        let config = self.config.clone();
        self.map = FluxMap::empty();
        self.currentms = 0;
        self.startms = 0;
        self.notes = vec![];
        self.note_index = 0;
        self.audio_manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).expect("Failed to create audio manager");
        self.config = config;
        self.current_notes = vec![];
    }

    pub fn reload_settings(&mut self, config: FluxConfig) {
        self.config = config;
        self.approach_time = (self.config.ad as f64 / self.config.ar as f64) * 1000.0;
    }

    pub fn update_notes(&mut self) {
        let mut remove: i32 = -1;
        for (i, mut note) in self.current_notes.iter_mut().enumerate() {
            let st: f64 = note.ms as f64 - self.approach_time;
            let t: f64 = (note.ms as i64 - self.currentms as i64) as f64 / (note.ms as f64 - st);
            note.z = t as f32 * self.config.ad;
            if note.z <= 1.0 {
                remove = i as i32;
            }
        }
        if remove != -1 {
            self.current_notes.remove(remove as usize);
        }
    }

    pub fn load_noteset(&mut self, app: &App, noteset: &str) {
        self.noteset_textures = vec![];
        for e in Path::new(noteset).read_dir().unwrap() {
            let path = e.unwrap().path();
            if let Some(ext) = path.extension() {
                if ext == "png" {
                    println!("what da tuna? {:?}", path);
                    let tex = wgpu::Texture::from_path(app, String::from(path.to_str().unwrap())).unwrap();
                    self.noteset_textures.push(tex);
                }
            }
        }
    }

    pub fn set_volume(&mut self, vol: f64) {
        self.audio_manager.main_track().set_volume(vol, Tween {
            ..Default::default()
        }).unwrap();
    }

    pub fn stop_audio(&mut self) {
        self.audio_manager.pause(Tween {
            ..Default::default()
        }).unwrap();
    }

    pub fn insert_map(&mut self, map: FluxMap) {
        self.map = map;
        for (i, v) in self.map.map_data.split(",").enumerate() {
            if i < 1 { 
                continue; // ignore roblox map id
            }
            let v1 = v.replace("\r", "").replace("\n", "");
            let note_data: Vec<&str> = v1.split("|").collect();
            let note = FluxNote{
                x: note_data[0].parse::<f32>().expect("Failed to parse note x"), 
                y: note_data[1].parse::<f32>().expect("Failed to parse note y"), 
                z: 300.0,
                i: i as u32,
                st: (note_data[2].parse::<u64>().expect("Failed to parse note st") as f64 - self.approach_time as f64) as u64,
                ti: i % self.noteset_textures.len(),
                ms: note_data[2].parse::<u64>().expect("Failed to parse note ms"),
            };

            self.notes.push(note);
            // var time = (note.time-current_time)/(note.time-spawn_time)
        }
    }

    pub fn play_map_audio(&mut self) {
        let cursor = Cursor::new(self.map.mp3_data.clone());
        let sound_data = StaticSoundData::from_cursor(cursor, StaticSoundSettings::default()).expect("Failed to create sound data");
        self.audio_manager.play(sound_data.clone()).unwrap();
        self.startms = (std::time::SystemTime::now().duration_since(UNIX_EPOCH).expect("Time traveler?").as_millis() as i64 + self.config.offset) as u64;
    }

    pub fn update_curernt_ms(&mut self) {
        let current_time = std::time::SystemTime::now().duration_since(UNIX_EPOCH).expect("Time traveler?").as_millis() as u64;
        let b4 = self.currentms;
        self.currentms = current_time - self.startms;
        if b4 == self.currentms {
            return;
        }
        // println!("currentms: {}", self.currentms);
    }

    pub fn update_note_index(&mut self) {
        
        // println!("{} {}", self.currentms, self.notes[self.note_index].st);
        if self.currentms < self.notes[self.note_index].st {
            return;
        }
        if self.note_index + 1 >= self.notes.len() {
            return;
        }
        self.current_notes.push(self.notes[self.note_index].clone());

        self.note_index += 1;
        return;
    }

    pub fn draw_before_loaded_map(&self, draw: Draw) {
    }

    pub fn draw_play_game(&self, draw: Draw) {
        // draw play area
        draw.rect().width(PLAY_AREA_WIDTH as f32).height(PLAY_AREA_HEIGHT as f32).x_y(0.0, 0.0).no_fill().stroke(WHITE).stroke_weight(2.0);
        draw.text(&format!("{} - {}", self.map.artist, self.map.song_name)).color(WHITE).y(HEIGHT as f32 / 2.2).font_size(25).width(WIDTH as f32);

        for (i, note) in self.current_notes.iter().rev().enumerate() {
            draw.texture(&self.noteset_textures[note.ti])
                .width((PLAY_AREA_WIDTH/3.0) / note.z)
                .height((PLAY_AREA_HEIGHT/3.0) / note.z)
                .x((-note.x + 1.0) * ((PLAY_AREA_WIDTH/3.0) / (note.z)))
                .y((note.y - 1.0) * ((PLAY_AREA_HEIGHT/3.0) / (note.z)));

            draw.text(
                &format!("i: {}, a: {}, ms: {}, nms: {}, x: {}, y: {}, z: {:.1}, st: {:.2}", self.note_index, note.i, self.currentms, note.ms, note.x, note.y, note.z, note.st))
                .color(WHITE)
                .x(10.0)
                .y(-(HEIGHT as f32 / 2.5) + (10.0 * i as f32))
                .width(WIDTH as f32)
                .left_justify();
        }
        self.cursor.draw(draw.clone());

    }
}