use bevy::prelude::*;
use sickle_ui::prelude::*;

use crate::state::{AppState, GameState};

pub struct TetrisUIPlugin;

impl Plugin for TetrisUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(OnEnter(GameState::Pause), setup_pause_menu)
            .add_systems(OnEnter(GameState::GameOver), setup_gameover_menu)
            .add_systems(Update, handle_buttons)
            .add_systems(
                Update,
                handle_pause.run_if(
                    in_state(AppState::GameState).and_then(not(in_state(GameState::GameOver))),
                ),
            );
    }
}

#[derive(Component)]
enum MenuButton {
    Play,
    Continue,
    Restart,
    MainMenu,
    Quit,
}

pub fn setup_main_menu(mut commands: Commands) {
    commands
        .ui_builder(UiRoot)
        .column(|column| {
            column.row(|row| {
                row.style()
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center);
                row.label(LabelConfig::from("TETRIS"))
                    .style()
                    .font_size(48.0);
            });
            column.row(|row| {
                row.style()
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center);
                row.label(LabelConfig::from("by")).style().font_size(24.0);
            });
            column.row(|row| {
                row.style()
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center);
                row.label(LabelConfig::from("Daniel Santana"))
                    .style()
                    .font_size(24.0);
            });

            column
                .row(|row| {
                    row.style()
                        .align_items(AlignItems::Center)
                        .justify_content(JustifyContent::Center);
                    row.spawn((
                        MenuButton::Play,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(200.0),
                                margin: UiRect::all(Val::Px(10.0)),
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    ))
                    .label(LabelConfig::from("PLAY"))
                    .style()
                    .font_size(32.0);
                })
                .style()
                .padding(UiRect::top(Val::Percent(15.0)));
            column.row(|row| {
                row.style()
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center);
                row.spawn((
                    MenuButton::Quit,
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ))
                .label(LabelConfig::from("QUIT"))
                .style()
                .font_size(32.0);
            });
        })
        .style()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .entity_commands()
        .insert((StateScoped(AppState::MainMenu), Name::new("MainMenu")));
}

pub fn setup_pause_menu(mut commands: Commands) {
    commands
        .ui_builder(UiRoot)
        .column(|column| {
            column.row(|row| {
                row.style()
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center);
                row.label(LabelConfig::from("PAUSE MENU"))
                    .style()
                    .font_size(48.0);
            });

            column
                .row(|row| {
                    row.style()
                        .align_items(AlignItems::Center)
                        .justify_content(JustifyContent::Center);
                    row.spawn((
                        MenuButton::Continue,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(200.0),
                                margin: UiRect::all(Val::Px(10.0)),
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    ))
                    .label(LabelConfig::from("CONTINUE"))
                    .style()
                    .font_size(32.0);
                })
                .style()
                .padding(UiRect::top(Val::Percent(15.0)));
            column.row(|row| {
                row.style()
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center);
                row.spawn((
                    MenuButton::MainMenu,
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ))
                .label(LabelConfig::from("MAIN MENU"))
                .style()
                .font_size(32.0);
            });
        })
        .style()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .entity_commands()
        .insert((StateScoped(GameState::Pause), Name::new("PauseMenu")));
}

pub fn setup_gameover_menu(mut commands: Commands) {
    commands
        .ui_builder(UiRoot)
        .column(|column| {
            column.row(|row| {
                row.style()
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center);
                row.label(LabelConfig::from("GAME OVER"))
                    .style()
                    .font_size(60.0);
            });

            column
                .row(|row| {
                    row.style()
                        .align_items(AlignItems::Center)
                        .justify_content(JustifyContent::Center);
                    row.spawn((
                        MenuButton::Restart,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(200.0),
                                margin: UiRect::all(Val::Px(10.0)),
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    ))
                    .label(LabelConfig::from("RESTART"))
                    .style()
                    .font_size(32.0);
                })
                .style()
                .padding(UiRect::top(Val::Percent(15.0)));
            column.row(|row| {
                row.style()
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center);
                row.spawn((
                    MenuButton::MainMenu,
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ))
                .label(LabelConfig::from("MAIN MENU"))
                .style()
                .font_size(32.0);
            });
        })
        .style()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .entity_commands()
        .insert((StateScoped(GameState::GameOver), Name::new("GameOverMenu")));
}

fn handle_buttons(
    mut query: Query<(&Interaction, &MenuButton, &mut BackgroundColor)>,
    mut state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button, mut background) in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match button {
                MenuButton::Play => {
                    // Change the state to the game
                    state.set(AppState::GameState);
                }
                MenuButton::Quit => {
                    // Just close the game
                    std::process::exit(0);
                }
                MenuButton::Continue => {
                    // Change the state to the game
                    game_state.set(GameState::Play);
                }
                MenuButton::Restart => {
                    // Change the state to the game
                    game_state.set(GameState::Play);
                }
                MenuButton::MainMenu => {
                    // Change the state to the game
                    state.set(AppState::MainMenu);
                }
            }
        } else if *interaction == Interaction::Hovered {
            background.0 = bevy::color::palettes::css::DARK_GREY.into();
        } else {
            background.0 = Color::NONE;
        }
    }
}

fn handle_pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if *current_state == GameState::Pause {
            game_state.set(GameState::Play);
        } else {
            game_state.set(GameState::Pause);
        }
    }
}
