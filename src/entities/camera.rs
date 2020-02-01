use amethyst::{
    core::{Parent, Transform},
    ecs::{prelude::World, Entity},
    prelude::*,
    renderer::{camera::Camera, transparent::Transparent},
    window::ScreenDimensions,
};

use crate::components::Subject;

pub fn create_camera(world: &mut World) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(100.0, 100.0, 0.0);
    let camera_subject = world
        .create_entity()
        .with(transform)
        .with(Subject::default())
        .with(Transparent)
        .build();

    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(Parent { entity: camera_subject })
        .with(Subject::default())
        .with(transform)
        .build()
}
