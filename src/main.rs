mod common;
mod grid;
mod piece;
mod state;
mod ui;

use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use piece::TetrisPiecePlugin;
use state::{AppState, GameState};
use ui::TetrisUIPlugin;

/// This is our entry point for the game
fn main() {
    // Create a new App
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    #[cfg(debug_assertions)]
    app.add_plugins(WorldInspectorPlugin::default());
    app.init_state::<AppState>()
        .add_sub_state::<GameState>()
        .enable_state_scoped_entities::<AppState>()
        .enable_state_scoped_entities::<GameState>()
        .add_plugins((TetrisUIPlugin, TetrisPiecePlugin))
        .add_systems(Startup, (setup_camera, grid::setup))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
