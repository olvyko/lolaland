use amethyst::{
    core::ArcThreadPool,
    ecs::prelude::{Dispatcher, DispatcherBuilder},
    GameData, SimpleState, SimpleTrans, StateData, Trans,
};

use crate::components::register_components;
use crate::entities::init_entities;
use crate::resources::add_resources;

#[derive(Default)]
pub struct GameState<'a, 'b> {
    fixed_dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for GameState<'a, 'b> {
    fn on_start(&mut self, state_data: StateData<GameData>) {
        let world = state_data.world;

        //let mut builder = DispatcherBuilder::new().with_pool(world.res.fetch::<ArcThreadPool>().clone());

        register_components(world);
        add_resources(world);
        init_entities(world);
    }

    fn fixed_update(&mut self, data: StateData<GameData>) -> SimpleTrans {
        if let Some(dispatcher) = &mut self.fixed_dispatcher {
            //dispatcher.dispatch(&data.world.res);
        }
        Trans::None
    }
}
