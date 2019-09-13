use amethyst::{
    core::{math::Vector2, Transform},
    ecs::prelude::*,
    prelude::Builder,
};

use specs_physics::{
    colliders::Shape, nphysics::object::BodyStatus, PhysicsBodyBuilder, PhysicsColliderBuilder,
};

pub fn init_dynamic_box(world: &mut World, transform: Transform, width: f32, height: f32) {
    world
        .create_entity()
        .with(transform)
        .with(
            PhysicsBodyBuilder::<f32>::from(BodyStatus::Dynamic)
                .gravity_enabled(true)
                .mass(16.0)
                .build(),
        )
        .with(
            PhysicsColliderBuilder::<f32>::from(Shape::Cuboid {
                half_extents: Vector2::new(width / 2.0, height / 2.0),
            })
            .build(),
        )
        .build();
}
