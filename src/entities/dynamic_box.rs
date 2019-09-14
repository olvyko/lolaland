use amethyst::{
    core::{
        math::{Vector2, Vector3},
        Transform,
    },
    ecs::{prelude::World, Entity},
    prelude::Builder,
    renderer::sprite::SpriteRender,
};

use specs_physics::{
    colliders::Shape, nphysics::object::BodyStatus, PhysicsBodyBuilder, PhysicsColliderBuilder,
};

use crate::{components::DynamicBox, resources::Context};

pub fn load_dynamic_box(
    world: &mut World,
    sprite: SpriteRender,
    ctx: &Context,
    transform: Transform,
    width: f32,
    height: f32,
) -> Entity {
    world
        .create_entity()
        .with(DynamicBox::default())
        .with(transform)
        .with(
            PhysicsBodyBuilder::<f32>::from(BodyStatus::Dynamic)
                .gravity_enabled(true)
                .mass(16.0)
                .build(),
        )
        .with(
            PhysicsColliderBuilder::<f32>::from(Shape::Cuboid {
                half_extents: Vector2::new(width / 2.0, height / 2.0),
            })
            .build(),
        )
        .with(sprite)
        .build()
}
