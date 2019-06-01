use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::SpriteRender,
};

use crate::resources::AnimationId;

pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, AnimationSet<AnimationId, SpriteRender>>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (entities, animation_sets, mut control_sets, input): Self::SystemData) {
        // if pos == neg { 0.0 } else if pos { 1.0 } else { -1.0 }
        let x_move = input.axis_value("x_move").unwrap();

        for (entity, animation_set) in (&entities, &animation_sets).join() {
            let control_set = get_animation_set(&mut control_sets, entity).unwrap();

            if x_move > 0.0 {
                control_set.add_animation(
                    AnimationId::Walk,
                    &animation_set.get(&AnimationId::Walk).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Start,
                );
            } else if x_move < 0.0 {
                control_set.add_animation(
                    AnimationId::Walk,
                    &animation_set.get(&AnimationId::Walk).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Start,
                );
            } else {
                control_set.abort(AnimationId::Walk);
                control_set.add_animation(
                    AnimationId::Idle,
                    &animation_set.get(&AnimationId::Idle).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Start,
                );
            }
        }
    }
}
