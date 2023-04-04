mod maploader;
mod game;
mod cursor;

use game::{FluxGame, FluxConfig};
use maploader::FluxMaploader;
use nannou::prelude::*;
use nannou_egui::{Egui, egui::{self, Button, DragValue}};

const TITLE: &'static str = "Flux | ALPHA v0.1";
pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 720;
pub const CURSOR_PATH: &'static str = "data/cursor.png";
pub const MAP_DIR: &'static str = "data/maps";
pub const NOTESETS_DIR: &'static str = "data/notesets";
fn main() {
    nannou::app(model).update(update).loop_mode(LoopMode::RefreshSync).run();
}

#[derive(PartialEq)]
enum FluxState {
    InitGame,
    MapMenu,
    PlayMap,
}

pub const DEFAULT_SETTINGS: FluxConfig = FluxConfig {
    ar: 20.0,
    ad: 10.0,
    fs: 10,
    sens: 1.0,
    volume: 1.0,
    cursor_size: 60.0,
    offset: 0,
    noteset: "rounded",
};

pub struct Model {
    window: window::Id,
    game: FluxGame,
    state: FluxState,
    captured: bool,
    maps: Vec<String>,
    menu_gui: Egui,
    notesets: Vec<String>,
    show_settings: bool,
    settings: FluxConfig,
    selected_noteset: String,
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
        game: FluxGame::new(app, DEFAULT_SETTINGS),
        state: FluxState::InitGame,
        captured: false,
        menu_gui:  Egui::from_window(&w),
        maps: vec![],
        notesets: vec![],
        settings: DEFAULT_SETTINGS,
        show_settings: false,
        selected_noteset: String::from(""),
    }
}

fn dropped_file(_app: &App, model: &mut Model, path: std::path::PathBuf) {
    // if model.state != FluxState::MapMenu {
    //     return;
    // }
    // let map = FluxMaploader::load_map(path.into_os_string().into_string().unwrap());
    // model.state = FluxState::PlayMap;
    // model.game.insert_map(map);
    // model.game.play_map_audio();
}

fn mouse_moved(_app: &App, model: &mut Model, mp: Point2) {
    if model.captured {
        model.game.cursor.cursor_move(mp, model.settings.sens);
    }
}

fn key_pressed(app: &App, model: &mut Model, keycode: Key) {
    match keycode {
        Key::Tab => {
            if model.state == FluxState::PlayMap {
                model.captured = !model.captured;
            }
            let w = app.window(model.window).unwrap();
            w.set_cursor_grab(model.captured).unwrap();
            w.set_cursor_visible(!model.captured);
        },
        Key::Q => {
            model.game.stop_audio();
            model.state = FluxState::MapMenu;
            model.game.reset();
            model.captured = false;
            let w = app.window(model.window).unwrap();
            w.set_cursor_grab(model.captured).unwrap();
            w.set_cursor_visible(!model.captured);
        },
        Key::S => {
            model.show_settings = !model.show_settings;
        }
        _ => {}, 
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.menu_gui.handle_raw_event(event);
}

fn update(app: &App, model: &mut Model, update: Update) {
    if model.state == FluxState::InitGame {
        for file in std::fs::read_dir(MAP_DIR).unwrap() {
            let path = file.unwrap().path();
            if let Some(ext) = path.extension() {
                if ext == "flux" {
                    model.maps.push(String::from(path.to_str().unwrap()));
                }
            }
        }
        for file in std::fs::read_dir(NOTESETS_DIR).unwrap() {
            let path = file.unwrap().path();
            if path.is_dir() {
                model.notesets.push(String::from(path.as_os_str().to_str().unwrap()));
            }
        }
        println!("All maps: ");
        for i in model.maps.clone().into_iter() {
            println!("{}", i);
        }
        println!("All notesets: ");
        for i in model.notesets.clone().into_iter() {
            println!("{}", i);
        }
        model.selected_noteset = model.notesets[0].clone();
        model.game.load_noteset(app, &model.selected_noteset);
        model.state = FluxState::MapMenu
    }
    if model.state == FluxState::MapMenu {
        let gui = &mut model.menu_gui;
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
                    model.captured = true;
                    let w = app.window(model.window).unwrap();
                    w.set_cursor_grab(model.captured).unwrap();
                    w.set_cursor_visible(!model.captured);
                }
            }
        });

        if model.show_settings {
            egui::Window::new("Settings").resizable(false).show(&ctx, |ui| {
                ui.label("AR: ");
                if ui.add(DragValue::new(&mut model.settings.ar).speed(0.1).clamp_range(0.0..=500.0)).changed() {
                    model.game.reload_settings(model.settings.clone());
                }
                ui.end_row();
                ui.label("AD: ");
                if ui.add(DragValue::new(&mut model.settings.ad).speed(0.1).clamp_range(0.0..=500.0)).changed() {
                    model.game.reload_settings(model.settings.clone());
                }
                ui.end_row();
                ui.label("Volume: ");
                if ui.add(DragValue::new(&mut model.settings.volume).clamp_range(0.0..=10.0).speed(0.01)).changed() {
                    model.game.set_volume(model.settings.volume);
                }
                ui.end_row();
                ui.label("SENS: ");
                ui.add(DragValue::new(&mut model.settings.sens).speed(0.1));
                ui.end_row();
                ui.label("noteset: ");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", model.selected_noteset))
                    .show_ui(ui, |ui| {
                        for  v in model.notesets.clone().into_iter() {
                            ui.selectable_value(&mut model.selected_noteset, v.clone(), v);
                        }
                    });
                if ui.button("Load selected noteset").clicked() {
                    model.game.load_noteset(app, &model.selected_noteset);
                }
            });
        }

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
        FluxState::InitGame => { draw.text("Loading maps").font_size(50).width(WIDTH as f32); },
        FluxState::MapMenu => model.game.draw_before_loaded_map(draw.clone()),
        FluxState::PlayMap => model.game.draw_play_game(draw.clone()),
    };
    // draw.text(&format!("{:.2} FPS", app.fps())).right_justify().y((HEIGHT as f32 / 2.0) - 10.0).color(YELLOW).font_size(20).width(WIDTH as f32);
    draw.to_frame(app, &frame).unwrap();
    if model.state == FluxState::MapMenu {
        model.menu_gui.draw_to_frame(&frame).unwrap();
    }
}
