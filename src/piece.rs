use bevy::prelude::*;
use bevy::color::palettes::css::*;

use crate::common::BLOCK_SIZE;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

// Define the pieces as a 2D array of 4 2D coordinates
// x 3 4 5 6
// y---------
// 1| | | | |
// 0| | | | |
//  ---------

/// ####
const SHAPE_I: [[i32; 2]; 4] = [[3, 0], [4, 0], [5, 0], [6, 0]];
/// #
/// ###
const SHAPE_J: [[i32; 2]; 4] = [[3, 1], [3, 0], [4, 0], [5, 0]];
///   #
/// ###
const SHAPE_L: [[i32; 2]; 4] = [[3, 0], [4, 0], [5, 0], [5, 1]];

///  ##
///  ##
const SHAPE_O: [[i32; 2]; 4] = [[4, 1], [4, 0], [5, 1], [5, 0]];

///  ##
/// ##
const SHAPE_S: [[i32; 2]; 4] = [[3, 0], [4, 0], [4, 1], [5, 1]];

///  #
/// ###
const SHAPE_T: [[i32; 2]; 4] = [[3, 1], [4, 1], [4, 0], [5, 0]];

/// ##
///  ##
const SHAPE_Z: [[i32; 2]; 4] = [[3, 1], [4, 1], [4, 0], [5, 0]];

pub fn select_piece(piece_type: PieceType) -> [Block; 4] {
    // Copilot suggested the rest of the matches after the first one
    match piece_type {
        PieceType::I => SHAPE_I.map(|[x, y]| Block { x, y }),
        PieceType::J => SHAPE_J.map(|[x, y]| Block { x, y }),
        PieceType::L => SHAPE_L.map(|[x, y]| Block { x, y }),
        PieceType::O => SHAPE_O.map(|[x, y]| Block { x, y }),
        PieceType::S => SHAPE_S.map(|[x, y]| Block { x, y }),
        PieceType::T => SHAPE_T.map(|[x, y]| Block { x, y }),
        PieceType::Z => SHAPE_Z.map(|[x, y]| Block { x, y }),
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Movable {
    down: bool,
    left: bool,
    right: bool,
}

#[derive(Bundle)]
pub struct PieceBundle {
    sprite: SpriteBundle,
    block: Block,
    piece_type: PieceType,
    movable: Movable,
}

impl From<PieceType> for [[i32; 2]; 4] {
    fn from(value: PieceType) -> Self {
        match value {
            PieceType::I => SHAPE_I,
            PieceType::J => SHAPE_J,
            PieceType::L => SHAPE_L,
            PieceType::O => SHAPE_O,
            PieceType::S => SHAPE_S,
            PieceType::T => SHAPE_T,
            PieceType::Z => SHAPE_Z,
        }
    }
}

impl From<&PieceType> for Color {
    fn from(value: &PieceType) -> Self {
        match value {
            PieceType::I => LIGHT_CYAN.into(),
            PieceType::J => BLUE.into(),
            PieceType::L => ORANGE.into(),
            PieceType::O => YELLOW.into(),
            PieceType::S => GREEN.into(),
            PieceType::T => PURPLE.into(),
            PieceType::Z => RED.into(),
        }
    }
}

const PIECES: [PieceType; 7] = [
    PieceType::I,
    PieceType::J,
    PieceType::L,
    PieceType::O,
    PieceType::S,
    PieceType::T,
    PieceType::Z,
];

impl PieceType {
    /// Build a piece from the piece type
    pub fn build(&self, commands: &mut Commands) {
        let blocks = select_piece(*self);
        for block in blocks.iter() {
            commands.spawn(PieceBundle {
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: self.into(),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        block.x as f32 * BLOCK_SIZE,
                        block.y as f32 * BLOCK_SIZE,
                        0.0,
                    ),
                    ..Default::default()
                },
                block: *block,
                piece_type: *self,
                movable: Movable {
                    down: true,
                    left: true,
                    right: true,
                },
            });
        }
    }
}
