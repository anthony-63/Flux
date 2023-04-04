mod maploader;
mod game;
mod cursor;

use std::path::Path;

use game::{FluxGame, FluxConfig};
use maploader::FluxMaploader;
use nannou::prelude::*;
use nannou_egui::{Egui, egui::{self, Button}};

const TITLE: &'static str = "Flux | ALPHA v0.1";
pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 720;
pub const CURSOR_PATH: &'static str = "data/cursor.png";
pub const MAP_DIR: &'static str = "data/maps";
fn main() {
    nannou::app(model).update(update).loop_mode(LoopMode::RefreshSync).run();
}

#[derive(PartialEq)]
enum FluxState {
    LoadMaps,
    MapMenu,
    PlayMap,
}

pub const SETTINGS: FluxConfig = FluxConfig {
    ar: 10.0,
    ad: 55.0,
    fs: 10,
    cursor_size: 60.0,
    offset: 0,
    noteset: "data/notesets/default",
};

pub struct Model {
    window: window::Id,
    game: FluxGame,
    state: FluxState,
    captured: bool,
    maps: Vec<String>,
    map_gui: Egui,
}

fn model(app: &App) -> Model {
    let window = 
        app.new_window()
        .resizable(false)
        .dropped_file(dropped_file)
        .view(view)
        .title(TITLE)
        .size(WIDTH, HEIGHT)
        .raw_event(raw_window_event)
        .key_pressed(key_pressed)
        .mouse_moved(mouse_moved)
        .build()
        .unwrap();
    let w = app.window(window).unwrap();
    Model {
        window,
        game: FluxGame::new(app, SETTINGS),
        state: FluxState::LoadMaps,
        captured: false,
        map_gui:  Egui::from_window(&w),
        maps: vec![],
    }
}

fn dropped_file(_app: &App, model: &mut Model, path: std::path::PathBuf) {
    if model.state != FluxState::MapMenu {
        return;
    }
    let map = FluxMaploader::load_map(path.into_os_string().into_string().unwrap());
    model.state = FluxState::PlayMap;
    model.game.insert_map(map);
    model.game.play_map_audio();
}

fn mouse_moved(_app: &App, model: &mut Model, mp: Point2) {
    if model.captured {
        model.game.cursor.cursor_move(mp);
    }
}

fn key_pressed(app: &App, model: &mut Model, keycode: Key) {
    match keycode {
        Key::Tab => {
            model.captured = !model.captured;
            let w = app.window(model.window).unwrap();
            w.set_cursor_grab(model.captured).unwrap();
            w.set_cursor_visible(!model.captured);
        },
        Key::Q => {
            model.game.stop_audio();
            model.state = FluxState::MapMenu;
            model.game = FluxGame::new(app, SETTINGS);
            model.game.load_noteset(app);
        },
        _ => {}, 
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.map_gui.handle_raw_event(event);
}

fn update(app: &App, model: &mut Model, update: Update) {
    if model.state == FluxState::LoadMaps {
        for e in Path::new(MAP_DIR).read_dir().unwrap() {
            let path = e.unwrap().path();
            if let Some(ext) = path.extension() {
                if ext == "flux" {
                    model.maps.push(String::from(path.to_str().unwrap()));
                }
            }
        }
        println!("All maps: ");
        for i in model.maps.clone().into_iter() {
            println!("{}", i);
        }
        model.game.load_noteset(app);
        model.state = FluxState::MapMenu
    }
    if model.state == FluxState::MapMenu {
        let gui = &mut model.map_gui;
        gui.set_elapsed_time(update.since_start);
        let ctx = gui.begin_frame();
        egui::Window::new("Map list")
            .fixed_pos(egui::Pos2::new(0.0, 0.0))
            .default_size(egui::Vec2::new(WIDTH as f32, HEIGHT as f32))
            .scroll2([false, true])
            .title_bar(false)
            .resizable(false)
            .show(&ctx, |ui| {

            ui.label("maps:");
            
            for i in model.maps.clone().into_iter() {

                if ui.add(Button::new(i.clone())).clicked() {
                    let map = FluxMaploader::load_map(i.clone());
                    model.state = FluxState::PlayMap;
                    model.game.insert_map(map);
                    model.game.play_map_audio();
                }
            }
        });

    }
    if model.state != FluxState::PlayMap {
        return;
    }

    model.game.update_curernt_ms();
    model.game.update_note_index();
    model.game.update_notes();
    model.game.cursor.lock_cursor_to_play_area();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    match model.state {
        FluxState::LoadMaps => { draw.text("Loading maps").font_size(50).width(WIDTH as f32); },
        FluxState::MapMenu => model.game.draw_before_loaded_map(draw.clone()),
        FluxState::PlayMap => model.game.draw_play_game(draw.clone()),
    };
    // draw.text(&format!("{:.2} FPS", app.fps())).right_justify().y((HEIGHT as f32 / 2.0) - 10.0).color(YELLOW).font_size(20).width(WIDTH as f32);
    draw.to_frame(app, &frame).unwrap();
    if model.state == FluxState::MapMenu {
        model.map_gui.draw_to_frame(&frame).unwrap();
    }
}
