mod bundle;
mod components;
mod entities;
mod resources;
mod states;
mod systems;

use self::bundle::GameBundle;
use self::resources::{AnimationId, AnimationPrefabData};
use self::states::GameState;

use amethyst::{
   animation::AnimationBundle,
   assets::PrefabLoaderSystem,
   core::transform::bundle::TransformBundle,
   input::InputBundle,
   prelude::*,
   renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, SpriteRender, Stage},
   ui::{DrawUi, UiBundle},
   utils::application_root_dir,
   LogLevelFilter,
};

// Dark gray
const BACKGROUND_COLOR: [f32; 4] = [0.01, 0.01, 0.01, 0.0];

pub fn run() -> Result<(), amethyst::Error> {
   let app_root = application_root_dir()?;

   let display_config_path = app_root.join("resources/configs/display.ron");
   let display_config = DisplayConfig::load(&display_config_path);
   let input_bindings_path = app_root.join("resources/configs/input_bindings.ron");
   let assets_path = app_root.join("resources/");

   let pipe = {
      Pipeline::build().with_stage(
         Stage::with_backbuffer()
            .clear_target(BACKGROUND_COLOR, 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawUi::new()),
      )
   };

   let game_data = GameDataBuilder::default()
      .with(
         PrefabLoaderSystem::<AnimationPrefabData>::default(),
         "prefab_loader_system",
         &[],
      )
      .with_bundle(
         InputBundle::<String, String>::new().with_bindings_from_file(&input_bindings_path)?,
      )?
      .with_bundle(GameBundle)?
      .with_bundle(TransformBundle::new())?
      .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
         "animation_control_system",
         "sampler_interpolation_system",
      ))?
      .with_bundle(UiBundle::<String, String>::new())?
      .with_bundle(RenderBundle::new(pipe, Some(display_config)).with_sprite_sheet_processor())?;

   let mut game = Application::new(assets_path, GameState::default(), game_data)?;

   Ok(game.run())
}

fn main() {
   amethyst::start_logger(amethyst::LoggerConfig {
      stdout: amethyst::StdoutLog::Colored,
      level_filter: LogLevelFilter::Debug,
      log_file: None,
      allow_env_override: true,
      log_gfx_device_level: Some(LogLevelFilter::Warn),
   });

   if let Err(e) = run() {
      println!("Error occurred during game execution: {}", e);
      std::process::exit(1);
   }
}
