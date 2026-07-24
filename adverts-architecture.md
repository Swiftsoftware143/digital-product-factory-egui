# Architecture: Adverts & Campaign Suite for DPF-egui

## Design Decisions

### 1. Module Structure
- **`src/adverts.rs`** — Core domain types, Advert/Campaign structs, AdEngine trait
- **`src/ui/adverts_view.rs`** — Main campaign listing + advert grid view
- **`src/ui/advert_composer.rs`** — Visual composer: product, background, copy, layout
- **`src/ui/advert_preview.rs`** — In-app rendered previews (egui-canvas for mockup)
- **`src/advert_generator.rs`** — AI-powered generation orchestrator (calls LLM, manages templates)
- **`src/advert_export.rs`** — JSON + image export to /assets/exports/

### 2. Why Not Modifying Existing Files
- `publishing.rs` does marketplace listings (Etsy/Gumroad) — separate concern
- `mockup_compositor.rs` does product mockups — related but distinct domain
- New module avoids touch existing working code

### 3. Tab Entry
- Add `Adverts` variant to `Tab` enum in `app.rs`
- Route in `main_content.rs`: `Tab::Adverts => adverts_view::show(app, ctx)`
- Add `crate::ui::adverts_view` to sidebar

### 4. Domain Types (src/adverts.rs)

```rust
pub struct Advert {
    pub id: usize,
    pub campaign_id: usize,
    pub product_id: usize,
    pub product_name: String,
    pub format: AdFormat,
    pub copy: AdCopy,
    pub background: AdBackground,
    pub conversion_score: u8,
    pub status: AdvertStatus,
    pub created_at: DateTime<Utc>,
}

pub enum AdFormat {
    Square1080,       // 1:1 Instagram/Facebook
    Story1920,        // 9:16 TikTok/Reels
    Landscape1200x628, // 16:9 Link Ads
    Banner728x90,
    Medium300x250,
    Skyscraper160x600,
    Custom { w: u32, h: u32, shape: Shape },
}

pub enum Shape { Rect, Circle, Hexagon, TransparentCutout }

pub struct AdCopy {
    pub angle: CopyAngle,
    pub headline: String,
    pub primary_text: String,
    pub cta: String,
}

pub enum CopyAngle { Pas, Aida, Bab, SocialProof, BenefitDriven }

pub struct AdBackground {
    pub concept_name: String,
    pub background_prompt: String,
}

pub struct Campaign {
    pub id: usize,
    pub name: String,
    pub goal: CampaignGoal,
    pub target_audience: String,
    pub landing_url: Option<String>,
    pub adverts: Vec<Advert>,
    pub created_at: DateTime<Utc>,
}

pub enum CampaignGoal {
    LeadGeneration,
    BrandAwareness,
    SalesConversion,
    PromoSale,
}

pub struct GeneratedOutput {
    pub brand_identity: BrandIdentity,
    pub ad_copy_variations: Vec<AdCopy>,
    pub visual_concepts: Vec<VisualConcept>,
    pub layout_specs: HashMap<String, LayoutSpec>,
}

pub struct BrandIdentity {
    pub extracted_tone: String,
    pub recommended_palette: Vec<String>,
    pub target_platforms: Vec<String>,
}

pub struct VisualConcept {
    pub concept_name: String,
    pub background_prompt: String,
    pub conversion_score: u8,
    pub score_reasoning: String,
}

pub struct LayoutSpec {
    pub dimensions: String,
    pub headline_position: String,
    pub product_scale: String,
    pub cta_badge: String,
}
```

### 5. Generation Pipeline (src/advert_generator.rs)
1. User selects product(s) from DPF database
2. User enters target audience, campaign goal, optional landing URL
3. AI generates: brand identity extraction, 2-5 copy variations, 2-3 visual concepts, layout specs for all 3 ratios
4. User can regenerate any section independently
5. User can mark as "ready" → stores to DB + exports JSON

### 6. Preview Rendering (src/ui/advert_preview.rs)
- Renders layout rectangles with aspect ratio guides
- Product image composited at correct scale per layout_specs
- Copy displayed at correct positions per layout_specs
- CTA badge shown in correct quadrant
- Conversion score badge shown

### 7. Export (src/advert_export.rs)
- JSON export ad copy + layout specs + generation params
- Option to save advert configuration to database
- Image export uses mockup_compositor::composite() if available

---

## Implementation Order

### Phase A: Core Types + Database (src/adverts.rs + database.rs)
1. Advert, Campaign, AdCopy, AdFormat, AdBackground types
2. Campaign store/load/delete in Database
3. Advert store/load/delete in Database

### Phase B: Composer UI (src/ui/advert_composer.rs)
1. Campaign creation form (name, goal, audience, URL)
2. Product selector (from existing DPF products)
3. Copy editor (headline, text, CTA per angle)
4. Background selector (concept from AI or custom prompt)
5. Format grid (all aspect ratios)

### Phase C: Preview + Generation (src/ui/advert_preview.rs + src/advert_generator.rs)
1. Layout rectangle rendering per format
2. Product image placement at scale
3. Copy positioning per layout_spec
4. CTA badge placement
5. Conversion score display
6. AI generation via LLM router

### Phase D: Campaign Manager Tab (src/ui/adverts_view.rs)
1. Campaign list (side panel)
2. Advert grid for selected campaign
3. Open-in-composer button
4. Delete/duplicate campaign
5. Export button (JSON)

### Phase E: Integration (app.rs + main_content.rs + ui/mod.rs)
1. Tab::Adverts
2. Router entry
3. Sidebar icon
4. module declarations
