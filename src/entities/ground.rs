use amethyst::{
    assets::*,
    core::{
        math::{Isometry3, Vector2, Vector3},
        Transform,
    },
    ecs::prelude::*,
    prelude::Builder,
};

use specs_physics::{
    colliders::Shape, nphysics::object::BodyStatus, PhysicsBodyBuilder, PhysicsColliderBuilder,
};

pub fn init_ground(world: &mut World) {
    world
        .create_entity()
        .with(Transform::from(Vector3::new(100.0, 20.0, 0.0)))
        .with(PhysicsBodyBuilder::<f32>::from(BodyStatus::Static).build())
        .with(
            PhysicsColliderBuilder::<f32>::from(Shape::Cuboid {
                half_extents: Vector2::new(100.0, 5.0),
            })
            .build(),
        )
        .build();
}
