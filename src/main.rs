mod components;
mod entities;
mod resources;
mod states;
mod systems;

use self::resources::{AnimationId, AnimationPrefabData};
use self::states::GameState;
use self::systems::GameBundle;

use amethyst::{
    animation::AnimationBundle,
    assets::PrefabLoaderSystem,
    core::transform::bundle::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, SpriteRender, Stage, ALPHA,
    },
    ui::{DrawUi, UiBundle},
    LogLevelFilter,
};

// Dark gray
const BACKGROUND_COLOR: [f32; 4] = [0.01, 0.01, 0.01, 0.0];

pub fn run() -> Result<(), amethyst::Error> {
    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("assets/configs/display.ron");
    let input_bindings_path = app_root.join("assets/configs/input.ron");
    let assets_path = app_root.join("assets/");
    let display_config = DisplayConfig::load(&display_config_path);

    let pipe = {
        Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target(BACKGROUND_COLOR, 1.0)
                .with_pass(
                    DrawFlat2D::new(), //.with_transparency_settings(
                                       //    ColorMask::all(),
                                       //    ALPHA,
                                       //    None,)
                )
                .with_pass(DrawUi::new()),
        )
    };

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&input_bindings_path)?,
        )?
        .with_bundle(GameBundle)?
        .with_bundle(RenderBundle::new(pipe, Some(display_config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
            "animation_control_system",
            "sampler_interpolation_system",
        ))?
        .with(
            PrefabLoaderSystem::<AnimationPrefabData>::default(),
            "prefab_loader_system",
            &[],
        )
        .with_bundle(UiBundle::<String, String>::new())?;

    let mut game = Application::new(assets_path, GameState, game_data)?;

    Ok(game.run())
}

fn main() {
    amethyst::start_logger(amethyst::LoggerConfig {
        stdout: amethyst::StdoutLog::Colored,
        level_filter: LogLevelFilter::Info,
        log_file: None,
        allow_env_override: true,
        log_gfx_device_level: Some(LogLevelFilter::Warn),
    });

    if let Err(e) = run() {
        println!("Error occurred during game execution: {}", e);
        pause();
        std::process::exit(1);
    }
}

use std::{env, io, path};
fn application_root_dir() -> Result<path::PathBuf, io::Error> {
    if let Some(manifest_dir) = env::var_os("CARGO_MANIFEST_DIR") {
        return Ok(path::PathBuf::from(manifest_dir));
    }

    let mut exe = env::current_exe()?;

    // Modify in-place to avoid an extra copy.
    if exe.pop() {
        return Ok(exe);
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "Failed to find an application root",
    ))
}

fn pause() {
    use std::io;
    use std::io::prelude::*;

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
