#[derive(Default)]
pub enum AppState {
    #[default]
    MainMenu,
    GameState(GameState),
    GameOver,
}

#[derive(Default)]
pub enum GameState {
    Play,
    Pause,
    Restart,
    #[default]
    Quit,
}
