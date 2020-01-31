use amethyst::{
    core::{math::Vector3, Transform},
    ecs::prelude::World,
    prelude::*,
};

use serde::{Deserialize, Serialize};

use crate::resources::Context;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tilewidth: i32,
    pub tileheight: i32,
}

impl Map {
    pub fn load_layers(&self, world: &mut World, ctx: &Context) {
        world
            .create_entity()
            .with(Transform::from(Vector3::new(100.0, 20.0, 0.0)))
            .build();

        let mut transform = Transform::from(Vector3::new(195.0, 98.0, 0.0));
        transform.set_rotation_2d(-0.2);
        world.create_entity().with(transform).build();

        let mut transform = Transform::from(Vector3::new(5.0, 98.0, 0.0));
        transform.set_rotation_2d(0.2);
        world.create_entity().with(transform).build();
    }
}
