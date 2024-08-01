mod common;
mod grid;
mod ui;
mod piece;
mod state;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, grid::setup)
        .run();
}
