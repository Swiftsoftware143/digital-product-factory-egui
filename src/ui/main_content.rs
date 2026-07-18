//! Main content area - routes to current tab

use egui::*;
use crate::app::{DpfApp, Tab};
use crate::inline_help;
use super::{pipeline_view, analytics_view, publish_view, mockup_view, admin_view};

pub fn show(app: &mut DpfApp, ctx: &Context) {
    // Show modal dialogs
    if app.show_add_sale_dialog {
        analytics_view::show_add_sale_dialog(app, ctx);
    }

    match app.current_tab {
        Tab::Dashboard => show_dashboard(app, ctx),
        Tab::Pipeline => pipeline_view::show(app, ctx),
        Tab::Create => show_create(app, ctx),
        Tab::Research => show_research(app, ctx),
        Tab::Templates => show_templates(app, ctx),
        Tab::Bundles => show_bundles(app, ctx),
        Tab::Scheduler => show_scheduler(app, ctx),
        Tab::Presets => super::presets_view::show(app, ctx),
        Tab::Contract => show_contract(app, ctx),
        Tab::Analytics => analytics_view::show(app, ctx),
        Tab::Publish => publish_view::show(app, ctx),
        Tab::Mockup => mockup_view::show(app, ctx),
        Tab::Settings => show_settings(app, ctx),
        Tab::Admin => admin_view::show(app, ctx),
    }

    // Help overlay (persistent across tabs)
    if let Some(topic_id) = &app.active_help_topic.clone() {
        if topic_id == "__index__" {
            inline_help::show_help_index(ctx, &mut app.active_help_topic);
        } else {
            inline_help::show_help_popup(ctx, topic_id, &mut app.active_help_topic);
        }
    }
}

fn show_dashboard(app: &mut DpfApp, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Dashboard");
            inline_help::help_button(ui, "dashboard", &mut app.active_help_topic);
        });
        ui.separator();

        ui.horizontal(|ui| {
            stat_card(ui, "Total Ideas", &app.pipeline.ideas.len().to_string());
            stat_card(ui, "In Progress", &app.pipeline.ideas_by_stage(crate::pipeline::PipelineStage::Creating).len().to_string());
            stat_card(ui, "Selling", &app.pipeline.ideas_by_stage(crate::pipeline::PipelineStage::Selling).len().to_string());

            let total_rev: f64 = app.analytics.records.iter().map(|r| r.net_revenue).sum();
            stat_card(ui, "Total Revenue", &format!("${:.0}", total_rev));
        });

        ui.separator();
        ui.label("Recent Sales");
        let recent: Vec<_> = app.analytics.records.iter().rev().take(5).collect();
        if recent.is_empty() {
            ui.label("  No sales recorded yet.");
        } else {
            for r in &recent {
                ui.label(format!("  {} · {} — ${:.2}", r.sale_date.format("%Y-%m-%d"), r.product_name, r.net_revenue));
            }
        }
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
        inline_help::help_button(ui, "templates", &mut app.active_help_topic);
        ui.label("Browse and manage product templates");
    });
}

fn show_bundles(app: &mut DpfApp, ctx: &Context) {
    super::bundle_view::show(app, ctx);
}

fn show_scheduler(app: &mut DpfApp, ctx: &Context) {
    super::scheduler_view::show(app, ctx);
}

fn show_contract(app: &mut DpfApp, ctx: &Context) {
    super::contract_view::show(app, ctx);
}

fn show_settings(app: &mut DpfApp, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Settings");
            inline_help::help_button(ui, "settings", &mut app.active_help_topic);
        });

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