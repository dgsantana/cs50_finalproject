use bevy::prelude::*;

use crate::common::{BLOCK_SIZE, BOARD_COLS, BOARD_ROWS, BORDER_COLOR, BORDER_SIZE};

pub fn setup(mut commands: Commands) {
    let half_x = BOARD_COLS as f32 / 2.0;
    let half_y = BOARD_ROWS as f32 / 2.0;
    // Lots of trial and error to get the cup to look right
    let side = half_x * BLOCK_SIZE + BORDER_SIZE;
    let bottom = half_y * BLOCK_SIZE + BORDER_SIZE;
    let size_y = BOARD_ROWS as f32 * BLOCK_SIZE + 2.0 * BORDER_SIZE;
    let size_x = BOARD_COLS as f32 * BLOCK_SIZE + 2.0 * BORDER_SIZE;
    // Draw the cup

    // Draw the left
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(-side, 0.0, 0.0).with_scale(Vec3::new(
            BORDER_SIZE,
            size_y,
            0.0,
        )),
        sprite: Sprite {
            color: BORDER_COLOR,
            ..default()
        },
        ..default()
    });
    // Draw the right
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(side, 0.0, 0.0).with_scale(Vec3::new(
            BORDER_SIZE,
            size_y,
            0.0,
        )),
        sprite: Sprite {
            color: BORDER_COLOR,
            ..default()
        },
        ..default()
    });
    // Draw the bottom
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0.0, -bottom, 0.0).with_scale(Vec3::new(
            size_x,
            BORDER_SIZE,
            0.0,
        )),
        sprite: Sprite {
            color: BORDER_COLOR,
            ..default()
        },
        ..default()
    });
}
