use std::collections::BTreeSet;

use bevy::prelude::*;

use crate::{
    common::VISIBILITY_LIMIT_Y,
    state::GameState,
    stats::{NextPieceEvent, Score, ScoreEvent},
};

use super::{
    components::{Block, Movable, Piece, PieceType},
    resources::{ManualMoveTimer, MoveDownTimer, PiecesQueue},
};

/// System to setup the pieces queue at the start of the game
pub fn setup_game(mut commands: Commands) {
    commands.insert_resource(PiecesQueue::new());
    commands.insert_resource(MoveDownTimer(Timer::from_seconds(
        1.0,
        TimerMode::Repeating,
    )));
    commands.insert_resource(ManualMoveTimer(Timer::from_seconds(
        0.05,
        TimerMode::Repeating,
    )));
}

/// System to add a new piece to the game when
/// the current one is gone or at the start of the game
pub fn add_piece(
    mut commands: Commands,
    query: Query<&PieceType>,
    mut pieces: ResMut<PiecesQueue>,
    mut next_piece_event: EventWriter<NextPieceEvent>,
) {
    if query.is_empty() {
        pieces.next().build(&mut commands);
        let Some(next) = pieces.peek() else {
            return;
        };
        next_piece_event.send(NextPieceEvent(*next));
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

pub fn move_piece(
    time: Res<Time>,
    q_static_blocks: Query<&Block, Without<PieceType>>,
    mut q_moveable_blocks: Query<(&mut Block, &mut Transform, &PieceType), With<PieceType>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut manual_timer: ResMut<ManualMoveTimer>,
    mut auto_timer: ResMut<MoveDownTimer>,
) {
    let manual = manual_timer.0.tick(time.delta()).just_finished();
    let auto = auto_timer.0.tick(time.delta()).just_finished();

    // We only calculate collisions if we are moving the piece
    if auto || manual {
        // Update collisions we can detect but we need to keep the piece look
        let blocks = q_moveable_blocks
            .iter()
            .map(|(b, _, _)| *b)
            .collect::<Vec<_>>();
        let mut moveable = valid_move(&blocks, &q_static_blocks);

        // If is auto move, we only move down and ignore the rest
        let mut move_down = false;
        if auto && moveable.can_move_down() {
            move_down = true;
            for (mut block, mut transform, _) in q_moveable_blocks.iter_mut() {
                block.move_down();
                transform.translation = block.as_board_translation();
            }
            // Update collisions
            let blocks = q_moveable_blocks
                .iter()
                .map(|(b, _, _)| *b)
                .collect::<Vec<_>>();
            moveable = valid_move(&blocks, &q_static_blocks);
        }

        for (mut block, mut transform, _) in q_moveable_blocks.iter_mut() {
            let mut moved = false;
            if keyboard_input.pressed(KeyCode::ArrowLeft) && moveable.can_move_left() {
                block.move_left();
                moved = true;
            } else if keyboard_input.pressed(KeyCode::ArrowRight) && moveable.can_move_right() {
                block.move_right();
                moved = true;
            } else if keyboard_input.pressed(KeyCode::ArrowDown)
                && moveable.can_move_down()
                && !move_down
            {
                block.move_down();
                moved = true;
            }

            if moved {
                transform.translation = block.as_board_translation();
            }
        }
    }
}

/// System to rotate the piece
pub fn rotate_piece(
    q_static_blocks: Query<&Block, Without<PieceType>>,
    mut q_moveable_blocks: Query<(&mut Block, &mut Transform, &PieceType), With<PieceType>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_released(KeyCode::ArrowUp) {
        // Update collisions we can detect but we need to keep the piece look
        let blocks = q_moveable_blocks
            .iter()
            .map(|(b, _, _)| *b)
            .collect::<Vec<_>>();
        let Some(piece_type) = q_moveable_blocks.iter().next().map(|(_, _, p)| p) else {
            return;
        };
        let piece = Piece::from_array(&blocks, *piece_type);
        let rotate_blocks = piece.rotate_blocks();
        let can_rotate = valid_rotation(&rotate_blocks, &q_static_blocks).can_rotate();

        for (mut block, mut transform, _) in q_moveable_blocks.iter_mut() {
            if can_rotate {
                // Use our helper to do the rotation.
                piece.rotate_block(&mut block);
                transform.translation = block.as_board_translation();
            }
        }
    }
}

/// Helper function to check if the piece can move.
fn valid_move(blocks: &[Block], q_static_blocks: &Query<&Block, Without<PieceType>>) -> Movable {
    let mut moveable = Movable::new();
    for block in blocks.iter() {
        if block.y() == 0
            || q_static_blocks
                .iter()
                .any(|b| b.y() == block.y() - 1 && b.x() == block.x())
        {
            moveable.down = false;
        }
        if block.x() == 0
            || q_static_blocks
                .iter()
                .any(|b| b.x() == block.x() - 1 && b.y() == block.y())
        {
            moveable.left = false;
        }
        if block.x() == 9
            || q_static_blocks
                .iter()
                .any(|b| b.x() == block.x() + 1 && b.y() == block.y())
        {
            moveable.right = false;
        }
    }
    moveable
}

fn valid_rotation(
    blocks: &[Block],
    q_static_blocks: &Query<&Block, Without<PieceType>>,
) -> Movable {
    let mut moveable = Movable::new();
    for block in blocks.iter() {
        if block.y() < 0
            || q_static_blocks
                .iter()
                .any(|b| b.y() == block.y() - 1 && b.x() == block.x())
        {
            moveable.down = false;
        }
        if block.x() < 0
            || q_static_blocks
                .iter()
                .any(|b| b.x() == block.x() - 1 && b.y() == block.y())
        {
            moveable.left = false;
        }
        if block.x() > 9
            || q_static_blocks
                .iter()
                .any(|b| b.x() == block.x() + 1 && b.y() == block.y())
        {
            moveable.right = false;
        }
    }
    moveable
}

/// System to check if the piece has collided with the bottom or another piece
/// and remove the PieceType component to make it static.
pub fn collisions_check(
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
        if block.y() == 0
            || q_blocks
                .iter()
                .any(|b| b.y() == block.y() - 1 && b.x() == block.x())
        {
            stop = true;
            break;
        }
    }

    if stop {
        for (entity, _) in query.iter() {
            commands.entity(entity).remove::<PieceType>();
        }
    }
}

/// System to remove the lines that are full
pub fn remove_lines(
    mut commands: Commands,
    mut q_blocks: Query<(Entity, &mut Block, &mut Transform), Without<PieceType>>,
    mut score_event: EventWriter<ScoreEvent>,
) {
    let mut lines = [0; 20];
    for (_, block, _) in q_blocks.iter() {
        // Ignore blocks that are out of the board
        if block.y() < 0 || block.y() >= 20 {
            continue;
        }
        lines[block.y() as usize] += 1;
    }

    // We use a BTreeSet to keep the lines removed sorted and unique
    let mut removed_lines = BTreeSet::new();
    for (entity, block, _) in q_blocks.iter() {
        // Ignore blocks that are out of the board
        if block.y() < 0 || block.y() >= 20 {
            continue;
        }
        if lines[block.y() as usize] == 10 {
            removed_lines.insert(block.y());
            commands.entity(entity).despawn_recursive();
        }
    }

    if removed_lines.is_empty() {
        return;
    }

    let lines = removed_lines.len() as u64;
    let score = match lines {
        1 => 40,
        2 => 100,
        3 => 300,
        4 => 1200,
        _ => 0,
    };

    score_event.send(ScoreEvent(Score {
        value: score,
        lines,
    }));

    // Move blocks above the removed lines down
    for (_, mut block, mut transform) in q_blocks.iter_mut() {
        let y = block.y();
        let mut offset = 0;
        for &removed_y in &removed_lines {
            if y > removed_y {
                offset += 1;
            }
        }
        if offset > 0 {
            block.shift_y(-offset);
            transform.translation = block.as_board_translation();
        }
    }
}

pub fn game_over_check(
    q_blocks: Query<&Block, Without<PieceType>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if q_blocks.iter().any(|b| b.y() >= VISIBILITY_LIMIT_Y) {
        state.set(GameState::GameOver);
    }
}

/// System to clear the pieces from the board
pub fn clear_pieces(mut commands: Commands, query: Query<Entity, With<Block>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<PiecesQueue>();
    commands.remove_resource::<MoveDownTimer>();
    commands.remove_resource::<ManualMoveTimer>();
}
