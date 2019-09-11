use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::World;
use amethyst::prelude::Builder;
use amethyst::renderer::*;

pub fn init_camera(world: &mut World) {
    let mut camera_transform = Transform::default();
    camera_transform.set_translation_xyz(100.0, 100.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(200.0, 200.0))
        .with(camera_transform)
        .build();
}
