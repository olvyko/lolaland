use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    core::{
        math::{Vector2, Vector3},
        Transform, WithNamed,
    },
    ecs::{prelude::World, VecStorage},
    error::Error,
    prelude::Builder,
    renderer::{sprite::SpriteRender, transparent::Transparent},
};

use specs_physics::{
    colliders::Shape, nphysics::object::BodyStatus, PhysicsBodyBuilder, PhysicsColliderBuilder,
};

use serde::{Deserialize, Serialize};

use crate::resources::{AssetType, Context, SpriteSheetList};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tilewidth: i32,
    pub tileheight: i32,
}

impl Map {
    pub fn load_layers(&self, world: &mut World, ctx: &Context) {
        world
            .create_entity()
            .with(Transform::from(Vector3::new(100.0, 20.0, 0.0)))
            .with(PhysicsBodyBuilder::<f32>::from(BodyStatus::Static).build())
            .with(
                PhysicsColliderBuilder::<f32>::from(Shape::Cuboid {
                    half_extents: Vector2::new(90.0, 5.0),
                })
                .build(),
            )
            .build();

        let mut transform = Transform::from(Vector3::new(195.0, 98.0, 0.0));
        transform.set_rotation_2d(-0.2);
        world
            .create_entity()
            .with(transform)
            .with(PhysicsBodyBuilder::<f32>::from(BodyStatus::Static).build())
            .with(
                PhysicsColliderBuilder::<f32>::from(Shape::Cuboid {
                    half_extents: Vector2::new(5.0, 90.0),
                })
                .build(),
            )
            .build();

        let mut transform = Transform::from(Vector3::new(5.0, 98.0, 0.0));
        transform.set_rotation_2d(0.2);
        world
            .create_entity()
            .with(transform)
            .with(PhysicsBodyBuilder::<f32>::from(BodyStatus::Static).build())
            .with(
                PhysicsColliderBuilder::<f32>::from(Shape::Cuboid {
                    half_extents: Vector2::new(5.0, 90.0),
                })
                .build(),
            )
            .build();
    }
}
