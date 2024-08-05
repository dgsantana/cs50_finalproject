use std::time::Duration;

use bevy::prelude::*;
use sickle_ui::prelude::*;

use crate::{
    common::BLOCK_SIZE,
    piece::{select_piece, MoveDownTimer, PieceType, TetrisSet},
    state::AppState,
};

#[derive(Resource, Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Score {
    pub value: u64,
    pub lines: u64,
}

#[derive(Debug, Clone, Copy, Resource)]
pub struct HighScore(pub Score);

#[derive(Component)]
struct NextPieceTag;

#[derive(Component)]
struct NextPieceLabel;

#[derive(Debug, Clone, Copy, Event)]
pub struct ScoreEvent(pub Score);

#[derive(Debug, Clone, Copy, Event)]
pub struct NextPieceEvent(pub PieceType);

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HighScore(Score::default()))
            .insert_resource(Score::default())
            .add_event::<ScoreEvent>()
            .add_event::<NextPieceEvent>()
            .add_systems(Startup, (setup_score_ui, setup_next_piece_ui))
            .add_systems(
                OnEnter(AppState::GameState),
                |mut commands: Commands,
                 mut score_event: EventWriter<ScoreEvent>,
                 query: Query<Entity, With<NextPieceTag>>| {
                    // Reset the score when entering the game state
                    commands.insert_resource(Score::default());
                    // Send a score event to update the UI
                    score_event.send(ScoreEvent(Score::default()));
                    // Despawn the next piece
                    for entity in query.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                },
            )
            .add_systems(
                Update,
                update_next_piece
                    .after(TetrisSet::Spawn)
                    .run_if(on_event::<NextPieceEvent>().and_then(in_state(AppState::GameState))),
            )
            .add_systems(
                Update,
                update_stats
                    .after(TetrisSet::Collision)
                    .run_if(on_event::<ScoreEvent>().and_then(in_state(AppState::GameState))),
            );
    }
}

#[derive(Component)]
enum ScoreText {
    Score,
    HighScore,
    Lines,
}

pub fn setup_score_ui(mut commands: Commands, highscore: Res<HighScore>) {
    commands
        .ui_builder(UiRoot)
        .container(
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
                        .align_content(AlignContent::Center)
                        .justify_content(JustifyContent::Start);
                    column.row(|row| {
                        row.style()
                            .padding(UiRect::top(Val::Px(50.0)))
                            .justify_content(JustifyContent::Center);
                        row.label(LabelConfig::from("Score"))
                            .style()
                            .font_size(24.0)
                            .align_self(AlignSelf::Center);
                    });
                    column.row(|row| {
                        row.style().justify_content(JustifyContent::Center);
                        row.label(LabelConfig::from("0"))
                            .style()
                            .font_size(24.0)
                            .align_self(AlignSelf::Center)
                            .entity_commands()
                            .insert(ScoreText::Score);
                    });

                    column.row(|row| {
                        row.style()
                            .padding(UiRect::top(Val::Px(20.0)))
                            .justify_content(JustifyContent::Center);
                        row.label(LabelConfig::from("High Score"))
                            .style()
                            .font_size(24.0)
                            .align_self(AlignSelf::Center);
                    });
                    column.row(|row| {
                        row.style().justify_content(JustifyContent::Center);
                        row.label(LabelConfig::from(format!("{}", highscore.0.value)))
                            .style()
                            .font_size(24.0)
                            .align_self(AlignSelf::Center)
                            .entity_commands()
                            .insert(ScoreText::HighScore);
                    });

                    column.row(|row| {
                        row.style()
                            .padding(UiRect::top(Val::Px(20.0)))
                            .justify_content(JustifyContent::Center);
                        row.label(LabelConfig::from("Lines"))
                            .style()
                            .font_size(24.0)
                            .align_self(AlignSelf::Center);
                    });
                    column.row(|row| {
                        row.style().justify_content(JustifyContent::Center);
                        row.label(LabelConfig::from("0"))
                            .style()
                            .font_size(24.0)
                            .align_self(AlignSelf::Center)
                            .entity_commands()
                            .insert(ScoreText::Lines);
                    });
                });
            },
        )
        .insert(Name::new("ScoreRoot"));
}

pub fn setup_next_piece_ui(mut commands: Commands) {
    commands
        .ui_builder(UiRoot)
        .container(
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Start,
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
                        .align_content(AlignContent::Center)
                        .justify_content(JustifyContent::Start);
                    column.row(|row| {
                        row.style()
                            .padding(UiRect::top(Val::Px(50.0)))
                            .justify_content(JustifyContent::Center);
                        row.label(LabelConfig::from("Next Piece"))
                            .style()
                            .font_size(24.0)
                            .align_self(AlignSelf::Center)
                            .entity_commands()
                            .insert(NextPieceLabel);
                    });
                });
            },
        )
        .insert(Name::new("NextPieceRoot"));
}

fn update_stats(
    mut q_score: Query<(&mut Text, &ScoreText)>,
    mut high_score: ResMut<HighScore>,
    mut score: ResMut<Score>,
    mut score_event: EventReader<ScoreEvent>,
    mut drop_timer: ResMut<MoveDownTimer>,
) {
    if score_event.is_empty() {
        return;
    }

    score_event.read().for_each(|s| {
        score.value += s.0.value;
        score.lines += s.0.lines;
    });

    if score.value > high_score.0.value {
        high_score.0 = *score;
    }

    // Decrease the speed of drop at every 500 score, making the drop faster up to 0.05s
    // Using i32 div will make the score work as a step function
    let level = score.value / 500;
    let new_duration = Duration::from_secs_f32((1.0 - level as f32 * 0.1).max(0.05));
    if drop_timer.0.duration() > new_duration {
        drop_timer.0.set_duration(new_duration);
        drop_timer.0.reset();
    }

    for (mut text, score_text) in q_score.iter_mut() {
        match score_text {
            ScoreText::Score => {
                text.sections[0].value = score.value.to_string();
            }
            ScoreText::HighScore => {
                text.sections[0].value = high_score.0.value.to_string();
            }
            ScoreText::Lines => {
                text.sections[0].value = score.lines.to_string();
            }
        }
    }
}

fn update_next_piece(
    mut commands: Commands,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    q_piece_label: Query<&GlobalTransform, With<NextPieceLabel>>,
    query: Query<Entity, With<NextPieceTag>>,
    mut next_piece_event: EventReader<NextPieceEvent>,
) {
    if next_piece_event.is_empty() {
        return;
    }

    let (camera, camera_transform) = q_camera.single();

    let label_pos = q_piece_label.single();

    // Clear the previous next piece
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let piece_type = next_piece_event.read().last().unwrap().0;
    let piece_color = Color::from(&piece_type);

    let blocks = select_piece(piece_type);

    // Calculate the first third of the screen width from the camera and place there the parent centered
    let mut pos = camera
        .viewport_to_world_2d(
            camera_transform,
            Vec2::new(label_pos.translation().x, label_pos.translation().y),
        )
        .unwrap();

    pos.x -= BLOCK_SIZE * 3.0;
    pos.y -= BLOCK_SIZE * 3.0;

    let parent = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(pos.extend(0.0)),
                ..Default::default()
            },
            NextPieceTag,
            Name::new("NextPiece"),
        ))
        .id();

    blocks.iter().for_each(|b| {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: piece_color,
                    ..Default::default()
                },
                transform: b.as_transform(),
                ..Default::default()
            })
            .set_parent(parent);
    });
}
