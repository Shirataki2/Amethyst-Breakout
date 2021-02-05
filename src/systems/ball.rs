use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, System, Read, ReadStorage, WriteStorage, SystemData},
    input::InputHandler,
};
use crate::{
    components::{ball::Ball, paddle::Paddle},
    settings::{MovementBindingTypes, ActionBinding, AxisBinding},
};

#[derive(SystemDesc)]
pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<MovementBindingTypes>>,
    );

    fn run(&mut self, (mut balls, paddles, mut locals, input): Self::SystemData) {
        for (ball, ball_transform) in (&mut balls, &mut locals).join() {
            ball_transform.prepend_translation_x(ball.velocity.0);
            ball_transform.prepend_translation_y(ball.velocity.1);
    
            for paddle in (&paddles).join() {
                if ball.sticked {
                    let x = paddle.position.x;
                    let y = paddle.position.y + 5.0;
                    ball_transform.set_translation_xyz(x, y, 0.0);
                    if input.action_is_down(&ActionBinding::Shoot(0)).unwrap_or(false) {
                        ball.sticked = false;
                        let amount = if let Some(value) = input.axis_value(&AxisBinding::Horizontal(0)) {
                            -value
                        } else {
                            1.0
                        };
                        ball.velocity = (1.2 * amount, 1.2);
                    }
                }
            }
        }
    }
}