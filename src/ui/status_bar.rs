//! Status bar at bottom of window

use egui::*;
use crate::app::DpfApp;

pub fn show(app: &mut DpfApp, ctx: &Context) {
    TopBottomPanel::bottom("status_bar")
        .exact_height(24.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Left side - status messages
                ui.label("Ready");
                
                ui.separator();
                
                // Center - current operation
                // TODO: Show current operation if any
                
                // Right side - stats
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(format!("{:.0} FPS", app.fps));
                    ui.separator();
                    
                    // License status
                    let license_status = if app.license_manager.is_licensed() {
                        "✓ Licensed"
                    } else {
                        "⚠ Unlicensed"
                    };
                    if ui.selectable_label(false, license_status).clicked() {
                        app.show_license_dialog = true;
                    }
                });
            });
        });
}
