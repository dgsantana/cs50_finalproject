use bevy::prelude::*;

// Tetris game constants
pub const BOARD_COLS: usize = 10;
pub const BOARD_ROWS: usize = 20;
pub const BOARD_CENTER_X: f32 = BOARD_COLS as f32 / 2.0;
pub const BOARD_CENTER_Y: f32 = BOARD_ROWS as f32 / 2.0;
pub const BLOCK_SIZE: f32 = 30.0;
pub const BLOCK_SPRITE_SIZE: f32 = 28.0;
pub const BORDER_SIZE: f32 = 5.0;
pub const BORDER_COLOR: Color = Color::WHITE;
pub const CACHED_PIECES: usize = 7;
/// The number of rows that are visible to the top of the cup
pub const VISIBILITY_LIMIT_Y: i32 = 21;