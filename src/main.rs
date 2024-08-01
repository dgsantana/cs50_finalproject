mod common;
mod grid;
mod piece;
mod state;
mod ui;

use bevy::prelude::*;
use state::{AppState, GameState};

fn main() {
    let mut app = App::new();
    app.init_state::<AppState>()
        .add_sub_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, grid::setup)
        .run();
}
