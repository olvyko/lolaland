use amethyst::assets::*;
use amethyst::core::math::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::*;
use amethyst::prelude::Builder;

use crate::resources::AnimationPrefabData;

const PLAYER_WIDTH: f32 = 32.0;
const PLAYER_HEIGHT: f32 = 32.0;

pub fn init_player(world: &mut World, progress_counter: &mut ProgressCounter) {
   // Starts asset loading
   let prefab_handle = world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {
      loader.load("prefabs/sprite_sheet.ron", RonFormat, (), progress_counter)
   });

   let mut sprite_transform = Transform::default();
   sprite_transform.set_scale(PLAYER_WIDTH, PLAYER_HEIGHT, 0.);
   sprite_transform.set_translation(Vector3::new(100., 100., 0.));

   // Creates new entities with components from MyPrefabData
   world
      .create_entity()
      .with(prefab_handle)
      .with(sprite_transform)
      .build();
}
