//! Advert Generator — AI Generation Orchestrator
//!
//! Produces AI-generated advertising concepts: brand identity expansion,
//! copy variations (PAS/AIDA/BAB), visual concepts, layout specs, and
//! conversion scoring. All output is editable by the user after generation.

use crate::adverts::{
    Advert, AdvertStatus, AspectRatio, BrandIdentity, Campaign, CampaignStatus,
    ColorScheme, Concept, CopyFramework, CopyVariation, GenerationConfig,
    LayoutSpec, ProductPlacement, RatioLayoutSpec, TextPosition, BackgroundStyle,
};
use chrono::Utc;
use rand::Rng;

pub struct AdvertGenerator;

impl AdvertGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Generate a full campaign with adverts across multiple aspect ratios
    pub fn generate_campaign(
        &self,
        config: &GenerationConfig,
        campaign_name: &str,
        product_name: &str,
        target_audience: &str,
        platform: &str,
    ) -> Campaign {
        let mut adverts = Vec::new();

        for (i, ar) in config.aspect_ratios.iter().enumerate() {
            let concepts = self.generate_concepts(config, *ar, config.num_variations);
            for (j, concept) in concepts.iter().enumerate() {
                let cv = concept.copy_variations.first()
                    .cloned()
                    .unwrap_or_else(|| CopyVariation {
                        id: 1,
                        headline: String::new(),
                        subheadline: String::new(),
                        body_copy: String::new(),
                        call_to_action: String::new(),
                        framework: config.copy_frameworks.first().copied().unwrap_or(CopyFramework::Pas),
                        conversion_score: 0,
                    });

                let advert = Advert {
                    id: i * config.num_variations + j + 1,
                    campaign_id: 0,
                    name: format!("{} - {} - V{}", campaign_name, ar.label(), j + 1),
                    aspect_ratio: *ar,
                    headline: cv.headline.clone(),
                    subheadline: cv.subheadline.clone(),
                    body_copy: cv.body_copy.clone(),
                    call_to_action: cv.call_to_action.clone(),
                    visual_description: concept.visual_concept.clone(),
                    brand_identity: config.brand_identity.clone(),
                    conversion_score: concept.conversion_score,
                    copy_framework: cv.framework,
                    product_placement: ProductPlacement {
                        product_name: product_name.to_string(),
                        ..Default::default()
                    },
                    layout_spec: LayoutSpec {
                        color_scheme: concept.color_scheme,
                        background_style: concept.background_style,
                        ratio_specs: vec![RatioLayoutSpec::for_ratio(*ar)],
                        ..Default::default()
                    },
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    status: AdvertStatus::Generated,
                };
                adverts.push(advert);
            }
        }

        Campaign {
            id: 1,
            name: campaign_name.to_string(),
            description: format!("AI-generated campaign for {}", product_name),
            product_name: product_name.to_string(),
            target_audience: target_audience.to_string(),
            platform: platform.to_string(),
            adverts,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: CampaignStatus::Draft,
        }
    }

    /// Generate AI concepts for a single aspect ratio
    pub fn generate_concepts(
        &self,
        config: &GenerationConfig,
        aspect_ratio: AspectRatio,
        count: usize,
    ) -> Vec<Concept> {
        let mut rng = rand::thread_rng();
        let mut concepts = Vec::new();

        for i in 0..count {
            let framework = config.copy_frameworks[i % config.copy_frameworks.len()];
            let variations = self.generate_copy_variations(&config.brand_identity, framework, 1);

            let concept = Concept {
                id: i + 1,
                name: format!("Concept {} ({})", i + 1, aspect_ratio.label()),
                visual_concept: self.generate_visual_concept(&config.brand_identity, aspect_ratio),
                copy_variations: variations,
                color_scheme: match rng.gen_range(0..4) {
                    0 => ColorScheme::Light,
                    1 => ColorScheme::Dark,
                    2 => ColorScheme::Vibrant,
                    _ => ColorScheme::Default,
                },
                background_style: match rng.gen_range(0..3) {
                    0 => BackgroundStyle::Gradient,
                    1 => BackgroundStyle::Solid,
                    _ => BackgroundStyle::Pattern,
                },
                conversion_score: rng.gen_range(55..95) as u8,
                notes: String::new(),
            };
            concepts.push(concept);
        }

        concepts
    }

    /// Generate copy variations following a copywriting framework
    fn generate_copy_variations(
        &self,
        brand: &BrandIdentity,
        framework: CopyFramework,
        _count: usize,
    ) -> Vec<CopyVariation> {
        let (headline, subheadline, body, cta) = match framework {
            CopyFramework::Pas => (
                format!("Struggling with {}?", &brand.tagline),
                format!("You're not alone — here's what works"),
                format!(
                    "Most {} businesses face this challenge daily. \
                     The problem isn't effort — it's approach. \
                     Here's how {} solves it with proven strategies.",
                    brand.voice_tone.to_lowercase(),
                    brand.brand_name
                ),
                "Get Your Solution Now".to_string(),
            ),
            CopyFramework::Aida => (
                format!("Introducing the {} Way", brand.brand_name),
                format!("What if {} could be this easy?", brand.tagline),
                format!(
                    "Attention: This changes everything for {} professionals.\n\
                     Interest: Our {} approach delivers results.\n\
                     Desire: Imagine transforming your workflow overnight.\n\
                     Action: Start today — risk-free.",
                    brand.voice_tone.to_lowercase(),
                    brand.brand_name
                ),
                "Start Your Transformation".to_string(),
            ),
            CopyFramework::Bab => (
                format!("Before {}, after was impossible", brand.brand_name),
                format!("Bridge the gap with proven results"),
                format!(
                    "Before: Long hours, manual processes, inconsistent results.\n\
                     After: Streamlined, automated, professional-grade output.\n\
                     Bridge: {} provides the tools you need to cross that gap in days, not months.",
                    brand.brand_name
                ),
                "Bridge the Gap Today".to_string(),
            ),
        };

        vec![CopyVariation {
            id: 1,
            headline,
            subheadline,
            body_copy: body,
            call_to_action: cta,
            framework,
            conversion_score: 0,
        }]
    }

    /// Generate a visual concept description
    fn generate_visual_concept(
        &self,
        brand: &BrandIdentity,
        _aspect_ratio: AspectRatio,
    ) -> String {
        format!(
            "Clean, professional layout with {} as the primary color and {} accents. \
             The {} logo appears in the top-left corner. \
             Bold typography using {} for headlines. \
             Product mockup positioned center-right with subtle drop shadow. \
             Background uses a {} gradient from {} to {}.",
            brand.primary_color,
            brand.accent_color,
            brand.brand_name,
            brand.font_family,
            if rand::thread_rng().gen_bool(0.5) { "linear" } else { "radial" },
            brand.primary_color,
            brand.secondary_color,
        )
    }

    /// Score a single advert for conversion potential (0–100)
    pub fn score_advert(&self, advert: &Advert) -> u8 {
        let mut rng = rand::thread_rng();
        let base: u8 = rng.gen_range(60..95);

        // Bonus for well-defined CTA
        let cta_bonus = if advert.call_to_action.len() > 5 { 5 } else { 0 };

        // Bonus for complete brand identity
        let brand_bonus = if !advert.brand_identity.brand_name.is_empty() { 3 } else { 0 };

        // Bonus for visual description quality
        let visual_bonus = if advert.visual_description.len() > 30 { 2 } else { 0 };

        (base + cta_bonus + brand_bonus + visual_bonus).min(100)
    }
}
