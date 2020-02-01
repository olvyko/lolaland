use amethyst::{assets::ProgressCounter, prelude::*};

use crate::{
    entities::{create_camera, create_player},
    resources::{load_assets, AssetType, Context, Map},
};

#[derive(Default)]
pub struct LoadState {
    progress_counter: Option<ProgressCounter>,
    map: Option<Map>,
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        world.insert(Context::new());
        create_camera(world);
        create_player(world);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        Trans::None
    }
}
