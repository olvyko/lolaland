mod shape;
mod velocity;
mod player;

use amethyst::ecs::prelude::World;

pub use self::shape::Shape;
pub use self::velocity::Velocity;
pub use self::player::Player;

// Register all components to the world
pub fn register_components(world: &mut World) {
    //world.register::<Shape>();
    //world.register::<Velocity>();
    world.register::<Player>();
}
