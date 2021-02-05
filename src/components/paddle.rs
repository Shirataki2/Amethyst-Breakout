use amethyst::{
    ecs::{Component, DenseVecStorage},
};
use serde::{Deserialize, Serialize};
use crate::{
    components::animator::Animator,
    utils::types::Position,
};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Paddle {
    pub animator: Animator,
    pub position: Position,
    pub scale_factor: f32,
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}
