use amethyst::{
    ecs::{prelude::World, Entity},
    prelude::{Builder, WorldExt},
};

use crate::components::{Player, Position, Rect};

pub fn create_player(world: &mut World) -> Entity {
    let rect = Rect::from_size(16.0, 16.0);
    let position = Position::from_coords(100.0, 100.0);
    world
        .create_entity()
        .with(Player::default())
        .with(position)
        .with(rect)
        .build()
}
