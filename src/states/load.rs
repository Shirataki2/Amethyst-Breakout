use amethyst::{
    prelude::*,
    assets::ProgressCounter,
};
use crate::{
    resources::assets::{AssetType, load_assets},
    states::{MenuState, MainGameState, GameStage},
};
use std::env;


const SKIP_MENU_ARGS: &'static str = "--no-menu";


#[derive(Default)]
pub struct LoadState {
    progress_counter: Option<ProgressCounter>
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        println!("Loading Assets: Start");

        let stage = GameStage::default();
        world.insert(stage);

        self.progress_counter = Some(load_assets(
            world,
            vec![
                AssetType::Blocks,
            ]
        ));
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(ref progress_counter) = self.progress_counter {
            if !progress_counter.is_complete() {
                println!("{} / {}", progress_counter.num_finished(), progress_counter.num_assets());
            } else {
                println!("Loading Assets: Finished");
                self.progress_counter = None;
                if env::args().any(|arg| arg == SKIP_MENU_ARGS) {
                    return Trans::Switch(Box::new(MainGameState::default()));
                } else {
                    return Trans::Switch(Box::new(MenuState::default()));
                }
            }
        }
        Trans::None
    }
}