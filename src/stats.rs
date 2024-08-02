use bevy::prelude::*;
use sickle_ui::prelude::*;

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
            .add_event::<NextPieceEvent>()
            .add_systems(Startup, setup_score_ui);
    }
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct HighScoreText;

#[derive(Component)]
struct LinesText;

pub fn setup_score_ui(mut commands: Commands) {
    commands.ui_builder(UiRoot).container(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::End,
                ..Default::default()
            },
            ..Default::default()
        },
        |child| {
            child.column(|column| {
                column
                    .style()
                    .right(Val::Px(0.0))
                    .width(Val::Percent(35.0))
                    .height(Val::Percent(100.0))
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Start);
                column.row(|row| {
                    row.style().padding(UiRect::top(Val::Px(50.0)));
                    row.label(LabelConfig::from("Score"))
                        .style()
                        .font_size(24.0)
                        .align_self(AlignSelf::Center);
                });
                column.row(|row| {
                    row.label(LabelConfig::from("0"))
                        .style()
                        .font_size(24.0)
                        .align_self(AlignSelf::Center)
                        .entity_commands()
                        .insert(ScoreText);
                });

                column.row(|row| {
                    row.style().padding(UiRect::top(Val::Px(20.0)));
                    row.label(LabelConfig::from("High Score"))
                        .style()
                        .font_size(24.0)
                        .align_self(AlignSelf::Center);
                });
                column.row(|row| {
                    row.label(LabelConfig::from("0"))
                        .style()
                        .font_size(24.0)
                        .align_self(AlignSelf::Center)
                        .entity_commands()
                        .insert(HighScoreText);
                });

                column.row(|row| {
                    row.style().padding(UiRect::top(Val::Px(20.0)));
                    row.label(LabelConfig::from("Lines"))
                        .style()
                        .font_size(24.0)
                        .align_self(AlignSelf::Center);
                });
                column.row(|row| {
                    row.label(LabelConfig::from("0"))
                        .style()
                        .font_size(24.0)
                        .align_self(AlignSelf::Center)
                        .entity_commands()
                        .insert(LinesText);
                });
            });
        },
    );
}
