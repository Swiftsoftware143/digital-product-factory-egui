//! Digital Product Factory - Pure Rust Native Desktop App

mod app;
mod pipeline;
mod product_generator;
mod license_manager;
mod templates;
mod llm_router;
mod research;
mod scheduler;
mod bundler;
mod exporter;
mod contract_generator;
mod database;
mod config;
mod presets;
mod ui;
pub mod mockup_compositor;
pub mod analytics;
pub mod publishing;
pub mod db_ext;
pub mod inline_help;
pub mod admin;
pub mod qc;
pub mod webhook;
pub mod asset_library;
pub mod product_variants;
pub mod compliance;

use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
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