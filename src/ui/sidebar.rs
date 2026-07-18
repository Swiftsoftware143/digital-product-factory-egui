//! Sidebar â€” Navigation

use egui::*;
use crate::app::{DpfApp, Tab};

pub fn show(app: &mut DpfApp, ctx: &Context) {
    SidePanel::left("sidebar")
        .resizable(false)
        .default_width(180.0)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("DPF");
            });

            ui.separator();

            // Main tabs
            ui.label("Main");
            if ui.selectable_label(app.current_tab == Tab::Dashboard, "ðŸ“Š Dashboard").clicked() {
                app.current_tab = Tab::Dashboard;
            }
            if ui.selectable_label(app.current_tab == Tab::Pipeline, "ðŸ“‹ Pipeline").clicked() {
                app.current_tab = Tab::Pipeline;
            }
            if ui.selectable_label(app.current_tab == Tab::Create, "ðŸ› ï¸ Create").clicked() {
                app.current_tab = Tab::Create;
            }
            if ui.selectable_label(app.current_tab == Tab::Research, "ðŸ” Research").clicked() {
                app.current_tab = Tab::Research;
            }
            if ui.selectable_label(app.current_tab == Tab::Templates, "ðŸ“ Templates").clicked() {
                app.current_tab = Tab::Templates;
            }

            ui.separator();
            ui.label("Tools");
            if ui.selectable_label(app.current_tab == Tab::Bundles, "ðŸ“¦ Bundles").clicked() {
                app.current_tab = Tab::Bundles;
            }
            if ui.selectable_label(app.current_tab == Tab::Scheduler, "â° Scheduler").clicked() {
                app.current_tab = Tab::Scheduler;
            }
            if ui.selectable_label(app.current_tab == Tab::Mockup, "ðŸŽ¨ Mockups").clicked() {
                app.current_tab = Tab::Mockup;
            }
            if ui.selectable_label(app.current_tab == Tab::Presets, "ðŸŽ¯ Presets").clicked() {
                app.current_tab = Tab::Presets;
            }
            if ui.selectable_label(app.current_tab == Tab::Contract, "ðŸ“ Contracts").clicked() {
                app.current_tab = Tab::Contract;
            }

            ui.separator();
            ui.label("Business");
            if ui.selectable_label(app.current_tab == Tab::Analytics, "ðŸ“ˆ Analytics").clicked() {
                app.current_tab = Tab::Analytics;
            }
            if ui.selectable_label(app.current_tab == Tab::Publish, "ðŸ“¤ Publish").clicked() {
                app.current_tab = Tab::Publish;
            }

            ui.separator();
            if ui.selectable_label(app.current_tab == Tab::Admin, "ðŸ›¡ï¸ Admin").clicked() {
                app.current_tab = Tab::Admin;
                app.admin.admin_mode = true;
            }
        });
}
