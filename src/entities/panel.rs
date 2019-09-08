use amethyst::{
    assets::*,
    core::{math::Vector3, Transform},
    ecs::prelude::*,
    prelude::Builder,
};

use specs_physics::{
    colliders::Shape, nphysics::object::BodyStatus, PhysicsBody, PhysicsBodyBuilder,
    PhysicsColliderBuilder,
};

pub fn init_panel(world: &mut World) {
    let transform = Transform::from(Vector3::new(50.0, 0.0, 0.0));

    world
        .create_entity()
        .with(transform)
        .with(PhysicsBodyBuilder::<f32>::from(BodyStatus::Static).build())
        .with(
            PhysicsColliderBuilder::<f32>::from(Shape::Cuboid {
                half_extents: Vector3::new(20.0, 20.0, 1.0),
            })
            .build(),
        )
        .build();
}
