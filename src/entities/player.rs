use amethyst::{
    assets::*,
    core::{
        math::{Vector2, Vector3},
        Transform,
    },
    ecs::prelude::*,
    prelude::Builder,
};

use specs_physics::{
    colliders::Shape,
    nphysics::{algebra::Velocity3, object::BodyStatus},
    PhysicsBodyBuilder, PhysicsColliderBuilder,
};

use crate::components::Player;
use crate::resources::AnimationPrefabData;

pub fn init_player(world: &mut World) {
    // Starts asset loading
    let prefab_handle = world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {
        loader.load("prefabs/lola_spritesheet.ron", RonFormat, ())
    });

    let mut transform = Transform::from(Vector3::new(50.0, 50.0, 0.0));

    world
        .create_entity()
        .with(Player::new(32.0, 32.0, 70.0))
        .with(transform)
        .with(
            PhysicsBodyBuilder::<f32>::from(BodyStatus::Dynamic)
                .gravity_enabled(true)
                .mass(20.0)
                .build(),
        )
        .with(
            PhysicsColliderBuilder::<f32>::from(Shape::Cuboid {
                half_extents: Vector2::new(16.0, 16.0),
            })
            .density(1.0)
            .build(),
        )
        .with(prefab_handle)
        .build();
}

/*
pub trait Position<N: RealField>:
    Component<Storage = FlaggedStorage<Self, DenseVecStorage<Self>>> + Send + Sync
{
    fn isometry(&self) -> Isometry3<N>;
    fn set_isometry(&mut self, isometry: &Isometry3<N>) -> &mut Self;
}

#[cfg(feature = "amethyst")]
impl Position<f32> for amethyst_core::Transform {
    fn isometry(&self) -> Isometry3<f32> {
        *self.isometry()
    }

    fn set_isometry(&mut self, isometry: &Isometry3<f32>) -> &mut Self {
        self.set_isometry(*isometry)
    }
}
*/
