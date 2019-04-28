mod camera;
mod player;

use amethyst::assets::ProgressCounter;
use amethyst::ecs::prelude::World;

pub fn init_entities(world: &mut World, progress_counter: &mut ProgressCounter) {
   player::init_player(world, progress_counter);
   camera::init_camera(world);
}
