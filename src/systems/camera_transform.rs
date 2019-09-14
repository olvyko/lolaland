use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Lola, Subject};

pub struct CameraTransformSystem;

impl<'s> System<'s> for CameraTransformSystem {
    type SystemData = (
        ReadStorage<'s, Lola>,
        ReadStorage<'s, Subject>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (lolas, subject_tags, mut transforms): Self::SystemData) {
        let mut lola_x = 0.0;
        let mut lola_y = 0.0;

        for (_, transform) in (&lolas, &transforms).join() {
            lola_x = transform.translation().x;
            lola_y = transform.translation().y;
        }

        for (_, transform) in (&subject_tags, &mut transforms).join() {
            transform.set_translation_x(lola_x / 2.0);
            transform.set_translation_y(lola_y / 2.0);
        }
    }
}
