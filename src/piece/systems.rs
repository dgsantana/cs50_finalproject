use bevy::prelude::*;

use crate::common::VISIBILITY_LIMIT_Y;

use super::{
    components::{Block, Movable, PieceType},
    resources::{MoveDownTimer, PiecesQueue},
};

/// System to setup the pieces queue at the start of the game
pub fn setup_game(mut commands: Commands) {
    commands.insert_resource(PiecesQueue::new());
    commands.insert_resource(MoveDownTimer(Timer::from_seconds(
        1.0,
        TimerMode::Repeating,
    )));
}

/// System to add a new piece to the game when
/// the current one is gone or at the start of the game
pub fn add_piece(
    mut commands: Commands,
    query: Query<&PieceType>,
    mut pieces: ResMut<PiecesQueue>,
) {
    if query.is_empty() {
        pieces.next().build(&mut commands);
    }
}

/// System to control the visibility of the pieces
pub fn visibility_control(mut query: Query<(&Block, &mut Visibility), With<PieceType>>) {
    for (piece, mut visible) in query.iter_mut() {
        if piece.y() >= VISIBILITY_LIMIT_Y {
            *visible = Visibility::Hidden;
        } else {
            *visible = Visibility::Visible;
        }
    }
}

pub fn auto_move_down(
    time: Res<Time>,
    mut timer: ResMut<MoveDownTimer>,
    q_blocks: Query<&Block, Without<PieceType>>,
    mut query: Query<(&mut Block, &mut Transform, &Movable), With<PieceType>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (mut block, mut transform, moveable) in query.iter_mut() {
            // Check if the block can move down
            if moveable.can_move_down()
                && block.y() > 0
                && q_blocks.iter().all(|b| b.y() != block.y() + 1)
            {
                block.move_down();
                transform.translation = block.as_translation();
            }
        }
    }
}

pub fn check_collision(
    mut commands: Commands,
    q_blocks: Query<&Block, Without<PieceType>>,
    mut query: Query<(Entity, &Block), With<PieceType>>,
) {
    let mut stop = false;
    for (_, block) in query
        .iter_mut()
        .sort_by::<(Entity, &Block)>(|a, b| a.1.y().partial_cmp(&b.1.y()).unwrap())
    {
        // Check if the block can move down
        if block.y() == 0 || q_blocks.iter().any(|b| b.y() == block.y() - 1) {
            stop = true;
            break;
        }
    }

    if stop {
        for (entity, _) in query.iter() {
            commands
                .entity(entity)
                .remove::<Movable>()
                .remove::<PieceType>();
        }
    }
}

/// System to clear the pieces from the board
pub fn clear_pieces(mut commands: Commands, query: Query<Entity, With<Block>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<PiecesQueue>();
    commands.remove_resource::<MoveDownTimer>();
}
