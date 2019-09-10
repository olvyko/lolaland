use amethyst::{
    assets::*,
    core::{
        math::{Isometry3, Vector3},
        Transform,
    },
    ecs::prelude::*,
    prelude::Builder,
};

use specs_physics::{
    colliders::Shape, nphysics::object::BodyStatus, PhysicsBodyBuilder, PhysicsColliderBuilder,
};

pub fn init_panel(world: &mut World) {
    world
        .create_entity()
        .with(Transform::from(Vector3::new(50.0, 20.0, 0.0)))
        .with(PhysicsBodyBuilder::<f32>::from(BodyStatus::Static).build())
        .with(
            PhysicsColliderBuilder::<f32>::from(Shape::Cuboid {
                half_extents: Vector3::new(20.0, 20.0, 1.0),
            })
            .offset_from_parent(Isometry3::translation(50.0, 20.0, 1.0))
            .build(),
        )
        .build();
}
