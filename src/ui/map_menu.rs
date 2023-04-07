use std::path::Path;

use nannou::App;
use nannou_egui::{egui::{self, Button}, FrameCtx};

use crate::{core::{maploader::FluxMaploader, constants::MAP_DIR}, FluxState, Model};

pub struct FluxMapMenuUI {
    maps: Vec<String>,
    map_search: String,
}

impl FluxMapMenuUI {
    pub fn new() -> Self {
        Self {
            maps: vec![],
            map_search: String::from(""),
        }
    }

    pub fn init(&mut self) {
        for file in std::fs::read_dir(MAP_DIR).expect("Failed to read from maps directory") {
            let path = file.unwrap().path();
            if let Some(ext) = path.extension() {
                if ext == "flux" {
                    self.maps.push(String::from(path.to_str().unwrap()));
                }
            }
        }
    }

    pub fn render(&mut self, app: &App, model: &mut Model, ctx: FrameCtx) {
        egui::Window::new("Map list")
            .fixed_pos(egui::Pos2::new(0.0, 0.0))
            .default_size(egui::Vec2::new(app.window_rect().w(), app.window_rect().h()))
            .scroll2([false, true])
            .title_bar(false)
            .resizable(false)
            .show(&ctx, |ui| {

            ui.horizontal(|ui| {
                ui.label("Search: ");
                ui.text_edit_singleline(&mut self.map_search);
            });

            ui.label("maps:");
            for i in self.maps.clone().into_iter() {
                let mut contains: Vec<bool> = vec![];
                for s in self.map_search.clone().as_str().split(" ").into_iter() {
                    if !s.is_empty() {
                        contains.push(i.contains(s));
                    }
                }
                if contains.contains(&false) && !self.map_search.is_empty() {
                    continue;
                }
                if ui.add(Button::new(Path::new(&i.clone()).file_name().unwrap().to_str().unwrap().to_string())).clicked() {
                    let map = FluxMaploader::load_map(i);
                    model.state = FluxState::PlayMap;
                    model.update_rpc = true;
                    model.game.insert_map(map);
                    model.game.start_audio();
                    model.captured = true;
                    let w = app.window(model.window).unwrap();

                    w.set_cursor_visible(!model.captured);
                }
            }
        });
    } 
}