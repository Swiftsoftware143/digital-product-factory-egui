TARGET: HERMES
TASK_TITLE: Build Adverts & Campaign Suite Module for DPF-egui
CONTEXT: /opt/swift/digital-product-factory-egui — existing egui/eframe desktop app, 0 errors 0 warnings, Rust stable, egui 0.27

## Overview
Build the "Adverts & Campaign Suite" module inside the existing Digital Product Factory desktop app. This enables users to create advertising creatives across multiple aspect ratios, with AI-generated copy, background concepts, and conversion scores.

## Architecture document
Read `/opt/swift/digital-product-factory-egui/adverts-architecture.md` for the full architecture spec. The key points:

### Files to create (in order)
1. `src/adverts.rs` — Core domain types (Advert, Campaign, AdCopy, AdFormat, AdBackground, etc.)
2. `src/advert_generator.rs` — AI generation orchestrator that calls llm_router
3. `src/advert_export.rs` — JSON export + image export
4. `src/ui/advert_composer.rs` — Campaign creation + ad composition form
5. `src/ui/advert_preview.rs` — Preview rendering with aspect ratio guides
6. `src/ui/adverts_view.rs` — Campaign manager tab (list + grid + actions)

### Files to modify
7. `src/app.rs` — Add `Adverts` variant to Tab enum; add `adverts_manager` field to DpfApp struct
8. `src/ui/main_content.rs` — Add `Tab::Adverts` routing to `adverts_view::show`
9. `src/ui/mod.rs` — Add `pub mod adverts_view`, `pub mod advert_composer`, `pub mod advert_preview`
10. `src/ui/sidebar.rs` — Add "Adverts" tab to sidebar with 📢 icon
11. `src/main.rs` — Add `mod adverts; mod advert_generator; mod advert_export;`

### Database changes (12. `src/database.rs`)
Add methods:
- `campaigns() -> Vec<Campaign>`
- `save_campaign(c: &Campaign) -> Result<usize>`
- `delete_campaign(id: usize)`
- `save_advert(a: &Advert) -> Result<usize>`
- `adverts_for_campaign(campaign_id: usize) -> Vec<Advert>`
- `delete_advert(id: usize)`

### Execution Instructions
1. **Types first** — Write `src/adverts.rs` with all domain types. `cargo check` must pass before proceeding.
2. **Database** — Add campaign/advert CRUD to `database.rs`. `cargo check` must pass.
3. **Generator** — `advert_generator.rs` takes a product_id, audience, goal, landing_url → calls llm_router with properly formatted prompt → parses output into GeneratedOutput struct
4. **Composer UI** — egui form layout: campaign name input, goal dropdown, audience text input, optional URL, product selector (dropdown from existing products)
5. **Preview** — Renders colored rectangles at aspect ratios, product image at scale, copy text at position per layout_specs
6. **Campaign Manager** — List campaigns on left, selected campaign's adverts on right, open/compose/delete buttons
7. **Integration** — Wire Tab, router, sidebar icon, main.rs module decls

### Guardrails
- `cargo check` after every file change — no exceptions
- All new types must implement Debug + Clone + Serialize + Deserialize
- Use egui 0.27 APIs (not deprecated: no `.wrap()`, use `ComboBox::new(.., "")`, use `TextEdit::singleline().add()` pattern)
- Add `mod` declarations to `src/main.rs` in alphabetical order with existing mods
- Follow existing code patterns: `pub fn show(app: &mut DpfApp, ctx: &Context)` for views
- Use `egui::CentralPanel` for main content, `egui::SidePanel` for campaign list
- Don't modify publishing.rs, mockup_compositor.rs, or asset_library.rs — these are separate domains

### Expected Outcome
- Clean `cargo build` with 0 errors, 0 warnings
- User clicks "Adverts" tab → sees campaign manager
- Can create campaign → compose ads → preview all 3 formats → export JSON
- Generated ads have: brand identity, copy variations (2-5), visual concepts (2-3), layout specs per format, conversion scores

### Verification
1. `cargo check` → 0 errors, 0 warnings
2. `cargo build` → binaries compile
3. Manual: `Tab::Adverts` appears in sidebar, clickable, shows campaign manager UI
