use amethyst::{
    core::Transform,
    ecs::{prelude::World, Entity},
    prelude::{Builder, WorldExt},
};

use crate::{components::Lola, resources::Context};

pub fn load_lola(world: &mut World, ctx: &Context) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_x(50.0);
    transform.set_translation_y(50.0);
    world.create_entity().with(Lola::default()).with(transform).build()
}
