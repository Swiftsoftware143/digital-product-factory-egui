//! Digital Product Factory - Pure Rust Native Desktop App
//! Built with egui for maximum performance

mod app;
mod pipeline;
mod product_generator;
mod license_manager;
mod template_engine;
mod research;
mod scheduler;
mod bundler;
mod exporter;
mod contract_generator;
mod database;
mod config;
mod ui;

use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
    // Native options for fast startup
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Digital Product Factory"),
        ..Default::default()
    };

    eframe::run_native(
        "Digital Product Factory",
        options,
        Box::new(|cc| Box::new(app::DpfApp::new(cc))),
    )
}
