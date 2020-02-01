use crate::components::{Player, Position, Subject};
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};

pub struct CameraTransformSystem;

impl<'s> System<'s> for CameraTransformSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Subject>,
        WriteStorage<'s, Position>,
    );

    fn run(&mut self, (players, subject_tags, mut positions): Self::SystemData) {
        let mut player_x = 0.0;
        let mut player_y = 0.0;
        for (_, position) in (&players, &positions).join() {
            player_x = position.x;
            player_y = position.y;
        }
        for (_, position) in (&subject_tags, &mut positions).join() {
            position.x = player_x / 2.0;
            position.y = player_y / 2.0;
        }
    }
}
