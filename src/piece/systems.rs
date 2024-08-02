use bevy::prelude::*;

use crate::{common::VISIBILITY_LIMIT_Y, state::GameState};

use super::{
    components::{Block, Movable, PieceType},
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
    mut query: Query<(&mut Block, &mut Transform), With<PieceType>>,
) {
    if timer.0.tick(time.delta()).finished() {
        for (mut block, mut transform) in query.iter_mut() {
            // Check if the block can move down
            if block.y() > 0 && q_blocks.iter().all(|b| b.y() != block.y() + 1) {
                block.move_down();
                transform.translation = block.as_translation();
            }
        }
    }
}

pub fn manual_move(
    time: Res<Time>,
    q_static_blocks: Query<&Block, Without<PieceType>>,
    mut q_moveable_blocks: Query<(&mut Block, &mut Transform, &PieceType), With<PieceType>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut manual_timer: ResMut<ManualMoveTimer>,
    mut auto_timer: ResMut<MoveDownTimer>,
) {
    if manual_timer.0.tick(time.delta()).just_finished() {
        // Update collisions we can detect but we need to keep the piece look
        let blocks = q_moveable_blocks
            .iter()
            .map(|(b, _, _)| *b)
            .collect::<Vec<_>>();
        let moveable = valid_move(&blocks, &q_static_blocks);
        let rotate_blocks = blocks
            .iter()
            .map(|b| {
                let mut b = *b;
                b.rotate();
                b
            })
            .collect::<Vec<_>>();
        let can_rotate = !q_moveable_blocks
            .iter()
            .any(|(_, _, piece_type)| matches!(piece_type, PieceType::O))
            && valid_move(&rotate_blocks, &q_static_blocks).can_rotate();

        for (mut block, mut transform, _) in q_moveable_blocks.iter_mut() {
            if keyboard_input.pressed(KeyCode::ArrowLeft) && moveable.can_move_left() {
                block.move_left();
                transform.translation = block.as_translation();
            } else if keyboard_input.pressed(KeyCode::ArrowRight) && moveable.can_move_right() {
                block.move_right();
                transform.translation = block.as_translation();
            } else if keyboard_input.pressed(KeyCode::ArrowDown) && moveable.can_move_down() {
                block.move_down();
                transform.translation = block.as_translation();
                auto_timer.0.reset();
            } else if keyboard_input.just_released(KeyCode::ArrowUp) && can_rotate {
                block.rotate();
                transform.translation = block.as_translation();
            }
        }
    }
}

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

pub fn remove_lines(
    mut commands: Commands,
    mut q_blocks: Query<(Entity, &mut Block, &mut Transform), Without<PieceType>>,
) {
    let mut lines = [0; 20];
    for (_, block, _) in q_blocks.iter() {
        lines[block.y() as usize] += 1;
    }

    let mut ignore_lines = Vec::new();
    for (entity, block, _) in q_blocks.iter() {
        if lines[block.y() as usize] == 10 {
            ignore_lines.push(entity);
            commands.entity(entity).despawn_recursive();
        }
    }

    // Move lines down
    for y in (1..20).rev() {
        if lines[y] == 10 {
            for (entity, mut block, mut transform) in q_blocks.iter_mut() {
                if ignore_lines.contains(&entity) {
                    continue;
                }
                if block.y() < y as i32 {
                    block.move_down();
                    transform.translation = block.as_translation();
                }
            }
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
