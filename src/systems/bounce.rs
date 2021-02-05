use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, System, ReadStorage, WriteStorage, SystemData}
};
use crate::{
    components::{Ball, Block},
    states::game::normalize_coords
};

#[derive(SystemDesc)]
pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Block>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, _blocks, mut locals): Self::SystemData) {
        for (ball, ball_transform) in (&mut balls, &mut locals).join() {
            let x = ball_transform.translation().x;
            let y = ball_transform.translation().y;
            let r = ball.radius;
            let (r, _) = normalize_coords(r, 0.0);
            let (minx, miny) = normalize_coords(0.0, 0.0);
            let (maxx, maxy) = normalize_coords(1.0, 1.0);

            if x - r < minx || x + r > maxx {
                ball.velocity.0 *= -1.0;
            }
            if y - r < miny || y + r > maxy {
                ball.velocity.1 *= -1.0;
            }
        }
    }
}
