use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    core::Transform,
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::SpriteRender,
};

use std::f32;

use crate::resources::AnimationId;

pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, AnimationSet<AnimationId, SpriteRender>>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(
        &mut self,
        (entities, mut transforms, animation_sets, mut control_sets, input): Self::SystemData,
    ) {
        // if pos == neg { 0.0 } else if pos { 1.0 } else { -1.0 }
        let x_move = input.axis_value("x_move").unwrap();
        let jump_input = input.action_is_down("jump").unwrap();

        for (entity, animation_set, transform) in
            (&entities, &animation_sets, &mut transforms).join()
        {
            let control_set = get_animation_set(&mut control_sets, entity).unwrap();

            if x_move > 0.0 {
                control_set.add_animation(
                    AnimationId::Walk,
                    &animation_set.get(&AnimationId::Walk).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Start,
                );
                transform.set_rotation_y_axis(0.0);
            } else if x_move < 0.0 {
                control_set.add_animation(
                    AnimationId::Walk,
                    &animation_set.get(&AnimationId::Walk).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Start,
                );
                transform.set_rotation_y_axis(f32::consts::PI);
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

            if jump_input {
                control_set.add_animation(
                    AnimationId::Jump,
                    &animation_set.get(&AnimationId::Jump).unwrap(),
                    EndControl::Normal,
                    1.0,
                    AnimationCommand::Start,
                );
            }
        }
    }
}
