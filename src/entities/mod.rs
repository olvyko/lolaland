mod camera;
mod dynamic_box;
mod ground;
mod player;

use amethyst::{
    assets::*,
    core::{
        math::{Vector2, Vector3},
        Transform,
    },
    ecs::prelude::World,
};

pub fn init_entities(world: &mut World) {
    ground::init_ground(world);

    let num = 4;
    for i in 0..num {
        for j in 0..num {
            let x = 100.0 + i as f32 * 14.0;
            let y = 100.0 + j as f32 * 14.0;

            dynamic_box::init_dynamic_box(
                world,
                Transform::from(Vector3::new(x, y, 0.0)),
                14.0,
                14.0,
            );
        }
    }

    player::init_player(world);
    camera::init_camera(world);
}
