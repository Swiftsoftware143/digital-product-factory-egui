//! Main content area - routes to current tab

use egui::*;
use crate::app::{DpfApp, Tab};
use super::pipeline_view;

pub fn show(app: &mut DpfApp, ctx: &Context) {
    match app.current_tab {
        Tab::Dashboard => show_dashboard(app, ctx),
        Tab::Pipeline => pipeline_view::show(app, ctx),
        Tab::Create => show_create(app, ctx),
        Tab::Research => show_research(app, ctx),
        Tab::Templates => show_templates(app, ctx),
        Tab::Bundles => show_bundles(app, ctx),
        Tab::Scheduler => show_scheduler(app, ctx),
        Tab::Presets => super::presets_view::show(app, ctx),
        Tab::Settings => show_settings(app, ctx),
    }
}

fn show_dashboard(app: &mut DpfApp, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Dashboard");
        ui.separator();
        
        // Quick stats
        ui.horizontal(|ui| {
            stat_card(ui, "Total Ideas", &app.pipeline.ideas.len().to_string());
            stat_card(ui, "In Progress", &app.pipeline.ideas_by_stage(crate::pipeline::PipelineStage::Creating).len().to_string());
            stat_card(ui, "Selling", &app.pipeline.ideas_by_stage(crate::pipeline::PipelineStage::Selling).len().to_string());
        });
        
        ui.separator();
        
        // Recent activity
        ui.label("Recent Activity");
        // TODO: Show recent actions
    });
}

fn stat_card(ui: &mut Ui, label: &str, value: &str) {
    Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(150.0);
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(value).size(32.0).strong());
            ui.label(label);
        });
    });
}

fn show_create(app: &mut DpfApp, ctx: &Context) {
    super::create_view::show(app, ctx);
}

fn show_research(app: &mut DpfApp, ctx: &Context) {
    super::research_view::show(app, ctx);
}

fn show_templates(app: &mut DpfApp, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Templates");
        ui.label("Browse and manage product templates");
        // TODO: Implement template browser
    });
}

fn show_bundles(app: &mut DpfApp, ctx: &Context) {
    super::bundle_view::show(app, ctx);
}

fn show_scheduler(app: &mut DpfApp, ctx: &Context) {
    super::scheduler_view::show(app, ctx);
}

fn show_settings(app: &mut DpfApp, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Settings");
        
        ui.group(|ui| {
            ui.label("API Keys");
            ui.text_edit_singleline(&mut app.config.openai_key)
                .hint_text("OpenAI API Key");
            ui.text_edit_singleline(&mut app.config.anthropic_key)
                .hint_text("Anthropic API Key");
        });
        
        ui.group(|ui| {
            ui.label("Preferences");
            ui.checkbox(&mut app.config.auto_save, "Auto-save");
            ui.checkbox(&mut app.config.dark_mode, "Dark mode");
        });
    });
}
