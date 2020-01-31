use amethyst::{assets::ProgressCounter, prelude::*};

use crate::{
    entities::{load_camera, load_camera_subject},
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
        let camera_subject = load_camera_subject(world);
        load_camera(world, camera_subject);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        Trans::None
    }
}
