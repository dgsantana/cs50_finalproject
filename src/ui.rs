use bevy::prelude::*;
use sickle_ui::prelude::*;

use crate::state::AppState;

pub struct TetrisUIPlugin;

impl Plugin for TetrisUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(Update, handle_buttons.run_if(in_state(AppState::MainMenu)));
    }
}

#[derive(Component)]
enum MenuButton {
    Play,
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
                row.label(LabelConfig::from("Cetris"))
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
                    row.spawn((MenuButton::Play, ButtonBundle::default()))
                        .label(LabelConfig::from("Play"))
                        .style()
                        .width(Val::Px(200.0))
                        .font_size(32.0);
                })
                .style()
                .padding(UiRect::top(Val::Percent(15.0)));
            column.row(|row| {
                row.style()
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center);
                row.spawn((MenuButton::Quit, ButtonBundle::default()))
                    .label(LabelConfig::from("Quit"))
                    .style()
                    .width(Val::Px(200.0))
                    .font_size(32.0);
            });
        })
        .style()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .entity_commands()
        .insert(StateScoped(AppState::MainMenu));
}

fn handle_buttons(
    mut query: Query<(&Interaction, &MenuButton, &mut BackgroundColor)>,
    mut state: ResMut<NextState<AppState>>,
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
            }
        } else if *interaction == Interaction::Hovered {
            background.0 = bevy::color::palettes::css::DARK_GREY.into();
        } else {
            background.0 = Color::NONE;
        }
    }
}
