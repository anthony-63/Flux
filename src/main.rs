mod maploader;
mod game;
mod cursor;

use std::path::Path;

use discord_rich_presence::{DiscordIpcClient, DiscordIpc, activity::{self}};
use game::{FluxGame, FluxConfig};
use log::LevelFilter;
use log4rs::{append::file::FileAppender, encode::pattern::PatternEncoder, Config, config::{Appender, Root}};
use maploader::FluxMaploader;
use nannou::prelude::*;
use nannou_egui::{Egui, egui::{self, Button, DragValue, FontDefinitions}};

const TITLE: &'static str = "Flux | ALPHA v0.1";
pub const MAP_DIR: &'static str = "data/maps";
pub const NOTESETS_DIR: &'static str = "data/notesets";
pub const HITSETS_DIR: &'static str = "data/hitsets";
pub const CURSORSETS_DIR: &'static str = "data/cursorsets";
pub const LOG_FILE: &'static str = "data/flux.log.txt";
fn main() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .append(false)
        .build(LOG_FILE).unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
        .appender("logfile")
        .build(LevelFilter::Warn)).unwrap();
    log4rs::init_config(config).unwrap();
    log_panics::init();
    nannou::app(model).update(update).loop_mode(LoopMode::RefreshSync).run();
}

#[derive(PartialEq)]
enum FluxState {
    InitGame,
    MapMenu,
    PlayMap,
}

pub const DEFAULT_SETTINGS: FluxConfig = FluxConfig {
    ar: 10.0,
    ad: 6.0,
    fs: 10,
    sens: 0.9,
    volume: 1.0,
    speed: 1.0,
    edge_buffer: 20.0,
    cursor_size: 70.0,
    offset: 0,
    hitbox: 1.14,
    noteset: "rounded",
    cursorset: "default",
    hitset: "thump",
};

pub struct Model {
    window: window::Id,
    game: FluxGame,
    rpc: DiscordIpcClient,
    state: FluxState,
    captured: bool,
    maps: Vec<String>,
    menu_gui: Egui,
    notesets: Vec<String>,
    hitsets: Vec<String>,
    cursorsets: Vec<String>,
    show_settings: bool,
    settings: FluxConfig,
    selected_noteset: String,
    selected_cursorset: String,
    selected_hitset: String,
    lmx: Point2,
    map_search: String,
    update_rpc: bool,
}

fn model(app: &App) -> Model {
    let window = 
        app.new_window()
        .resizable(false)
        .dropped_file(dropped_file)
        .view(view)
        .title(TITLE)
        .fullscreen()
        .raw_event(raw_window_event)
        .key_pressed(key_pressed)
        .mouse_moved(mouse_moved)
        .build()
        .unwrap();
    let w = app.window(window).unwrap();
    let mut rpc = DiscordIpcClient::new("1093315881104310352").unwrap();
    rpc.connect().unwrap();

    Model {
        window,
        game: FluxGame::new(app, DEFAULT_SETTINGS),
        state: FluxState::InitGame,
        captured: false,
        menu_gui:  Egui::from_window(&w),
        maps: vec![],
        map_search: String::from(""),
        notesets: vec![],
        rpc,
        update_rpc: false,
        cursorsets: vec![],
        hitsets: vec![],
        settings: DEFAULT_SETTINGS,
        show_settings: true,
        selected_noteset: String::from(""),
        selected_cursorset: String::from(""),
        selected_hitset: String::from(""),
        lmx: Point2::new(0.0, 0.0),
    }
}

fn dropped_file(_app: &App, _model: &mut Model, _path: std::path::PathBuf) {
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
        // w.set_cursor_position_points(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0).unwrap();
        model.game.cursor.cursor_move(mp, model.settings.sens);
        model.lmx = mp;
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
        Key::Back => {
            model.game.pause_audio();
            model.state = FluxState::MapMenu;
            model.game.reset();
            model.captured = false;
            let w = app.window(model.window).unwrap();
            w.set_cursor_visible(!model.captured);
            model.update_rpc = true;
        },
        Key::F1 => {
            model.show_settings = !model.show_settings;
        },
        Key::Space => {
            if model.state != FluxState::PlayMap || model.game.unpaused_ms < 1000 {
                return;
            }
            model.game.paused = !model.game.paused;
            if model.game.paused {
                model.game.pause_game();
            }
            if !model.game.paused {
                model.game.play_game();
            }
        },
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
        for file in std::fs::read_dir(CURSORSETS_DIR).unwrap() {
            let path = file.unwrap().path();
            if path.is_dir() {
                model.cursorsets.push(String::from(path.as_os_str().to_str().unwrap()));
            }
        }

        for file in std::fs::read_dir(HITSETS_DIR).unwrap() {
            let path = file.unwrap().path();
            if path.is_dir() {
                model.hitsets.push(String::from(path.as_os_str().to_str().unwrap()));
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
        println!("All cursorsets: ");
        for i in model.cursorsets.clone().into_iter() {
            println!("{}", i);
        }
        println!("All hitsets: ");
        for i in model.hitsets.clone().into_iter() {
            println!("{}", i);
        }
        model.selected_noteset = model.notesets[0].clone();
        model.selected_hitset = model.hitsets[0].clone();
        model.selected_cursorset = model.cursorsets[0].clone();
        model.game.load_noteset(app, &model.selected_noteset);
        model.game.load_hitset(&model.selected_hitset);
        model.game.load_cursorset(app, &model.selected_cursorset);
        model.state = FluxState::MapMenu;
        model.update_rpc = true;
    }
    if model.state == FluxState::MapMenu {
        if model.update_rpc {
            let payload = activity::Activity::new()
                .details("In map menu")
                .state("Choosing map")
                .buttons(vec![discord_rich_presence::activity::Button::new("Join Flux", "https://discord.gg/C4fgSxabQt")])
                .assets(
                    activity::Assets::new()
                        .large_image("flux"));
            model.rpc.set_activity(payload).unwrap();
            model.update_rpc = false;
        }
        let mut fonts = FontDefinitions::default();
        fonts.family_and_size.insert(egui::TextStyle::Body, (egui::FontFamily::Proportional, 22.0));
        fonts.family_and_size.insert(egui::TextStyle::Button, (egui::FontFamily::Proportional, 22.0));
        fonts.family_and_size.insert(egui::TextStyle::Monospace, (egui::FontFamily::Proportional, 22.0));
        fonts.family_and_size.insert(egui::TextStyle::Small, (egui::FontFamily::Proportional, 22.0));

        let gui = &mut model.menu_gui;
        gui.set_elapsed_time(update.since_start);
        let ctx = gui.begin_frame();
        ctx.set_fonts(fonts);

        if model.show_settings {
            egui::Window::new("Settings").resizable(false).show(&ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("AR: ");
                    if ui.add(DragValue::new(&mut model.settings.ar).speed(0.1).clamp_range(0.0..=500.0)).changed() {
                        model.game.reload_settings(model.settings.clone());
                    }
                }); 

                ui.horizontal(|ui| {
                    ui.label("AD: ");
                    if ui.add(DragValue::new(&mut model.settings.ad).speed(0.1).clamp_range(0.0..=500.0)).changed() {
                        model.game.reload_settings(model.settings.clone());
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Cursor Size: ");
                    if ui.add(DragValue::new(&mut model.settings.cursor_size).speed(0.1).clamp_range(0.0..=500.0)).changed() {
                        model.game.cursor.change_cursor_size(model.settings.cursor_size);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Volume: ");
                    if ui.add(DragValue::new(&mut model.settings.volume).clamp_range(0.0..=10.0).speed(0.01)).changed() {
                        model.game.set_volume(model.settings.volume);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Sensitivity: ");
                    ui.add(DragValue::new(&mut model.settings.sens).speed(0.1));
                });

                ui.horizontal(|ui| {
                    ui.label("Speed: ");
                    if ui.add(DragValue::new(&mut model.settings.speed).speed(0.01).clamp_range(0.0..=10.0)).changed() {
                        model.game.set_speed(model.settings.clone().speed);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Noteset: ");
                    egui::ComboBox::from_label(" ")
                        .selected_text(format!("{:?}", Path::new(&model.selected_noteset).file_name().unwrap().to_str().unwrap().to_string()))
                        .show_ui(ui, |ui| {
                            for  v in model.notesets.clone().into_iter() {
                                ui.selectable_value(&mut model.selected_noteset, v.clone(), Path::new(&v).into_iter().last().unwrap().to_str().unwrap().to_string());
                            }
                        });
                    if ui.button("Load selected noteset").clicked() {
                        model.game.load_noteset(app, &model.selected_noteset);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Hitset: ");
                    egui::ComboBox::from_label("  ")
                        .selected_text(format!("{:?}", Path::new(&model.selected_hitset).file_name().unwrap().to_str().unwrap().to_string()))
                        .show_ui(ui, |ui| {
                            for  v in model.hitsets.clone().into_iter() {
                                ui.selectable_value(&mut model.selected_hitset, v.clone(), Path::new(&v).into_iter().last().unwrap().to_str().unwrap().to_string());
                            }
                        });
                    if ui.button("Load selected hitset").clicked() {
                        model.game.load_hitset(&model.selected_hitset);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Cursorset: ");
                    egui::ComboBox::from_label("   ")
                        .selected_text(format!("{:?}", Path::new(&model.selected_cursorset).file_name().unwrap().to_str().unwrap().to_string()))
                        .show_ui(ui, |ui| {
                            for  v in model.cursorsets.clone().into_iter() {
                                ui.selectable_value(&mut model.selected_noteset, v.clone(), Path::new(&v).into_iter().last().unwrap().to_str().unwrap().to_string());
                            }
                        });
                    if ui.button("Load selected cursorset").clicked() {
                        model.game.load_cursorset(app, &model.selected_cursorset);
                    }
                });
            });
        }

        egui::Window::new("Map list")
            .fixed_pos(egui::Pos2::new(0.0, 0.0))
            .default_size(egui::Vec2::new(app.window_rect().w(), app.window_rect().h()))
            .scroll2([false, true])
            .title_bar(false)
            .resizable(false)
            .show(&ctx, |ui| {

            ui.horizontal(|ui| {
                ui.label("Search: ");
                ui.text_edit_singleline(&mut model.map_search);
            });

            ui.label("maps:");
            for i in model.maps.clone().into_iter() {
                let mut contains: Vec<bool> = vec![];
                for s in model.map_search.clone().as_str().split(" ").into_iter() {
                    if !s.is_empty() {
                        contains.push(i.contains(s));
                    }
                }
                if contains.contains(&false) && !model.map_search.is_empty() {
                    continue;
                }
                if ui.add(Button::new(Path::new(&i.clone()).file_name().unwrap().to_str().unwrap().to_string())).clicked() {
                    let map = FluxMaploader::load_map(i);
                    model.state = FluxState::PlayMap;
                    model.update_rpc = true;
                    model.game.insert_map(map);
                    model.game.play_map_audio();
                    model.captured = true;
                    let w = app.window(model.window).unwrap();

                    w.set_cursor_visible(!model.captured);
                }
            }
        });
    }
    if model.state != FluxState::PlayMap {
        return;
    }

    if model.game.currentms % 2500 == 0 {
        model.update_rpc = true;
    }

    if model.update_rpc {
        let details = format!("{} - {}", 
                model.game.map.artist, 
                model.game.map.song_name);

        let state = format!("{}:{:02} - {:.02}% - {} Misses - {:.02}x", 
            ((model.game.currentms / 1000) / 60), 
            (model.game.currentms / 1000) % 60, 
            (model.game.hit as f32 / model.game.removed as f32) * 100.0, 
            model.game.missed,
            model.settings.speed);

        let payload = activity::Activity::new()
            .details(&details)
            .state(&state)
            .buttons(vec![discord_rich_presence::activity::Button::new("Join Flux", "https://discord.gg/C4fgSxabQt")])
            .assets(
                activity::Assets::new()
                    .large_image("flux"));
        model.rpc.set_activity(payload).unwrap();
        model.update_rpc = false;
    }

    model.game.update_ms();
    model.game.update_note_index();
    model.game.update_notes();
    if model.captured {
        model.game.cursor.lock_real_cursor_to_play_area(app, model.window, model.settings.sens, model.settings.edge_buffer, model.game.play_area_width, model.game.play_area_height);
        model.game.cursor.lock_cursor_to_play_area(model.game.play_area_width, model.game.play_area_height);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    match model.state {
        FluxState::InitGame => { draw.text("Loading content").font_size(50).width(app.window_rect().w() as f32); },
        FluxState::MapMenu => model.game.draw_before_loaded_map(draw.clone()),
        FluxState::PlayMap => model.game.draw_play_game(app, draw.clone()),
    };
    // draw.text(&format!("{:.2} FPS", app.fps())).right_justify().y((HEIGHT as f32 / 2.0) - 10.0).color(YELLOW).font_size(20).width(WIDTH as f32);
    draw.to_frame(app, &frame).unwrap();
    if model.state == FluxState::MapMenu {
        model.menu_gui.draw_to_frame(&frame).unwrap();
    }
}
