use amethyst::{
    assets::*,
    core::{
        math::{Vector2, Vector3},
        Transform,
    },
    ecs::prelude::*,
    prelude::Builder,
};

use specs_physics::{
    colliders::Shape, nphysics::object::BodyStatus, PhysicsBodyBuilder, PhysicsColliderBuilder,
};

use crate::components::Player;
use crate::resources::AnimationPrefabData;

pub fn init_player(world: &mut World) {
    let prefab_handle = world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {
        loader.load("prefabs/lola_spritesheet.ron", RonFormat, ())
    });

    world
        .create_entity()
        .with(Player::new(32.0, 32.0, 70.0))
        .with(Transform::from(Vector3::new(50.0, 50.0, 0.0)))
        .with(
            PhysicsBodyBuilder::<f32>::from(BodyStatus::Dynamic)
                .gravity_enabled(true)
                .mass(16.0)
                .build(),
        )
        .with(
            PhysicsColliderBuilder::<f32>::from(Shape::Cuboid {
                half_extents: Vector2::new(16.0, 16.0),
            })
            .build(),
        )
        .with(prefab_handle)
        .build();
}
