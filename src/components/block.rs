use amethyst::{
    ecs::{Component, DenseVecStorage},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Stage {
    pub blocks: Vec<Block>
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Block {
    pub x: f32,
    pub y: f32,
    pub sprite: usize,
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}