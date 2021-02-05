use amethyst::{
    core::{timing::Time},
    derive::SystemDesc,
    renderer::SpriteRender,
    ecs::{Join, System, Read, WriteStorage, SystemData}
};
use crate::components::{
    animator::Animator,
};

#[derive(SystemDesc)]
pub struct AnimatorSystem;

impl<'s> System<'s> for AnimatorSystem {
    type SystemData = (
        WriteStorage<'s, Animator>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut animators, mut renderers, time): Self::SystemData) {
        for (animator, renderer) in (&mut animators, &mut renderers).join() {
            animator.elapsed_time += time.delta_seconds();
            let frame_count = (animator.elapsed_time / animator.time_per_frame) as usize % animator.sprite_length;
            if frame_count != animator.current_frame {
                animator.current_frame = frame_count;
                renderer.sprite_number = animator.start_sprites + frame_count;
            }
        }
    }
}