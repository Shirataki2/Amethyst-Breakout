use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, System, Read, WriteStorage, SystemData},
    input::InputHandler,
};
use crate::{
    components::paddle::Paddle,
    settings::{MovementBindingTypes, AxisBinding},
    states::game::normalize_coords,
};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Paddle>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<MovementBindingTypes>>
    );

    fn run(&mut self, (mut paddles, mut locals, input): Self::SystemData) {
        for (paddle, transform) in (&mut paddles, &mut locals).join() {
            if let Some(amount) = input.axis_value(&AxisBinding::Horizontal(0)) {
                let paddle_x = paddle.position.x + 1.2 * amount;
                let (x_min, x_max) = normalize_coords(0.08, 0.92);
                let paddle_x = paddle_x.min(x_max).max(x_min);
                paddle.position.x = paddle_x;
                transform.set_translation_x(paddle_x);
            }
        }
    }
}