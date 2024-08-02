use bevy::prelude::*;

use crate::piece::PieceType;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Score {
    pub value: u64,
    pub lines: u64,
}

#[derive(Debug, Clone, Copy, Resource)]
pub struct HighScore(pub Score);

#[derive(Debug, Clone, Copy, Event)]
pub struct ScoreEvent(pub Score);

#[derive(Debug, Clone, Copy, Event)]
pub struct NextPieceEvent(pub PieceType);

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HighScore(Score::default()))
            .add_event::<ScoreEvent>()
            .add_event::<NextPieceEvent>();
    }
}
