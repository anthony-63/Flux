mod sets;
mod core;
mod managers;
mod ui;

use crate::core::constants::*;
use crate::core::config::{FluxConfig, FluxNoteConfig, FluxCursorConfig, FluxAudioConfig, FluxMiscConfig, FluxSetsConfig};
use std::path::Path;

use discord_rich_presence::{DiscordIpcClient, DiscordIpc, activity::{self}};
use ui::map_menu::FluxMapMenuUI;
use ui::settings::FluxSettingsUI;
use crate::core::game::{FluxGame};
use log::LevelFilter;
use log4rs::{append::file::FileAppender, encode::pattern::PatternEncoder, Config, config::{Appender, Root}};
use crate::core::maploader::FluxMaploader;
use nannou::prelude::*;
use nannou_egui::{Egui, egui::{self, Button, FontDefinitions}};


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

pub const DEFAULT_CONFIG: FluxConfig = FluxConfig {
    note: FluxNoteConfig {
        ar: 10.0,
        ad: 6.0,
        fs: 10,
        hitbox: 1.14,
        approach_time: 0.0,
    },
    cursor: FluxCursorConfig {
        sens: 0.9,
        size: 70.0,
        edge_buffer: 20.0,
    },
    audio: FluxAudioConfig {
        volume: 1.0,
        speed: 1.0,
        offset: 0,
    },
    sets: FluxSetsConfig {
        note: "rounded",
        cursor: "default",
        hit: "thump",
    },
    misc: FluxMiscConfig {
        play_area_width: 0.0,
        play_area_height: 0.0,
        debug: false,
    }
};

pub struct Model {
    window: window::Id,
    game: FluxGame,
    rpc: DiscordIpcClient,
    state: FluxState,
    captured: bool,
    gui: Egui,
    update_rpc: bool,
    settings_ui: FluxSettingsUI,
    map_menu_ui: FluxMapMenuUI,
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
        .build()
        .unwrap();
    let w = app.window(window).unwrap();
    let mut rpc = DiscordIpcClient::new("1093315881104310352").unwrap();
    rpc.connect().unwrap();

    Model {
        window,
        game: FluxGame::new(app, DEFAULT_CONFIG),
        state: FluxState::InitGame,
        captured: false,
        gui:  Egui::from_window(&w),
        rpc,
        update_rpc: false,
        settings_ui: FluxSettingsUI::new(),
        map_menu_ui: FluxMapMenuUI::new(),
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
            model.game.audio_manager.pause_song();
            model.state = FluxState::MapMenu;
            model.game.reset();
            model.captured = false;
            let w = app.window(model.window).unwrap();
            w.set_cursor_visible(!model.captured);
            model.update_rpc = true;
        },
        Key::F1 => {
            model.settings_ui.show_settings = !model.settings_ui.show_settings;
        },
        Key::Space => {
            if model.state != FluxState::PlayMap 
            || model.game.time_manager.pause_timer.current_ms < 1000 {
                return;
            }
            model.game.time_manager.paused = !model.game.time_manager.paused;
            if model.game.time_manager.paused {
                model.game.pause_game();
            }
            if !model.game.time_manager.paused {
                model.game.play_game();
            }
        },
        _ => {}, 
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.gui.handle_raw_event(event);
}


fn update(app: &App, model: &mut Model, update: Update) {
    if model.state == FluxState::InitGame {
        model.settings_ui.clone().init(app, model);

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

        
        model.gui.set_elapsed_time(update.since_start);
        let ctx = model.gui.begin_frame();
        ctx.set_fonts(fonts);
        
        model.settings_ui.clone().render(app, model, ctx);
    }
    if model.state != FluxState::PlayMap {
        return;
    }

    if model.game.time_manager.song_timer.current_ms % 2500 == 0 {
        model.update_rpc = true;
    }

    if model.update_rpc {
        let artist =String::from_utf8_lossy(model.game.map.meta.get("artist").unwrap());
        let title = String::from_utf8_lossy(model.game.map.meta.get("song_name").unwrap());

        let details = format!("{} - {}", 
                artist, 
                title);

        let state = format!("{}:{:02} - {:.02}% - {} Misses - {:.02}x", 
            ((model.game.time_manager.song_timer.current_ms / 1000) / 60), 
            (model.game.time_manager.song_timer.current_ms / 1000) % 60, 
            (model.game.stats_manager.notes_hit as f32 / model.game.stats_manager.note_total as f32) * 100.0, 
            model.game.stats_manager.note_total,
            model.settings_ui.config.audio.speed);

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
    if model.captured {
        // w.set_cursor_position_points(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0).unwrap();
        model.game.cursor.cursor_move(Point2::new(app.mouse.x, app.mouse.y), model.settings_ui.config.cursor.sens);
    }
    model.game.time_manager.update();
    model.game.update_notes(app);
    if model.captured {
        model.game.cursor.lock_real_cursor_to_play_area(app, model.window, model.settings_ui.config.cursor.sens, model.settings_ui.config.cursor.edge_buffer, model.settings_ui.config.misc.play_area_width, model.settings_ui.config.misc.play_area_height);
        model.game.cursor.lock_cursor_to_play_area(model.settings_ui.config.misc.play_area_width, model.settings_ui.config.misc.play_area_height);
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
        model.gui.draw_to_frame(&frame).unwrap();
    }
}
