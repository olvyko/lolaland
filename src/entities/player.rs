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
    colliders::Shape,
    nphysics::{algebra::Velocity3, object::BodyStatus},
    PhysicsBodyBuilder, PhysicsColliderBuilder,
};

use crate::components::Player;
use crate::resources::AnimationPrefabData;

pub fn init_player(world: &mut World) {
    // Starts asset loading
    let prefab_handle = world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {
        loader.load("prefabs/lola_spritesheet.ron", RonFormat, ())
    });

    world
        .create_entity()
        .with(Player::new(32.0, 32.0, 1.0))
        .with(Transform::from(Vector3::new(50.0, 50.0, 0.0)))
        .with(
            PhysicsBodyBuilder::<f32>::from(BodyStatus::Dynamic)
                .velocity(Velocity3::linear(0.0, -4.0, 0.0))
                .build(),
        )
        .with(
            PhysicsColliderBuilder::<f32>::from(Shape::Cuboid {
                half_extents: Vector3::new(32.0, 32.0, 1.0),
            })
            .offset_from_parent(Isometry3::translation(50.0, 50.0, 1.0))
            .build(),
        )
        .with(prefab_handle)
        .build();
}
