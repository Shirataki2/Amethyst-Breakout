use amethyst::{
    assets::{Loader, ProgressCounter, AssetStorage},
    ecs::prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteSheetFormat, SpriteSheetHandle},
        SpriteSheet, Texture
    }
};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetType {
    Blocks
}

#[derive(Clone, Default)]
pub struct SpriteSheetList {
    sprite_list: HashMap<AssetType, SpriteSheetHandle>,
}

impl SpriteSheetList {
    pub fn insert(&mut self, asset_type: AssetType, sprite_handle: SpriteSheetHandle) {
        self.sprite_list.insert(asset_type, sprite_handle);
    }

    #[allow(dead_code)]
    pub fn get(&self, asset_type: AssetType) -> Option<&SpriteSheetHandle> {
        self.sprite_list.get(&asset_type)
    }
}

pub fn load_assets(world: &mut World, asset_types: Vec<AssetType>) -> ProgressCounter {
    let mut sprite_list = SpriteSheetList::default();
    let mut progress_counter = ProgressCounter::new();
    for &asset_type in asset_types.iter() {
        let (texture_path, ron_path) = match asset_type {
            AssetType::Blocks => ("images/blocks.png", "images/blocks.ron")
        };
        match asset_type {
            AssetType::Blocks => {
                let handle = get_sprite_sheet_handle(world, texture_path, ron_path, &mut progress_counter);
                sprite_list.insert(asset_type, handle);
            }
        };
    }
    world.insert(sprite_list);
    progress_counter
}

fn get_sprite_sheet_handle(world: &World, texture_path: &str, ron_path: &str, progress_counter: &mut ProgressCounter) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = &world.read_resource::<Loader>();
        let storage = &world.read_resource::<AssetStorage<Texture>>();
        loader.load(texture_path, ImageFormat::default(), (), &storage)
    };
    let loader = &world.read_resource::<Loader>();
    let sprite_sheet_store = &world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        progress_counter,
        &sprite_sheet_store
    )
}