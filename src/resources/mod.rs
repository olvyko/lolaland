mod animation;

use amethyst::ecs::prelude::World;

use specs_physics::{
    colliders::Shape,
    nalgebra::Vector2,
    nphysics::{algebra::Velocity3, object::BodyStatus},
    parameters::Gravity,
    PhysicsBody, PhysicsBodyBuilder, PhysicsColliderBuilder,
};

pub use self::animation::{AnimationId, AnimationPrefabData};

// Add all resources needed at the start to the world
pub fn add_resources(world: &mut World) {
    world.add_resource(Gravity::<f32>(Vector2::<f32>::new(0.0, -2500.0)));
}
