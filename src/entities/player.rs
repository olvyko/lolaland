use amethyst::{assets::*, core::Transform, ecs::prelude::*, prelude::Builder};

use crate::components::Player;
use crate::resources::AnimationPrefabData;

pub fn init_player(world: &mut World) {
    // Starts asset loading
    let prefab_handle = world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {
        loader.load("prefabs/lola_spritesheet.ron", RonFormat, (), ())
    });

    let mut sprite_transform = Transform::default();
    sprite_transform.set_translation_xyz(50., 50., 0.);

    world
        .create_entity()
        .with(Player::new(32.0, 32.0, 1.0))
        .with(prefab_handle)
        .with(sprite_transform)
        .build();
}
