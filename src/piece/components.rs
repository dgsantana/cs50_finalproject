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

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Movable {
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Movable {
    pub fn new() -> Self {
        Self {
            down: true,
            left: true,
            right: true,
        }
    }

    pub fn disable_all(&mut self) {
        self.down = false;
        self.left = false;
        self.right = false;
    }

    pub fn can_move_down(&self) -> bool {
        self.down
    }

    pub fn can_move_left(&self) -> bool {
        self.left
    }

    pub fn can_move_right(&self) -> bool {
        self.right
    }
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
                    movable: Movable {
                        down: true,
                        left: true,
                        right: true,
                    },
                })
                .insert(StateScoped(AppState::GameState));
        }
    }
}
