mod maploader;
mod game;

use game::FluxGame;
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

pub struct Model {
    _window: window::Id,
    game: FluxGame,
    state: FluxState,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().dropped_file(dropped_file).view(view).title(TITLE).size(WIDTH, HEIGHT).build().unwrap();
    Model {
        _window,
        game: FluxGame::new(),
        state: FluxState::WaitForMap,
    }
}

fn dropped_file(_app: &App, model: &mut Model, path: std::path::PathBuf) {
    if model.state != FluxState::WaitForMap {
        return;
    }
    let map = FluxMaploader::load_map(path.into_os_string().into_string().unwrap());
    model.game.insert_map(map);
    model.state = FluxState::PlayMap;
    model.game.play_map_audio();
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    match model.state {
        FluxState::WaitForMap => model.game.draw_before_loaded_map(draw.clone()),
        FluxState::PlayMap => model.game.draw_play_game(draw.clone()),
    };
    draw.to_frame(app, &frame).unwrap();
}
