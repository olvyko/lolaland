use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum PlayerState {
    Idling,
    Jumping,
    Running,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Player {
    pub state: PlayerState,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            state: PlayerState::Idling,
        }
    }
}
