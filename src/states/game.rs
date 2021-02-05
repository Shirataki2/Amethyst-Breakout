use amethyst::{
    prelude::*,
    core::{transform::Transform, math::Vector3},
    renderer::{Camera, SpriteRender},
    utils::application_root_dir,
};
use crate::{
    states::GameStage,
    components::{
        block::{Stage, Block},
        paddle::Paddle,
        ball::Ball,
    },
    resources::assets::{SpriteSheetList, AssetType},
    utils::Dimension,
};

#[derive(Default)]
pub struct MainGameState;

impl SimpleState for MainGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let stage = world.read_resource::<GameStage>().stage;
        let spritesheet_list = (*world.read_resource::<SpriteSheetList>()).clone();
    
        initialize_camera(world);
        initialize_paddle(world, &spritesheet_list);
        initialize_ball(world, &spritesheet_list);
        initialize_stage(world, stage, &spritesheet_list);
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    let camera_config_path = application_root_dir().unwrap()
        .join("config")
        .join(format!("camera.ron"));
    let dim = Dimension::load(camera_config_path).unwrap();
    transform.set_translation_xyz(dim.width * 0.5, dim.height * 0.5, 1.0);
    world.insert(dim);
    world
        .create_entity()
        .with(Camera::standard_2d(dim.width, dim.height))
        .with(transform)
        .build();
}

fn initialize_stage(world: &mut World, stage: usize, spritesheet_list: &SpriteSheetList) {
    println!("Stage: {} Started.", stage);
    let state_config_path = application_root_dir().unwrap()
        .join("assets")
        .join("stages")
        .join(format!("{}.ron", stage));
    let stage_config = Stage::load(state_config_path).unwrap();
    for block in stage_config.blocks.iter() {
        let (x, y) = normalize_coords(block.x, block.y);
        let mut transform = Transform::default();
        transform.set_scale(Vector3::new(3e-2, 3e-2, 3e-2));
        transform.set_translation_xyz(x, y, 0.0);
        let handle = spritesheet_list.get(AssetType::Blocks).unwrap().clone();
        let render = SpriteRender::new(handle, block.sprite);
        world
            .create_entity()
            .with(render)
            .with(Block { x, y, sprite: block.sprite })
            .with(transform)
            .build();
    }
}

fn initialize_paddle(world: &mut World, spritesheet_list: &SpriteSheetList) {
    let paddle_config_path = application_root_dir().unwrap()
        .join("assets")
        .join("sprites")
        .join("paddles.ron");
    let mut paddle = Paddle::load(paddle_config_path).unwrap();
    let animator = paddle.animator.clone();
    let (x, y) = normalize_coords(paddle.position.x, paddle.position.y);
    paddle.position.x = x;
    paddle.position.y = y;
    let mut transform = Transform::default();
    let f = paddle.scale_factor;
    transform.set_translation_xyz(x, y, 0.0);
    transform.set_scale(Vector3::new(1.3 * f, f, f));
    let handle = spritesheet_list.get(AssetType::Blocks).unwrap().clone();
    let render = SpriteRender::new(handle, paddle.animator.start_sprites);
    world
        .create_entity()
        .with(render)
        .with(paddle)
        .with(animator)
        .with(transform)
        .build();
}

fn initialize_ball(world: &mut World, spritesheet_list: &SpriteSheetList) {
    let ball_config_path = application_root_dir().unwrap()
        .join("assets")
        .join("sprites")
        .join("ball.ron");
    let mut ball = Ball::load(ball_config_path).unwrap();
    let animator = ball.animator.clone();
    let (x, y) = ball.initial_position;
    let (x, y) = normalize_coords(x, y);
    ball.initial_position = (x, y);
    let mut transform = Transform::default();
    let f = ball.scale_factor;
    transform.set_translation_xyz(x, y, 0.0);
    transform.set_scale(Vector3::new(1.3 * f, f, f));
    let handle = spritesheet_list.get(AssetType::Blocks).unwrap().clone();
    let render = SpriteRender::new(handle, ball.animator.start_sprites);
    world
        .create_entity()
        .with(render)
        .with(ball)
        .with(animator)
        .with(transform)
        .build();
}

pub fn normalize_coords(x: f32, y: f32) -> (f32, f32) {
    (140.0 * x, 140.0 * y)
}
