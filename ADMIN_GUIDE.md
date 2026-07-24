# Digital Product Factory — Administrator Guide

## Overview

This guide is for administrators managing license keys, feature gating, and platform configuration for the Digital Product Factory desktop app. The app uses a **local license key** system — no internet activation required, no server calls.

---

## License Architecture

### License Tiers

| Tier | Users | Features Unlocked |
|------|-------|-------------------|
| **Personal** (default) | 1 device | Pipeline, Create, Research, Templates, Contracts, Export, Settings, Presets |
| **Team** | 5 devices | All Personal features + Analytics, Publishing, Bundles, Scheduler, Presets |
| **Agency** | 20 devices | All Team features + Whitelabel, Client Management, Custom Integrations |
| **Enterprise** | Unlimited | All features + API Access, Dedicated Support, Custom Contracts |

### Feature Gating

Features are gated by license tier. Each feature identifier maps to a minimum tier:

| Feature | Min Tier | Module |
|---------|----------|--------|
| pipeline | Personal | Pipeline (Kanban) |
| ai_generation | Personal | Create |
| templates | Personal | Templates |
| market_research | Personal | Research |
| contract_generator | Personal | Contracts |
| export | Personal | Export |
| analytics | Team | Analytics & Sales Tracking |
| publishing | Team | Marketplace Publishing |
| bundles | Team | Bundles |
| scheduler | Team | Scheduler |
| presets | Team | Industry Presets |
| whitelabel | Agency | Branding & White Label |
| client_management | Agency | Client accounts |
| custom_integrations | Agency | Custom integrations |
| api_access | Enterprise | API access |

### Configuration Files

All license and feature configurations are **external JSON files** — no rebuild needed to add tiers, change pricing, or update platform formats.

#### feature_tiers.json

\\\json
{
  "pipeline": "personal",
  "ai_generation": "personal",
  "templates": "personal",
  "market_research": "personal",
  "contract_generator": "personal",
  "export": "personal",
  "analytics": "team",
  "publishing": "team",
  "bundles": "team",
  "scheduler": "team",
  "presets": "team",
  "whitelabel": "agency",
  "client_management": "agency",
  "custom_integrations": "agency",
  "api_access": "enterprise",
  "admin_panel": "enterprise"
}
\\\

#### pricing.json

\\\json
{
  "personal": {
    "name": "Personal",
    "price": 0,
    "period": "lifetime",
    "users": 1,
    "features": ["pipeline", "ai_generation", "templates",
                 "market_research", "contract_generator", "export"]
  },
  "team": {
    "name": "Team",
    "price": 29,
    "period": "month",
    "users": 5,
    "features": ["pipeline", "ai_generation", "templates",
                 "market_research", "contract_generator", "export",
                 "analytics", "publishing", "bundles", "scheduler",
                 "presets"]
  },
  "agency": {
    "name": "Agency",
    "price": 99,
    "period": "month",
    "users": 20,
    "features": ["pipeline", "ai_generation", "templates",
                 "market_research", "contract_generator", "export",
                 "analytics", "publishing", "bundles", "scheduler",
                 "presets", "whitelabel", "client_management",
                 "custom_integrations"]
  },
  "enterprise": {
    "name": "Enterprise",
    "price": 299,
    "period": "month",
    "users": -1,
    "features": ["pipeline", "ai_generation", "templates",
                 "market_research", "contract_generator", "export",
                 "analytics", "publishing", "bundles", "scheduler",
                 "presets", "whitelabel", "client_management",
                 "custom_integrations", "api_access", "admin_panel"]
  }
}
\\\

#### platform_formats.json

\\\json
{
  "etsy": {
    "thumbnail_width": 3000,
    "thumbnail_height": 3000,
    "max_file_size_mb": 20,
    "max_title_length": 140,
    "max_description_length": 5000,
    "max_tags": 13,
    "allowed_types": ["physical", "digital"]
  },
  "gumroad": {
    "thumbnail_width": 1280,
    "thumbnail_height": 720,
    "max_file_size_mb": 50,
    "max_title_length": 255,
    "max_description_length": 10000,
    "max_tags": 0,
    "allowed_types": ["digital", "membership"]
  },
  "shopify": {
    "thumbnail_width": 2048,
    "thumbnail_height": 2048,
    "max_file_size_mb": 20,
    "max_title_length": 255,
    "max_description_length": 5000,
    "max_tags": 0,
    "allowed_types": ["physical", "digital", "service"]
  },
  "payhip": {
    "thumbnail_width": 1280,
    "thumbnail_height": 720,
    "max_file_size_mb": 100,
    "max_title_length": 100,
    "max_description_length": 20000,
    "max_tags": 0,
    "allowed_types": ["digital", "bundle"]
  }
}
\\\

---

## Key Generation

License keys use a format: \DPF-XXXX-XXXX-XXXX\ where X is alphanumeric. Keys encode the tier and a signature.

### Creating Keys

Use the \LicenseManager\ to generate keys:

\\\
ust
let key = license_manager.generate_key("team");
// Returns: "DPF-A3K8-M2P1-X9R5"
\\\

Keys are stored in the local SQLite database in the \license_keys\ table. Revoked keys are tracked separately.

### Validating Keys

When a user enters a license key:
1. Check format matches \DPF-XXXX-XXXX-XXXX\
2. Verify the key exists in the database
3. Check the key has not been revoked
4. Unlock features matching the key's tier

### Revoking Keys

Add to \
evoked_keys.json\:

\\\json
{
  "revoked": ["DPF-XXXX-XXXX-XXXX"]
}
\\\

---

## Admin Panel

The admin panel is accessible via **Settings > Admin** when an Enterprise license is active.

### Features

- **License Key Management** — View all generated keys, their tiers, and status
- **Revoke Keys** — Revoke a key by entering it
- **Feature Tiers Editor** — Edit which features map to which tiers
- **Pricing Editor** — Edit pricing and plan descriptions
- **Platform Formats Editor** — Update platform requirements
- **Database Viewer** — Browse tables and records

### Testing License Keys

1. Go to Settings > License
2. Click "Enter License Key"
3. Enter the generated key
4. Features unlock immediately — no restart needed

---

## Deployment Checklist

1. **Config files in app directory:**
   - \eature_tiers.json\
   - \pricing.json\
   - \platform_formats.json\
   - \
evoked_keys.json\

2. **Database location:** \$HOME/dpf_data.db\ (SQLite)

3. **API keys (user-provided):**
   - OpenAI API key
   - Anthropic API key
   - Google API key
   - DeepSeek API key
   - Moonshot API key

4. **Marketplace API keys (stored in OS keychain):**
   - Etsy API key
   - Gumroad access token

5. **Font:** \/assets/Inter-Regular.ttf\

---

## Testing

### Quick Test

\\\
cargo run
\\\

### Feature Gating Test

1. Launch without a license key — confirm only Personal features are accessible
2. Enter a valid "team" tier key — confirm Analytics, Publishing, Bundles unlock
3. Enter an "agency" tier key — confirm whitelabel and client management unlock
4. Try a revoked key — confirm rejection
5. Try an invalid format key — confirm rejection

### Marketplace Publishing Test

1. Add Etsy API key via Publishing tab
2. Confirm key shows as stored (green check)
3. Remove key
4. Confirm key shows as not stored (red X)
5. Repeat with Gumroad

---

## Configuration File Locations

| File | Purpose | Location |
|------|---------|----------|
| \eature_tiers.json\ | Feature-to-tier mapping | App directory |
| \pricing.json\ | Pricing and plan details | App directory |
| \platform_formats.json\ | Marketplace format rules | App directory |
| \
evoked_keys.json\ | Revoked license keys | App directory |
| \dpf_data.db\ | SQLite database | \$HOME/dpf_data.db\ |
| \sales_export.csv\ | CSV export output | App directory |

---

*Version 1.4.1 — Digital Product Factory Admin Guide*