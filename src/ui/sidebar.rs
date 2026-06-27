//! Fast sidebar navigation

use egui::*;
use crate::app::{DpfApp, Tab};

pub fn show(app: &mut DpfApp, ctx: &Context) {
    let width = if app.sidebar_expanded { 200.0 } else { 50.0 };
    
    SidePanel::left("sidebar")
        .exact_width(width)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                // Toggle button
                if ui.button(if app.sidebar_expanded { "◀" } else { "▶" }).clicked() {
                    app.sidebar_expanded = !app.sidebar_expanded;
                }
                
                ui.separator();
                
                // Navigation items
                nav_button(app, ui, Tab::Dashboard, "📊", "Dashboard");
                nav_button(app, ui, Tab::Pipeline, "🔄", "Pipeline");
                nav_button(app, ui, Tab::Create, "➕", "Create");
                nav_button(app, ui, Tab::Research, "🔍", "Research");
                nav_button(app, ui, Tab::Templates, "📋", "Templates");
                nav_button(app, ui, Tab::Bundles, "📦", "Bundles");
                nav_button(app, ui, Tab::Scheduler, "⏰", "Scheduler");
                
                ui.separator();
                
                nav_button(app, ui, Tab::Settings, "⚙", "Settings");
            });
        });
}

fn nav_button(app: &mut DpfApp, ui: &mut Ui, tab: Tab, icon: &str, label: &str) {
    let selected = app.current_tab == tab;
    
    let response = if app.sidebar_expanded {
        ui.selectable_label(selected, format!("{} {}", icon, label))
    } else {
        ui.selectable_label(selected, icon)
    };
    
    if response.clicked() {
        app.current_tab = tab;
    }
}
