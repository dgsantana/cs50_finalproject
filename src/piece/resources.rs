use std::collections::VecDeque;

use bevy::prelude::*;
use rand::prelude::*;

use crate::common::CACHED_PIECES;

use super::components::{PieceType, PIECES};

#[derive(Resource)]
pub struct MoveDownTimer(pub Timer);

#[derive(Resource)]
pub struct ManualMoveTimer(pub Timer);

#[derive(Resource)]
pub struct PiecesQueue(VecDeque<PieceType>);

impl PiecesQueue {
    pub fn new() -> Self {
        let mut result = Self(VecDeque::new());
        result.generate();
        result
    }

    /// Generates a new queue of pieces
    ///
    /// Only the first generated set of pieces is guaranteed to be unique.
    /// Other generated sets may contain duplicates, due to extending the queue.
    fn generate(&mut self) {
        let mut rng = thread_rng();

        let mut pieces = Vec::new();
        loop {
            match PIECES.get(rng.gen_range(0..7)) {
                Some(piece) => {
                    if pieces.contains(piece) {
                        continue;
                    }
                    pieces.push(*piece);
                }
                None => {
                    // Just ignore the error and try again logging the error
                    error!("Failed to generate a piece");
                }
            }
            if pieces.len() >= (CACHED_PIECES - self.0.len()) {
                break;
            }
        }
        info!("Generated pieces: {:?}", pieces);

        // Add the pieces to the queue
        self.0.extend(pieces);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets the next piece and keeps the queue filled
    pub fn next(&mut self) -> PieceType {
        // Always keep the queue filled
        if self.0.len() <= CACHED_PIECES {
            self.generate();
        }
        self.0.pop_front().unwrap()
    }

    pub fn peek(&self) -> Option<&PieceType> {
        self.0.front()
    }
}
