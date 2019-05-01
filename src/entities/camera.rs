use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::World;
use amethyst::prelude::Builder;
use amethyst::renderer::*;

pub fn init_camera(world: &mut World) {
    let mut camera_transform = Transform::default();
    camera_transform.set_translation_z(1.0);

    world
        .create_entity()
        .with(camera_transform)
        .with(Camera::from(Projection::orthographic(
            0.0, 120.0, 0.0, 120.0,
        )))
        .build();
}
