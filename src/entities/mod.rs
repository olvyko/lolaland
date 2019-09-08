mod camera;
mod panel;
mod player;

use amethyst::ecs::prelude::World;

pub fn init_entities(world: &mut World) {
    panel::init_panel(world);
    player::init_player(world);
    camera::init_camera(world);
}
