#[derive(Clone, Copy, Default)]
pub struct Context {
    pub map_width: f32,
    pub bg_width: f32,
    pub bg_height: f32,
    pub x_correction: f32,
    pub y_correction: f32,
    pub bg_z_translation: f32,
    pub scale: f32,
}

impl Context {
    pub fn new() -> Self {
        let mut context = Context {
            map_width: 1000.0,
            bg_width: 200.0,
            bg_height: 200.0,
            x_correction: 0.0,
            y_correction: 0.0,
            bg_z_translation: -50.0,
            scale: 1.0,
        };

        context.x_correction = -(600.0 / 2.0 - context.bg_width); // (screen_width / 2.0 - background_width) * -1
        context.y_correction = -(context.bg_height / 2.0); // (background_height / 2.0) * -1.
        context
    }
}
