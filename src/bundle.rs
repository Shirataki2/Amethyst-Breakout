use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    error::Error,
};
use crate::systems::{
    block::BlockSystem,
    animator::AnimatorSystem,
    paddle::PaddleSystem,
    ball::BallSystem,
    bounce::BounceSystem,
};

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, _world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(AnimatorSystem, "animator_system", &[]);
        builder.add(BlockSystem, "block_system", &[]);
        builder.add(PaddleSystem, "paddle_system", &["animator_system"]);
        builder.add(BallSystem, "ball_system", &["animator_system", "paddle_system"]);
        builder.add(BounceSystem, "bounce_system", &["block_system", "ball_system"]);
        Ok(())
    }
}
