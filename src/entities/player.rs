use amethyst::{assets::*, core::Transform, ecs::prelude::*, prelude::Builder};

use crate::resources::AnimationPrefabData;

pub fn init_player(world: &mut World) {
    // Starts asset loading
    let prefab_handle = world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {
        loader.load("prefabs/sprite_sheet.ron", RonFormat, (), ())
    });

    let mut sprite_transform = Transform::default();
    sprite_transform.set_translation_xyz(100., 100., 0.);

    // sprite_transform.set

    world
        .create_entity()
        .with(prefab_handle)
        .with(sprite_transform)
        .build();
}
