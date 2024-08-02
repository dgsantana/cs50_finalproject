mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use components::{Block, PieceType};

use crate::state::{AppState, GameState};

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TetrisSet {
    // The piece is spawned
    Spawn,
    // The piece is moved
    Movement,
    // The piece is checked for collisions, removed lines, and game over
    Collision,
    // The piece visibility is controlled to avoid rendering it when it is out of the screen
    Visibility,
}

pub struct TetrisPiecePlugin;

impl Plugin for TetrisPiecePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Block>()
            .register_type::<PieceType>()
            .configure_sets(
                Update,
                (
                    TetrisSet::Spawn,
                    TetrisSet::Movement,
                    TetrisSet::Collision,
                    TetrisSet::Visibility,
                )
                    .chain(),
            )
            .add_systems(
                OnEnter(AppState::GameState),
                (systems::setup_game, systems::clear_pieces),
            )
            .add_systems(
                Update,
                systems::add_piece
                    .in_set(TetrisSet::Spawn)
                    .run_if(in_state(GameState::Play)),
            )
            .add_systems(
                Update,
                (systems::rotate_piece, systems::move_piece)
                    .chain()
                    .in_set(TetrisSet::Movement)
                    .run_if(in_state(GameState::Play)),
            )
            .add_systems(
                Update,
                (
                    systems::collisions_check,
                    systems::remove_lines,
                    systems::game_over_check,
                )
                    .chain()
                    .in_set(TetrisSet::Collision)
                    .run_if(in_state(GameState::Play)),
            )
            .add_systems(
                Update,
                systems::visibility_control
                    .in_set(TetrisSet::Visibility)
                    .run_if(in_state(AppState::GameState)),
            );
    }
}
