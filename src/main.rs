mod maploader;
mod game;
mod cursor;

use game::{FluxGame, FluxConfig, PLAY_AREA_WIDTH, PLAY_AREA_HEIGHT};
use maploader::FluxMaploader;
use nannou::{prelude::*, text::{FontCollection, Font}};

const TITLE: &'static str = "Flux | ALPHA v0.1";
pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 720;
pub const CURSOR_PATH: &'static str = "data/cursor.png";

fn main() {
    nannou::app(model).update(update).loop_mode(LoopMode::RefreshSync).run();
}

#[derive(PartialEq)]
enum FluxState {
    WaitForMap,
    PlayMap,
}

pub const SETTINGS: FluxConfig = FluxConfig {
    ar: 10.0,
    ad: 55.0,
    fs: 10,
    cursor_size: 60.0,
    offset: 0,
};

pub struct Model {
    window: window::Id,
    game: FluxGame,
    state: FluxState,
    captured: bool,
}

fn model(app: &App) -> Model {
    let window = 
        app.new_window()
        .resizable(false)
        .dropped_file(dropped_file)
        .view(view)
        .title(TITLE)
        .size(WIDTH, HEIGHT)
        .focused(focused)
        .unfocused(unfocused)
        .key_pressed(key_pressed)
        .mouse_moved(mouse_moved)
        .build()
        .unwrap();
    Model {
        window,
        game: FluxGame::new(app, SETTINGS),
        state: FluxState::WaitForMap,
        captured: false,
    }
}

fn dropped_file(_app: &App, model: &mut Model, path: std::path::PathBuf) {
    if model.state != FluxState::WaitForMap {
        return;
    }
    let map = FluxMaploader::load_map(path.into_os_string().into_string().unwrap());
    model.state = FluxState::PlayMap;
    model.game.insert_map(map);
    model.game.play_map_audio();
}

fn mouse_moved(app: &App, model: &mut Model, mp: Point2) {
    model.game.cursor.cursor_move(mp);
}

fn focused(app: &App, model: &mut Model) {

}

fn unfocused(app: &App, model: &mut Model) {
    
}

fn key_pressed(app: &App, model: &mut Model, keycode: Key) {
    match keycode {
        Key::Tab => {
            model.captured = !model.captured;
            let w = app.window(model.window).unwrap();
            w.set_cursor_grab(model.captured).unwrap();
            w.set_cursor_visible(!model.captured);
        },
        _ => {}, 
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    if model.state != FluxState::PlayMap {
        return;
    }

    model.game.update_curernt_ms();
    model.game.update_note_index();
    model.game.update_notes(update.since_last.as_nanos());
    model.game.cursor.lock_cursor_to_play_area(Point2::new(PLAY_AREA_WIDTH, PLAY_AREA_HEIGHT));
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    match model.state {
        FluxState::WaitForMap => model.game.draw_before_loaded_map(draw.clone()),
        FluxState::PlayMap => model.game.draw_play_game(draw.clone()),
    };
    draw.text(&format!("{:.2} FPS", app.fps())).right_justify().y((HEIGHT as f32 / 2.0) - 10.0).color(YELLOW).font_size(20).width(WIDTH as f32);
    draw.to_frame(app, &frame).unwrap();
}
