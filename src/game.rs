use std::{io::Cursor, thread, sync::mpsc::{self, Sender}, time::UNIX_EPOCH, process::exit};

use nannou::prelude::*;

use crate::{maploader::FluxMap, WIDTH, HEIGHT};


pub const PLAY_AREA_SIZE_DIVIDER: f32 = 2.5;
pub const PLAY_AREA_WIDTH: f32 = WIDTH as f32 / PLAY_AREA_SIZE_DIVIDER;
pub const PLAY_AREA_HEIGHT: f32 = PLAY_AREA_WIDTH;

#[derive(Clone, PartialEq)]
pub struct FluxNote {
    x: f32,
    y: f32,
    z: f32,
    ms: u32,
}

pub struct FluxConfig {
    pub ar: f32,
    pub sd: f32,
    pub offset: i64,
}

pub struct FluxGame {
    map: FluxMap,
    config: FluxConfig,
    audio_sender: Sender<Vec<u8>>,
    currentms: i64,
    startms: i64,
    notes: Vec<FluxNote>,
    note_index: usize,
    current_notes: Vec<FluxNote>,
}

impl FluxGame {
    pub fn new(config: FluxConfig) -> Self {
        let (s, r) = mpsc::channel();

        thread::spawn(move || {
            let mp3_data: Vec<u8> = r.recv().unwrap();
            let cursor = Cursor::new(mp3_data);
            let (_stream, handle) = rodio::OutputStream::try_default().unwrap();

            let sink = rodio::Sink::try_new(&handle).unwrap();

            sink.set_volume(0.05);
            sink.append(rodio::decoder::Decoder::new_mp3(cursor).unwrap());
    
            sink.sleep_until_end();
        });
        Self {
            map: FluxMap::empty(),
            audio_sender: s,
            currentms: 0,
            startms: 0,
            notes: vec![],
            note_index: 0,
            config,
            current_notes: vec![],
        }
    }

    pub fn draw_before_loaded_map(&self, draw: Draw) {
        draw.background().color(BLACK);
        draw.text("Drag and drop map to play").color(WHITE).font_size(30).width(WIDTH as f32);

    }

    pub fn update_notes(&mut self, dt: u128) {
        let n = dt as f32 / 1000000000.0;
        let mut remove: i32 = -1;
        for (i, mut note) in self.current_notes.iter_mut().enumerate() {
            note.z -= self.config.ar as f32 * n;
            
            if note.z <= 1.0 {
                remove = i as i32;
            }
        }
        if remove != -1 {
            self.current_notes.remove(remove as usize);
        }
            
    }

    pub fn insert_map(&mut self, map: FluxMap) {
        self.map = map;
        for (i, v) in self.map.map_data.split(",").enumerate() {
            if i < 1 { 
                continue; // ignore roblox map id
            }
            let v1 = v.replace("\r", "").replace("\n", "");
            let note_data: Vec<&str> = v1.split("|").collect();
            self.notes.push(FluxNote{
                x: note_data[0].parse::<f32>().expect("Failed to parse note x"), 
                y: note_data[1].parse::<f32>().expect("Failed to parse note y"), 
                z: self.config.sd,
                ms: note_data[2].parse::<u32>().expect(&format!("Failed to parse note ms: {}", note_data[2]))
            });
        }
    }

    pub fn play_map_audio(&mut self) {
        self.audio_sender.send(self.map.mp3_data.clone()).unwrap();
        self.startms = std::time::SystemTime::now().duration_since(UNIX_EPOCH).expect("Time traveler?").as_millis() as i64 + self.config.offset;
    }

    pub fn update_curernt_ms(&mut self) {
        let current_time = std::time::SystemTime::now().duration_since(UNIX_EPOCH).expect("Time traveler?").as_millis() as i64;
        let b4 = self.currentms;
        self.currentms = current_time - self.startms;
        if b4 == self.currentms {
            return;
        }
        // println!("currentms: {}", self.currentms);
    }

    pub fn update_note_index(&mut self) {
        if self.currentms > self.notes[self.note_index].ms as i64 - (self.config.sd * 10.0) as i64 {
            self.current_notes.push(self.notes[self.note_index].clone());
            if self.note_index + 1 >= self.notes.len() {
                return;
            }
            self.note_index += 1;
            return;
        }
    }

    pub fn draw_play_game(&self, draw: Draw) {
        draw.background().color(BLACK);
        // draw play area
        draw.rect().width(PLAY_AREA_WIDTH as f32).height(PLAY_AREA_HEIGHT as f32).x_y(0.0, 0.0).no_fill().stroke(WHITE).stroke_weight(2.0);
        draw.text(&format!("{} - {}", self.map.artist, self.map.song_name)).color(WHITE).y(HEIGHT as f32 / 2.2).font_size(25).width(WIDTH as f32);

        for (i, note) in self.current_notes.iter().enumerate() {
            draw.rect()
                .width((PLAY_AREA_WIDTH/3.1) / note.z)
                .height((PLAY_AREA_HEIGHT/3.1) / note.z)
                .x((note.x - 1.0) * ((PLAY_AREA_WIDTH/3.0) / (note.z)))
                .y((note.y - 1.0) * ((PLAY_AREA_HEIGHT/3.0) / (note.z)))
                .stroke(CYAN)
                .stroke_weight(1.0 / (note.z / 7.0))
                .no_fill();
            draw.text(&format!("Note index: {}, Currentms: {}, Note ms: {}, x: {}, y: {}, z: {:.1}", self.note_index, self.currentms, note.ms, note.x, note.y, note.z)).color(WHITE).x(-450.0).y(-(HEIGHT as f32 / 2.5) + (10.0 * i as f32)).width(WIDTH as f32);
        }

    }
}