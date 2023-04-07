use std::path::{Path, PathBuf};

use nannou::App;
use nannou_egui::{egui::{self, DragValue}, FrameCtx};

use crate::{core::{config::FluxConfig, constants::{NOTESETS_DIR, CURSORSETS_DIR, HITSETS_DIR}}, DEFAULT_CONFIG, Model};

#[derive(Clone)]
pub struct FluxSettingsUI {
    pub selected_noteset: String,
    pub selected_cursorset: String,
    pub selected_hitset: String,
    pub show_settings: bool,
    pub notesets: Vec<String>,
    pub hitsets: Vec<String>,
    pub cursorsets: Vec<String>,
    pub config: FluxConfig,
}

impl FluxSettingsUI {
    pub fn new() -> Self {
        let mut config = DEFAULT_CONFIG;
        config.note.approach_time = config.note.ad / config.note.ar;
        Self {
            config,
            show_settings: false,
            selected_noteset: String::from(""),
            selected_cursorset: String::from(""),
            selected_hitset: String::from(""),
            cursorsets: vec![],
            notesets: vec![],
            hitsets: vec![],
        }
    }

    pub fn init(&mut self, app: &App, model: &mut Model) {
        for file in std::fs::read_dir(NOTESETS_DIR).expect("Failed to read from notesets directory") {
            let path = file.unwrap().path();
            if path.is_dir() {
                self.notesets.push(String::from(path.as_os_str().to_str().unwrap()));
            }
        }
        for file in std::fs::read_dir(CURSORSETS_DIR).expect("Failed to read from cursorsets directory") {
            let path = file.unwrap().path();
            if path.is_dir() {
                self.cursorsets.push(String::from(path.as_os_str().to_str().unwrap()));
            }
        }

        for file in std::fs::read_dir(HITSETS_DIR).expect("Failed to read from hitsets directory") {
            let path = file.unwrap().path();
            if path.is_dir() {
                self.hitsets.push(String::from(path.as_os_str().to_str().unwrap()));
            }
        }

        self.selected_noteset = model.settings_ui.notesets[0].clone();
        self.selected_hitset = model.settings_ui.hitsets[0].clone();
        self.selected_cursorset = model.settings_ui.cursorsets[0].clone();
        model.game.noteset.load_from_path(app, PathBuf::from(model.settings_ui.selected_noteset.clone()));
        model.game.hitset.load_from_path(PathBuf::from(model.settings_ui.selected_hitset.clone()));
        model.game.cursorset.load_from_path(app, PathBuf::from(model.settings_ui.selected_cursorset.clone()));
    }

    pub fn render(&mut self, app: &App, model: &mut Model, ctx: FrameCtx) {
        if self.show_settings {
            egui::Window::new("Settings").resizable(false).show(&ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("AR: ");
                    if ui.add(DragValue::new(&mut self.config.note.ar).speed(0.1).clamp_range(0.0..=500.0)).changed() {
                        model.game.new_config(self.config.clone());
                    }
                }); 

                ui.horizontal(|ui| {
                    ui.label("AD: ");
                    if ui.add(DragValue::new(&mut self.config.note.ad).speed(0.1).clamp_range(0.0..=500.0)).changed() {
                        model.game.new_config(self.config.clone());
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Cursor Size: ");
                    if ui.add(DragValue::new(&mut self.config.cursor.size).speed(0.1).clamp_range(0.0..=500.0)).changed() {
                        model.game.cursor.change_cursor_size(self.config.cursor.size);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Volume: ");
                    if ui.add(DragValue::new(&mut self.config.audio.volume).clamp_range(0.0..=10.0).speed(0.01)).changed() {
                        model.game.audio_manager.set_song_volume(self.config.audio.volume);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Sensitivity: ");
                    ui.add(DragValue::new(&mut self.config.cursor.sens).speed(0.1));
                });

                ui.horizontal(|ui| {
                    ui.label("Speed: ");
                    if ui.add(DragValue::new(&mut self.config.audio.speed).speed(0.01).clamp_range(0.0..=10.0)).changed() {
                        model.game.set_speed(self.config.audio.speed);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Noteset: ");
                    egui::ComboBox::from_label(" ")
                        .selected_text(format!("{:?}", Path::new(&self.selected_noteset).file_name().unwrap().to_str().unwrap().to_string()))
                        .show_ui(ui, |ui| {
                            for  v in self.notesets.clone().into_iter() {
                                ui.selectable_value(&mut self.selected_noteset, v.clone(), Path::new(&v).into_iter().last().unwrap().to_str().unwrap().to_string());
                            }
                        });
                    if ui.button("Load selected noteset").clicked() {
                        model.game.noteset.load_from_path(app, PathBuf::from(self.selected_noteset.clone()));
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Hitset: ");
                    egui::ComboBox::from_label("  ")
                        .selected_text(format!("{:?}", Path::new(&self.selected_hitset).file_name().unwrap().to_str().unwrap().to_string()))
                        .show_ui(ui, |ui| {
                            for  v in self.hitsets.clone().into_iter() {
                                ui.selectable_value(&mut self.selected_hitset, v.clone(), Path::new(&v).into_iter().last().unwrap().to_str().unwrap().to_string());
                            }
                        });
                    if ui.button("Load selected hitset").clicked() {
                        model.game.hitset.load_from_path(PathBuf::from(self.selected_hitset.clone()));
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Cursorset: ");
                    egui::ComboBox::from_label("   ")
                        .selected_text(format!("{:?}", Path::new(&self.selected_cursorset).file_name().unwrap().to_str().unwrap().to_string()))
                        .show_ui(ui, |ui| {
                            for  v in self.cursorsets.clone().into_iter() {
                                ui.selectable_value(&mut self.selected_noteset, v.clone(), Path::new(&v).into_iter().last().unwrap().to_str().unwrap().to_string());
                            }
                        });
                    if ui.button("Load selected cursorset").clicked() {
                        model.game.cursorset.load_from_path(app, PathBuf::from(self.selected_cursorset.clone()));
                    }
                });
            });
        }
    }
}