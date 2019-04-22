extern crate amethyst;

mod bundle;
mod components;
mod entities;
mod resources;
mod states;
mod systems;

use crate::bundle::GameBundle;
use crate::states::GameState;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA},
    ui::{DrawUi, UiBundle},
    utils::application_root_dir,
};

const BACKGROUND_COLOUR: [f32; 4] = [0.01, 0.01, 0.01, 0.0]; // dark grey

pub fn run() -> Result<(), amethyst::Error> {
    let app_root = application_root_dir();

    let display_config_path = format!("{}/resources/display_config.ron", app_root);
    let display_config = DisplayConfig::load(&display_config_path);
    let input_bindings_path = format!("{}/resources/input_bindings.ron", app_root);
    let assets_path = format!("{}/assets", app_root);

    let pipe = {
        Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target(BACKGROUND_COLOUR, 1.0)
                .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None))
                .with_pass(DrawUi::new()),
        )
    };

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&input_bindings_path)?,
        )?
        .with_bundle(GameBundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(RenderBundle::new(pipe, Some(display_config)).with_sprite_sheet_processor())?;

    let mut game = Application::new(assets_path, GameState, game_data)?;

    Ok(game.run())
}

fn main() {
    amethyst::start_logger(Default::default());
    if let Err(e) = run() {
        println!("Error occurred during game execution: {}", e);
        ::std::process::exit(1);
    }
}
