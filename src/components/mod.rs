mod shape;
mod velocity;

use amethyst::ecs::prelude::World;

pub use self::shape::Shape;
pub use self::velocity::Velocity;

// Register all components to the world
pub fn register_components(world: &mut World) {
   world.register::<Shape>();
   world.register::<Velocity>();
}
