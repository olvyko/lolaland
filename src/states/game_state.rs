use amethyst::{
    animation::*, assets::*, core::*, ecs::*, renderer::*, GameData, SimpleState, SimpleTrans,
    StateData, Trans,
};

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
}
