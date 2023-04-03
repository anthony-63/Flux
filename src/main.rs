mod maploader;
mod game;

use game::{FluxGame, FluxConfig};
use maploader::FluxMaploader;
use nannou::prelude::*;

const TITLE: &str = "Flux | ALPHA v0.1";
pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 720;
fn main() {
    nannou::app(model).update(update).run();
}

#[derive(PartialEq)]
enum FluxState {
    WaitForMap,
    PlayMap,
}

pub const SETTINGS: FluxConfig = FluxConfig {
    ar: 17.0,
    sd: 10.0,
    offset: -50,
};

pub struct Model {
    _window: window::Id,
    game: FluxGame,
    state: FluxState,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().resizable(false).dropped_file(dropped_file).view(view).title(TITLE).size(WIDTH, HEIGHT).build().unwrap();
    Model {
        _window,
        game: FluxGame::new(SETTINGS),
        state: FluxState::WaitForMap,
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

fn update(_app: &App, model: &mut Model, update: Update) {
    if model.state != FluxState::PlayMap {
        return;
    }
    model.game.update_curernt_ms();
    model.game.update_note_index();
    model.game.update_notes(update.since_last.as_nanos());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    match model.state {
        FluxState::WaitForMap => model.game.draw_before_loaded_map(draw.clone()),
        FluxState::PlayMap => model.game.draw_play_game(draw.clone()),
    };
    draw.to_frame(app, &frame).unwrap();
}
