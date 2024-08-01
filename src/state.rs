use bevy::prelude::*;

#[derive(Default, States, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum AppState {
    #[default]
    MainMenu,
    GameState,
    GameOver,
}

#[derive(Default, SubStates, PartialEq, Eq, Hash, Debug, Clone, Copy)]
#[source(AppState = AppState::GameState)]
pub enum GameState {
    #[default]
    Play,
    Pause,
    Restart,
    Quit,
}
