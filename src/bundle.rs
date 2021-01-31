use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    error::Error,
};

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, _world: &mut World, _builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        Ok(())
    }
}
