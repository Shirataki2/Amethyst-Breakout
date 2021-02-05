use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, System, ReadStorage, WriteStorage, SystemData}
};
use crate::components::block::Block;

#[derive(SystemDesc)]
pub struct BlockSystem;

impl<'s> System<'s> for BlockSystem {
    type SystemData = (
        ReadStorage<'s, Block>,
        WriteStorage<'s, Transform>
    );

    fn run(&mut self, (blocks, mut locals): Self::SystemData) {
        for (block, transform) in (&blocks, &mut locals).join() {
            transform.set_translation_xyz(
                block.x,
                block.y,
                0.0
            );
        }
    }
}
