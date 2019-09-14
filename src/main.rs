#[macro_use]
extern crate log;
#[macro_use]
extern crate specs_derive;

mod components;
mod entities;
mod resources;
mod states;
mod systems;

use self::components::AnimationId;
use self::states::LoadState;
use self::systems::{GameBundle, PhysicsBundle};

use amethyst::{
    animation::AnimationBundle,
    core::transform::bundle::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle, SpriteRender,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

// Dark gray
const BACKGROUND_COLOR: [i32; 4] = [100, 100, 100, 255];

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("assets\\configs\\display.ron");
    let input_bindings_path = app_root.join("assets\\configs\\input.ron");
    let assets_path = app_root.join("assets\\");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&input_bindings_path)?,
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear(BACKGROUND_COLOR),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderDebugLines::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
            "animation_control_system",
            "sampler_interpolation_system",
        ))?
        .with_bundle(PhysicsBundle::default().with_debug_lines())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(GameBundle)?;

    let mut game = Application::new(assets_path, LoadState::default(), game_data)?;

    Ok(game.run())
}
