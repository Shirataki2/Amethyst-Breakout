pub mod menu;
pub mod game;
pub mod load;

pub use self::menu::MenuState;
pub use self::game::MainGameState;
pub use self::load::LoadState;

#[derive(Clone, Default)]
pub struct GameStage {
    stage: usize,
}
