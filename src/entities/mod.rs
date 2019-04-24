mod camera;
mod player;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::ecs::prelude::World;
use amethyst::renderer::{PngFormat, Texture, TextureHandle, TextureMetadata};

// Init all entities to the world
pub fn init_entities(world: &mut World) {
    player::init_player(world);
    camera::init_camera(world);
}

pub fn load_texture<N>(name: N, world: &World) -> TextureHandle
where
    N: Into<String>,
{
    let loader = world.read_resource::<Loader>();
    loader.load(
        name,
        PngFormat,
        TextureMetadata::srgb_scale(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    )
}
