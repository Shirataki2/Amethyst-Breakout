mod states;
mod settings;
mod components;
mod bundle;
mod resources;
mod systems;
mod utils;

use amethyst::{
    prelude::*,
    core::{
        transform::TransformBundle,
        frame_limiter::FrameRateLimitStrategy,
    },
    input::InputBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use std::time::Duration;

use crate::{
    states::LoadState,
    settings::MovementBindingTypes,
    bundle::GameBundle,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let assets_root_dir = app_root.join("assets");

    let key_bindings_path = app_root.join("config").join("bindings.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<MovementBindingTypes>::new().with_bindings_from_file(key_bindings_path)?
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(GameBundle)?;

    let mut game = Application::build(assets_root_dir, LoadState::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)), 144
        )
        .build(game_data)?;
    game.run();
    Ok(())
}
