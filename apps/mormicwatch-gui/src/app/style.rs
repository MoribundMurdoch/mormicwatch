use eframe::egui::{
    Context,
    Visuals,
};

pub fn apply_style(
    ctx: &Context,
) {
    let mut visuals =
        Visuals::dark();

    visuals.window_fill =
        crate::app::theme::BG;

    ctx.set_visuals(visuals);
}
