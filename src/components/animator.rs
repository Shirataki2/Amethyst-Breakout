use amethyst::{
    ecs::{Component, DenseVecStorage},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Animator {
    pub start_sprites: usize,
    pub sprite_length: usize,
    pub time_per_frame: f32,

    pub elapsed_time: f32,
    pub current_frame: usize,
}

impl Component for Animator {
    type Storage = DenseVecStorage<Self>;
}
