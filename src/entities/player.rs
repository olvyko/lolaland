use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
    core::Transform,
    ecs::prelude::*,
    prelude::Builder,
    renderer::{
        FilterMethod, Flipped, PngFormat, SpriteGrid, SpriteRender, SpriteSheet, SpriteSheetFormat,
        Sprites, Texture, TextureMetadata,
    },
};

pub fn init_player(world: &mut World) {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/sprite_sheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "textures/sprite_sheet.ron",
            SpriteSheetFormat,
            texture_handle,
            (),
            &sprite_sheet_store,
        )
    };

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    let mut sprite_transform = Transform::default();
    sprite_transform.set_translation_xyz(60.0, 60.0, 0.);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(sprite_transform)
        .build();
}
