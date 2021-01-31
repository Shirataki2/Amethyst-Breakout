use amethyst::prelude::*;

#[derive(Default)]
pub struct MenuState;

impl SimpleState for MenuState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("started");
    }
}