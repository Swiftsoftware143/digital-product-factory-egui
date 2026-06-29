//! Main application state and UI

use egui::*;
use std::sync::Arc;
use tokio::runtime::Runtime;

use crate::{
    pipeline::{Pipeline, PipelineStage, ProductIdea},
    product_generator::ProductGenerator,
    license_manager::LicenseManager,
    templates::TemplateRegistry,
    research::MarketResearch,
    scheduler::Scheduler,
    bundler::Bundler,
    exporter::Exporter,
    contract_generator::ContractGenerator,
    database::Database,
    config::AppConfig,
    presets::PresetRegistry,
    ui::{sidebar, main_content, status_bar},
};

pub struct DpfApp {
    // Core state
    pub db: Arc<Database>,
    pub runtime: Arc<Runtime>,
    pub config: AppConfig,

    // Modules
    pub pipeline: Pipeline,
    pub generator: ProductGenerator,
    pub license_manager: LicenseManager,
    pub template_registry: TemplateRegistry,
    pub research: MarketResearch,
    pub scheduler: Scheduler,
    pub bundler: Bundler,
    pub exporter: Exporter,
    pub contract_generator: ContractGenerator,
    pub preset_registry: PresetRegistry,

    // UI State
    pub current_tab: Tab,
    pub sidebar_expanded: bool,
    pub search_query: String,
    pub selected_product: Option<usize>,
    pub show_settings: bool,
    pub show_license_dialog: bool,
    
    // Preset State
    pub selected_preset_id: Option<String>,
    pub loaded_preset_id: Option<String>,

    // Performance
    pub last_frame_time: std::time::Instant,
    pub fps: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Dashboard,
    Pipeline,
    Create,
    Research,
    Templates,
    Bundles,
    Scheduler,
    Presets,
    Settings,
}

impl DpfApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load fonts for crisp text (use embedded or system font)
        let mut fonts = egui::FontDefinitions::default();
        
        // Try to load Inter font if available, otherwise use default
        #[cfg(feature = "embed-font")]
        {
            fonts.font_data.insert(
                "inter".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/Inter-Regular.ttf")),
            );
            fonts.families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "inter".to_owned());
        }
        
        cc.egui_ctx.set_fonts(fonts);
        
        // Load previous state if any
        let config = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            AppConfig::default()
        };
        
        // Initialize async runtime
        let runtime = Arc::new(Runtime::new().expect("Failed to create Tokio runtime"));
        
        // Initialize database
        let db = Arc::new(Database::new().expect("Failed to initialize database"));
        
        // Load modules
        let pipeline = Pipeline::load(&db);
        let mut generator = ProductGenerator::new(&db, runtime.clone());
        
        // Set API keys from config
        generator.set_api_keys(
            config.openai_key.clone(),
            config.anthropic_key.clone(),
            config.google_key.clone(),
        );
        
        let license_manager = LicenseManager::new(&db);
        let template_registry = TemplateRegistry::new();
        let research = MarketResearch::new(runtime.clone());
        let scheduler = Scheduler::new(&db, runtime.clone());
        let bundler = Bundler::new();
        let exporter = Exporter::new();
        let contract_generator = ContractGenerator::new();
        let preset_registry = PresetRegistry::new();

        Self {
            db,
            runtime,
            config,
            pipeline,
            generator,
            license_manager,
            template_registry,
            research,
            scheduler,
            bundler,
            exporter,
            contract_generator,
            preset_registry,
            current_tab: Tab::Dashboard,
            sidebar_expanded: true,
            search_query: String::new(),
            selected_product: None,
            show_settings: false,
            show_license_dialog: false,
            selected_preset_id: None,
            loaded_preset_id: None,
            last_frame_time: std::time::Instant::now(),
            fps: 0.0,
        }
    }
}

impl eframe::App for DpfApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.config);
    }
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Calculate FPS for performance monitoring
        let now = std::time::Instant::now();
        let dt = now.duration_since(self.last_frame_time).as_secs_f32();
        self.fps = 1.0 / dt;
        self.last_frame_time = now;
        
        // Continuous UI mode for responsiveness
        ctx.request_repaint_after(std::time::Duration::from_millis(16)); // ~60 FPS
        
        // Top panel - Title bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Digital Product Factory");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{:.0} FPS", self.fps));
                    if ui.button("⚙").clicked() {
                        self.show_settings = true;
                    }
                });
            });
        });
        
        // Left sidebar - Navigation
        sidebar::show(self, ctx);
        
        // Main content area
        main_content::show(self, ctx);
        
        // Bottom status bar
        status_bar::show(self, ctx);
        
        // Modal dialogs
        if self.show_settings {
            ui::settings_dialog::show(self, ctx);
        }
        
        if self.show_license_dialog {
            ui::license_dialog::show(self, ctx);
        }
    }
}
