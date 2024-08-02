use bevy::color::palettes::css::*;
use bevy::prelude::*;

use crate::common::{BLOCK_SIZE, BLOCK_SPRITE_SIZE, BOARD_CENTER_X, BOARD_CENTER_Y};
use crate::state::AppState;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct Block {
    x: i32,
    y: i32,
}

impl Block {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn move_down(&mut self) {
        self.y -= 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn shift_y(&mut self, shift: i32) {
        self.y += shift;
    }

    pub fn as_translation(&self) -> Vec3 {
        Vec3::new(
            (self.x as f32 - BOARD_CENTER_X + 0.5) * BLOCK_SIZE,
            (self.y as f32 - BOARD_CENTER_Y + 0.5) * BLOCK_SIZE,
            0.0,
        )
    }
    pub fn as_transform(&self) -> Transform {
        // The origin is the center top of the board
        Transform::from_translation(self.as_translation())
            .with_scale(Vec3::splat(BLOCK_SPRITE_SIZE))
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Reflect)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Movable {
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Movable {
    /// Create a new Movable struct with all the movements enabled
    pub fn new() -> Self {
        Self {
            down: true,
            left: true,
            right: true,
        }
    }

    /// Check if the piece can rotate
    /// 
    /// All the movements should be enabled for rotation
    pub fn can_rotate(&self) -> bool {
        self.down && self.left && self.right
    }

    /// Check if the piece can move down
    pub fn can_move_down(&self) -> bool {
        self.down
    }

    /// Check if the piece can move left
    pub fn can_move_left(&self) -> bool {
        self.left
    }

    /// Check if the piece can move right
    pub fn can_move_right(&self) -> bool {
        self.right
    }
}

#[derive(Bundle)]
pub struct PieceBundle {
    sprite: SpriteBundle,
    block: Block,
    piece_type: PieceType,
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

pub const PIECES: [PieceType; 7] = [
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
        let mut blocks = select_piece(*self);
        for block in blocks.iter_mut() {
            block.y += 20;
            commands
                .spawn(PieceBundle {
                    sprite: SpriteBundle {
                        sprite: Sprite {
                            color: self.into(),
                            ..Default::default()
                        },
                        transform: block.as_transform(),
                        ..Default::default()
                    },
                    block: *block,
                    piece_type: *self,
                })
                .insert(StateScoped(AppState::GameState));
        }
    }
}

/// A helper to help with the rotation of the pieces
pub struct Piece {
    piece_type: PieceType,
    blocks: [Block; 4],
    pivot_x: i32,
    pivot_y: i32,
}

impl Piece {
    /// Create a new piece from an array of blocks
    pub fn from_array(q_blocks: &[Block], piece_type: PieceType) -> Self {
        let mut blocks = [Block::new(0, 0); 4];
        for (i, block) in q_blocks.iter().enumerate() {
            blocks[i] = *block;
        }
        let mut result = Self {
            piece_type,
            blocks,
            pivot_x: 0,
            pivot_y: 0,
        };
        result.compute();
        result
    }

    pub fn blocks(&self) -> [Block; 4] {
        self.blocks
    }

    /// Compute the pivot and if the piece is rotated
    fn compute(&mut self) {
        let sum_x: i32 = self.blocks.iter().map(|b| b.x).sum();
        let sum_y: i32 = self.blocks.iter().map(|b| b.y).sum();
        // Average of the x and y coordinates of the blocks
        self.pivot_x = sum_x / 4 - sum_y / 4;
        self.pivot_y = sum_x / 4 + sum_y / 4;
    }

    /// Rotate the blocks of the piece
    pub fn rotate_blocks(&self) -> [Block; 4] {
        let mut blocks = self.blocks;
        if self.piece_type == PieceType::O {
            return blocks;
        }
        blocks.iter_mut().for_each(|block| {
            self.rotate_block(block);
        });
        blocks
    }

    /// Rotate a block
    pub fn rotate_block(&self, block: &mut Block) {
        if self.piece_type == PieceType::O {
            return;
        }
        let inv_x = -block.x;
        block.x = block.y + self.pivot_x;
        // Trial and error to find the correct formula
        if matches!(self.piece_type, PieceType::L | PieceType::J) {
            block.y = inv_x + self.pivot_y;
        } else {
            block.y = inv_x + self.pivot_y + 1;
        }
    }
}
