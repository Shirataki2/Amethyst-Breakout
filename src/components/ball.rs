use amethyst::{
    ecs::{Component, DenseVecStorage},
};
use serde::{Deserialize, Serialize};
use crate::{
    components::animator::Animator,
};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Ball {
    pub animator: Animator,
    pub velocity: (f32, f32),
    pub initial_position: (f32, f32),
    pub radius: f32,
    pub scale_factor: f32,
    pub sticked: bool,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
