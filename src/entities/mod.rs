mod camera;
mod player;

use amethyst::ecs::prelude::World;

pub fn init_entities(world: &mut World) {
    player::init_player(world);
    camera::init_camera(world);
}
