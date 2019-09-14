use amethyst::{
    core::Transform,
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Lola, Subject},
    resources::Context,
};

pub struct CameraTransformSystem;

impl<'s> System<'s> for CameraTransformSystem {
    type SystemData = (
        ReadStorage<'s, Lola>,
        ReadStorage<'s, Subject>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, (lolas, subject_tags, mut transforms, ctx): Self::SystemData) {
        let mut lola_x = 0.0;

        for (_, transform) in (&lolas, &transforms).join() {
            lola_x = transform.translation().x;
            //println!("{:?}", lola_x);
        }

        for (_, transform) in (&subject_tags, &mut transforms).join() {
            transform.set_translation_x(lola_x / 2.0);
        }
    }
}
