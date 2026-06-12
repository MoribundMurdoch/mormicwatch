use eframe::egui;
use egui_plot::{
    Line,
    Plot,
    PlotPoints,
};

pub fn draw_history_graph(
    ui: &mut egui::Ui,
    history: impl Iterator<Item = f32>,
) {
    let points: PlotPoints<'_> =
        history
            .enumerate()
            .map(|(i, db)| {
                [i as f64, db as f64]
            })
            .collect();

    let line =
        Line::new("Microphone", points);

    Plot::new("audio_history")
        .height(150.0)
        .show(ui, |plot_ui| {
            plot_ui.line(line);
        });
}