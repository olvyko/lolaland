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
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle, SpriteRender,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    LogLevelFilter,
};

// Dark gray
const BACKGROUND_COLOR: [f32; 4] = [0.01, 0.01, 0.01, 1.0];

pub fn run() -> Result<(), amethyst::Error> {
    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("assets/configs/display.ron");
    let input_bindings_path = app_root.join("assets/configs/input.ron");
    let assets_path = app_root.join("assets/");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&input_bindings_path)?,
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and
                // drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear(BACKGROUND_COLOR),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
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
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(GameBundle)?;

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

fn pause() {
    use std::io;
    use std::io::prelude::*;

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
