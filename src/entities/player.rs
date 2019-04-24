use amethyst::core::math::Vector3;
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::{Entity, World};
use amethyst::prelude::Builder;
use amethyst::renderer::{Flipped, ScreenDimensions, SpriteRender, TextureHandle, Transparent};

use super::load_texture;

use crate::components::{Shape, Velocity};

const PLAYER_WIDTH: f32 = 32.0;
const PLAYER_HEIGHT: f32 = 32.0;

pub fn init_player(world: &mut World) {
    let texture = load_texture("texture/lola.png", world);

    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    // Set the scale and position of our player sprite
    let mut sprite_transform = Transform::default();
    sprite_transform.set_translation(Vector3::new(0., 0., 0.));

    world
        .create_entity()
        .with(texture.clone())
        .with(sprite_transform)
        .build();
}
