mod player;

use amethyst::ecs::prelude::World;

pub use self::player::Player;

// Register all components to the world
pub fn register_components(world: &mut World) {
    world.register::<Player>();
}
