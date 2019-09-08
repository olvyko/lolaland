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

use crate::components::Player;
use crate::resources::AnimationPrefabData;

pub fn init_player(world: &mut World) {
    // Starts asset loading
    let prefab_handle = world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {
        loader.load("prefabs/lola_spritesheet.ron", RonFormat, ())
    });

    let transform = Transform::from(Vector3::new(50.0, 50.0, 0.0));
    let player = Player::new(32.0, 32.0, 1.0);

    world
        .create_entity()
        .with(player.clone())
        .with(transform)
        .with(
            PhysicsBodyBuilder::<f32>::from(BodyStatus::Dynamic)
                .gravity_enabled(true)
                .build(),
        )
        .with(
            PhysicsColliderBuilder::<f32>::from(Shape::Cuboid {
                half_extents: Vector3::new(player.width, player.height, 1.0),
            })
            .build(),
        )
        .with(prefab_handle)
        .build();
}
