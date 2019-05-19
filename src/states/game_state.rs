use amethyst::{GameData, SimpleState, StateData};

use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    ecs::{Entities, Join, ReadStorage, WriteStorage},
    renderer::SpriteRender,
    SimpleTrans, Trans,
};

use crate::resources::AnimationId;

use crate::components::register_components;
use crate::entities::init_entities;
use crate::resources::add_resources;

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = state_data;

        register_components(world);
        add_resources(world);
        init_entities(world);
    }

    // fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    //     let StateData { world, .. } = data;
    //     // Execute a pass similar to a system
    //     world.exec(
    //         |(entities, animation_sets, mut control_sets): (
    //             Entities,
    //             ReadStorage<AnimationSet<AnimationId, SpriteRender>>,
    //             WriteStorage<AnimationControlSet<AnimationId, SpriteRender>>,
    //         )| {
    //             // For each entity that has AnimationSet
    //             for (entity, animation_set) in (&entities, &animation_sets).join() {
    //                 // Creates a new AnimationControlSet for the entity
    //                 let control_set = get_animation_set(&mut control_sets, entity).unwrap();
    //                 control_set.add_animation(
    //                     AnimationId::Idle,
    //                     &animation_set.get(&AnimationId::Idle).unwrap(),
    //                     EndControl::Loop(None),
    //                     1.0,
    //                     AnimationCommand::Start,
    //                 );
    //             }
    //         },
    //     );

    //     Trans::None
    // }
}
