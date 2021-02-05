use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, Entities, System, ReadStorage, WriteStorage, SystemData}
};
use crate::{
    components::{Ball, Block},
    states::game::normalize_coords
};

#[derive(SystemDesc)]
pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Block>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, mut balls, mut blocks, locals): Self::SystemData) {
        for (ball, ball_transform) in (&mut balls, &locals).join() {
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
            let mut removal_entities = vec![];
            let (mut inv_x, mut inv_y) = (false, false);
            for (_block, ent, block_transform) in (&mut blocks, &entities, &locals).join() {
                let block_x = block_transform.translation().x;
                let block_y = block_transform.translation().y;
                
                // TODO: Change to load (w, h) from ron file
                let (w, h) = normalize_coords(0.06f32, 0.02f32);
                match point_in_rect(x, y, block_x - r - w / 2.0, block_y - r - h / 2.0, block_x + w / 2.0 + r, block_y + h / 2.0 + r) {
                    Direction::Left | Direction::Right => {
                        inv_x |= true;
                        removal_entities.push(ent);
                    },
                    Direction::Bottom | Direction::Top => {
                        inv_y |= true;
                        removal_entities.push(ent);
                    },
                    _ => {}
                }
            }
            if inv_x { ball.velocity.0 *= -1.0; }
            if inv_y { ball.velocity.1 *= -1.0; }
            for &ent in removal_entities.iter() {
                let _ = entities.delete(ent);
            }
        }
    }
}

enum Direction {
    Top, Bottom, Left, Right, None
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> Direction {
    if x >= left && x <= right && y >= bottom && y <= top {
        let argmin = vec![(x - left).abs(), (x - right).abs(), (y - top).abs(), (y - bottom).abs()]
            .iter()
            .enumerate()
            .min_by(|(_, &a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);
        match argmin {
            0 => Direction::Left,
            1 => Direction::Right,
            2 => Direction::Top,
            3 => Direction::Bottom,
            _ => Direction::None,
        }
    } else {
        Direction::None
    }
}