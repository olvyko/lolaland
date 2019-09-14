use amethyst::{
    core::{Parent, Transform},
    ecs::{prelude::World, Entity},
    prelude::Builder,
    renderer::camera::Camera,
    window::ScreenDimensions,
};

use crate::components::Subject;

pub fn load_camera(world: &mut World, camera_subject: Entity) -> Entity {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(200.0, 200.0))
        .with(Parent {
            entity: camera_subject,
        })
        .with(Subject::default())
        .with(transform)
        .build()
}
