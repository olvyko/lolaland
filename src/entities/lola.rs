use amethyst::{
    assets::{Handle, Prefab},
    core::{
        math::{Vector2, Vector3},
        Transform,
    },
    ecs::{prelude::World, Entity},
    prelude::Builder,
};

use specs_physics::{
    colliders::Shape, nphysics::object::BodyStatus, PhysicsBodyBuilder, PhysicsColliderBuilder,
};

use crate::{
    components::{Animation, AnimationId, AnimationPrefabData, Lola, Motion},
    resources::Context,
};

pub fn load_lola(
    world: &mut World,
    prefab: Handle<Prefab<AnimationPrefabData>>,
    ctx: &Context,
) -> Entity {
    let mut transform = Transform::default();
    //transform.set_scale(Vector3::new(2.0, 2.0, 2.0));
    transform.set_translation_x(50.0);
    transform.set_translation_y(50.0);

    let mut motion = Motion::new();
    motion.velocity.x = 60.0;

    world
        .create_entity()
        .with(Lola::new())
        .with(transform)
        .with(motion)
        .with(
            PhysicsBodyBuilder::<f32>::from(BodyStatus::Dynamic)
                .gravity_enabled(true)
                .mass(16.0)
                .build(),
        )
        .with(
            PhysicsColliderBuilder::<f32>::from(Shape::Cuboid {
                half_extents: Vector2::new(16.0, 16.0),
            })
            .build(),
        )
        .with(Animation::new(
            AnimationId::Idle,
            vec![AnimationId::Idle, AnimationId::Jump, AnimationId::Walk],
        ))
        .with(prefab)
        .build()
}
