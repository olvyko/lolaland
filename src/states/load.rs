use amethyst::{
    assets::ProgressCounter,
    core::{math::Vector3, Transform},
    prelude::{GameData, SimpleState, SimpleTrans, StateData, Trans},
    renderer::SpriteRender,
};

use crate::{
    components::{Animation, DynamicBox, Subject},
    entities::{load_camera, load_camera_subject, load_dynamic_box, load_lola},
    resources::{load_assets, AssetType, Context, Map, PrefabList, SpriteSheetList},
};

use specs_physics::{nalgebra::Vector2, parameters::Gravity};

#[derive(Default)]
pub struct LoadState {
    progress_counter: Option<ProgressCounter>,
    map: Option<Map>,
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Animation>();
        world.register::<DynamicBox>();

        world.add_resource(Context::new());
        world.add_resource(Gravity::<f32>(Vector2::<f32>::new(0.0, -9.8)));

        self.progress_counter = Some(load_assets(
            world,
            vec![AssetType::Lola, AssetType::RedBricks],
        ));

        self.map = Some(Map::default());

        let camera_subject = load_camera_subject(world);
        load_camera(world, camera_subject);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(ref progress_counter) = self.progress_counter {
            // Check if all data has been loaded
            if progress_counter.is_complete() {
                let map = &self.map.take().unwrap();

                let ctx = data.world.read_resource::<Context>().clone();

                map.load_layers(data.world, &ctx);

                let lola_prefab_handle = {
                    let prefab_list = data.world.read_resource::<PrefabList>();
                    prefab_list.get(AssetType::Lola).unwrap().clone()
                };
                load_lola(data.world, lola_prefab_handle, &ctx);

                let redbricks_sprite = {
                    let sprite_sheet_list = data.world.read_resource::<SpriteSheetList>();
                    let sprite_sheet_handle =
                        sprite_sheet_list.get(AssetType::RedBricks).unwrap().clone();

                    SpriteRender {
                        sprite_sheet: sprite_sheet_handle.clone(),
                        sprite_number: 0,
                    }
                };
                load_dynamic_box(
                    data.world,
                    redbricks_sprite,
                    &ctx,
                    Transform::from(Vector3::new(100.0, 100.0, 0.0)),
                    32.0,
                    32.0,
                );

                let redbricks_sprite = {
                    let sprite_sheet_list = data.world.read_resource::<SpriteSheetList>();
                    let sprite_sheet_handle =
                        sprite_sheet_list.get(AssetType::RedBricks).unwrap().clone();

                    SpriteRender {
                        sprite_sheet: sprite_sheet_handle.clone(),
                        sprite_number: 0,
                    }
                };
                load_dynamic_box(
                    data.world,
                    redbricks_sprite,
                    &ctx,
                    Transform::from(Vector3::new(100.0, 140.0, 0.0)),
                    32.0,
                    32.0,
                );

                self.progress_counter = None;
            }
        }
        Trans::None
    }
}
