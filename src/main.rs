mod app;
mod content;
mod game;
mod persistence;

use app::SignalZeroApp;
use content::Story;

fn main() -> eframe::Result<()> {
    let story = Story::load_embedded().expect("Embedded story content must be valid");
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 780.0])
            .with_min_inner_size([1024.0, 640.0])
            .with_title("Rust Laboratory: Signal Zero"),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Laboratory: Signal Zero",
        options,
        Box::new(move |creation_context| Box::new(SignalZeroApp::new(creation_context, story))),
    )
}
