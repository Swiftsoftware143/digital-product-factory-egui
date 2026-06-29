# Digital Product Factory - User Guide

## Table of Contents
1. [Getting Started](#getting-started)
2. [Industry Presets](#industry-presets)
3. [The Pipeline](#the-pipeline)
4. [Creating Products](#creating-products)
5. [Market Research](#market-research)
6. [Contract Generator](#contract-generator)
7. [Bundles](#bundles)
8. [Scheduler](#scheduler)
9. [Exporting](#exporting)
10. [Settings](#settings)
11. [License Management](#license-management)
12. [Workflow Examples](#workflow-examples)

---

## Getting Started

### Installation

1. Download the latest release from GitHub
2. Extract the archive
3. Run the executable:
   - Windows: `dpf.exe`
   - macOS: `dpf`
   - Linux: `dpf`

### First Launch

On first launch, you'll see:
- **Dashboard**: Overview of your products and pipeline
- **Sidebar**: Navigation between all features
- **Status Bar**: License status and system info

### Initial Setup

1. Click **Settings** (⚙️ icon in top right)
2. Add your API keys:
   - OpenAI API key (for GPT-4o, GPT-3.5)
   - Anthropic API key (for Claude)
   - Google API key (for Gemini)
3. Configure preferences (auto-save, dark mode)
4. Click **Save**

---

## Industry Presets

Industry Presets are pre-configured workflows designed for specific business models. Each preset includes a complete pipeline with recommended actions, modules, and tips tailored to that industry.

### Available Presets

| Preset | Best For | Stages |
|--------|----------|--------|
| **Affiliate Marketing** | Product reviewers, email marketers | Research → Ideation → Create → Legal → Bundle → Schedule → Publish → Analyze |
| **Content Creator** | YouTubers, influencers, podcasters | Ideation → Sponsor Deals → Production → Legal → Assets → Schedule → Export → Archive |
| **Creator + Affiliate Hybrid** | Multi-monetization creators | Combined workflow for brand deals + affiliate revenue |
| **E-Learning** | Course creators, coaches | Curriculum → Content → Legal → Bundle → Schedule → Export |
| **SaaS** | Software founders, app developers | Roadmap → Development → Legal → Beta → Launch → Export |
| **Marketing Agency** | Agencies, consultants | Prospect → Onboard → Strategy → Execute → Optimize → Report |
| **Freelancer** | Developers, designers, writers | Lead → Proposal → Contract → Work → Review → Deliver |
| **Info Products** | Template sellers, ebook authors | Research → Outline → Create → Design → Legal → Bundle → Export → Launch |
| **Business Setup** | New sellers, side hustlers | Complete 12-step workflow from research to launch |

### Using Presets

1. Go to **🎯 Presets** tab in the sidebar
2. Browse preset cards with descriptions
3. Click **View Details** to see full workflow
4. Click **Load Pipeline** to create sample ideas for each stage
5. Customize the generated ideas for your needs

### Preset Features

Each preset includes:
- **Stage breakdown**: What to do at each step
- **Recommended modules**: Which Factory features to use
- **Action checklists**: Specific tasks to complete
- **Output descriptions**: What you should have after each stage
- **Quick tips**: Industry-specific advice

---

## The Pipeline

The Pipeline is your command center for managing products from idea to sale. It's a kanban-style workflow with 7 stages.

### Pipeline Stages

| Stage | Icon | Purpose |
|-------|------|---------|
| **Idea** | 💡 | Capture product ideas |
| **Research** | 🔍 | Market research completed |
| **Creating** | 🔨 | Actively building the product |
| **Review** | 👀 | Quality check before publishing |
| **Listed** | 📋 | Published on platforms |
| **Selling** | 💰 | Active sales and revenue |
| **Archived** | 📦 | No longer active |

### View Modes

Switch between three views using the toolbar:

1. **Kanban** (default): Drag-and-drop cards between columns
2. **List**: Sortable table view with all details
3. **Calendar**: Timeline view for scheduled releases

### Adding Ideas

**Quick Add:**
1. Click **"➕ Quick Add Idea"** button
2. Fill in:
   - Title (required)
   - Description
   - Product type
   - Priority (Low/Medium/High/Urgent)
   - Estimated value
3. Click **Add Idea**

**From Dashboard:**
- Click any Quick Action card to add directly to that stage

### Managing the Pipeline

**Moving Cards:**
- **Drag and drop**: Click and drag the ⋮⋮ handle to move between stages
- **Keyboard**: Select card, use arrow keys + Enter

**Editing:**
- Click any card to open details
- Edit inline or open full editor
- Changes auto-save

**Filtering:**
- Use search box to filter by title, description, or tags
- Click stage headers to filter by stage
- Use priority filters for urgent items

### Pipeline Statistics

The Dashboard shows:
- **Total Ideas**: All items in pipeline
- **In Progress**: Items in Creating stage
- **Selling**: Active revenue-generating products
- **Potential Value**: Sum of all estimated values

---

## Creating Products

### Selecting a Template

1. Go to **Create** tab
2. Browse templates by category:
   - **Planners**: Daily, weekly, monthly planners
   - **Journals**: Gratitude, fitness, travel journals
   - **Spreadsheets**: Budget trackers, calculators
   - **Guides**: How-to guides, workbooks
   - **Legal**: Contracts, agreements
3. Click **Select** on desired template

### Built-in Templates

#### Classic Templates

| Template | Best For | Output |
|----------|----------|--------|
| **Daily Planner** | Productivity enthusiasts | PDF |
| **Gratitude Journal** | Wellness market | PDF |
| **Budget Tracker** | Finance niche | XLSX |
| **Freelance Contract** | Service providers | DOCX |

#### Digital Product Templates

| Template | Category | Description |
|----------|----------|-------------|
| **Digital Stickers Pack** | Digital Stickers | Sticker packs for OneNote, GoodNotes, and note-taking apps |
| **Digital Art Printables** | Digital Art | AI-generated art for wall art and home decor |
| **Clip Art Bundle** | Clip Art | Pre-made graphics for presentations and documents |
| **Adult Coloring Pages** | Coloring Pages | Intricate mandalas, animals, landscapes for coloring books |
| **Logo Design Pack** | Logo Design | Professional logos for small businesses and startups |
| **Notion Template** | Notion Templates | Productivity templates for students and professionals |
| **Printables & Planners** | Printables | Printable planners, trackers, and organizers |
| **Print-on-Demand Designs** | POD Designs | AI designs for mugs, shirts, hoodies, and POD products |

### Template Categories

Templates are organized by category:
- **Planners**: Daily, weekly, monthly planners
- **Journals**: Gratitude, fitness, travel journals
- **Spreadsheets**: Budget trackers, calculators
- **Guides**: How-to guides, workbooks
- **Legal**: Contracts, agreements
- **Digital Stickers**: For note-taking apps
- **Digital Art**: Printable wall art
- **Clip Art**: Graphics for documents
- **Coloring Pages**: Adult coloring books
- **Logo Design**: Business branding
- **Notion Templates**: Productivity systems
- **Printables**: Planners and trackers
- **POD Designs**: Print-on-demand graphics

### Configuring Parameters

Each template has customizable parameters:

**Example: Daily Planner**
- Morning routine duration (15/30/60 min)
- Style (Minimal/Decorative/Professional)
- Color scheme

**Example: Freelance Contract**
- Client name
- Service description
- Payment amount
- Timeline
- Number of revisions

### Generating

1. Fill all required fields (marked with red *)
2. Click **Preview Prompt** to see the AI prompt
3. Click **⚡ Generate Product**
4. Wait for generation (typically 5-30 seconds)
5. Review the output
6. Export or save to pipeline

### AI Models

The system automatically selects the best AI model:

| Task | Model | Provider |
|------|-------|----------|
| Creative writing | GPT-4o | OpenAI |
| Structured data | Claude 3.5 | Anthropic |
| Technical content | Gemini 1.5 | Google |
| Quick tasks | GPT-3.5 | OpenAI |

### AI Prompt Templates

Each template includes optimized AI prompts with:
- **Aspect ratio presets**: `--ar 293:151` for mugs, `--ar 1:1` for stickers, etc.
- **Style modifiers**: Watercolor, minimalist, vintage, etc.
- **Output specifications**: Format, resolution, use case
- **Time estimates**: Most products take ~1 day/month to maintain

Example prompts are shown in the template details and can be copied for use with your preferred AI image generator.

---

## Market Research

### Searching Platforms

1. Go to **Research** tab
2. Enter search query (e.g., "digital planner 2026")
3. Select platforms:
   - Etsy (handmade/creative)
   - Gumroad (digital products)
   - Amazon (broader market)
4. Click **🔍 Search**

### Understanding Results

For each product found:
- **Title**: Product name
- **Price**: Current selling price
- **Rating**: Customer rating (if available)
- **Reviews**: Number of reviews
- **Seller**: Shop/platform name

### Market Insights

After searching, the system analyzes:

**Price Analysis:**
- Average price
- Price range (min/max)
- Recommended pricing

**Competition Level:**
- **Low** (0-10 results): High opportunity
- **Medium** (11-50): Moderate competition
- **High** (51-200): Saturated market
- **Saturated** (200+): Very competitive

**Top Keywords:**
- Most common words in successful listings
- Use these for SEO and titles

### Trending Searches

Quick-access buttons for hot niches:
- planner 2026
- digital journal
- budget tracker
- social media templates
- notion template
- resume template

Click any trend to auto-fill search.

---

## Contract Generator

### Creating Legal Contracts

1. Go to **Contracts** section (in Create tab)
2. Select contract type:
   - Freelance Service Agreement
   - Mutual NDA
   - Rental/Lease Agreement
   - Employment Contract
   - Sales of Goods
   - Partnership Agreement
   - Consulting Agreement
   - Coaching Agreement

### Guided Prompts

The assistant will ask for:
- Party names (client, freelancer, etc.)
- Specific details (payment, timeline, scope)
- Jurisdiction (governing law)

**Example: Freelance Contract**
- Client name
- Your name/business
- Service description
- Payment amount
- Payment schedule
- Project timeline
- Revision count
- Jurisdiction

### Understanding the Output

**Full Contract:**
- Professional legal language
- All standard clauses
- Customized to your inputs
- Ready for signatures

**Plain English Summary:**
- Simple explanation of terms
- Who the parties are
- Key obligations
- Payment details

**Legal Disclaimer:**
Every contract includes:
```
IMPORTANT: This is a template only, not legal advice.
Always have a lawyer review before signing.
```

### When to Use vs. Hire Lawyer

| Use Generator | Hire Lawyer |
|---------------|-------------|
| Simple freelance work | High-value deals ($10K+) |
| Basic NDA | Complex partnerships |
| Standard rental | Employment with equity |
| Small sales | International contracts |
| Coaching services | Regulated industries |

---

## Bundles

### Auto-Bundle Strategies

**By Category:**
Groups products by type (all planners, all journals, etc.)
- Best for: Themed collections
- Discount: 20%

**By Value:**
Creates tiered bundles:
- **Premium Collection**: Top 5 products (25% off)
- **Starter Pack**: Entry-level selection (30% off)

**Seasonal:**
Creates time-themed bundles:
- Winter Collection (Jan-Mar)
- Spring Collection (Apr-Jun)
- Summer Collection (Jul-Sep)
- Fall Collection (Oct-Dec)

### Manual Bundle Builder

1. Go to **Bundles** tab
2. Select products from your pipeline
3. Set bundle name and description
4. Choose discount percentage
5. Review bundle statistics
6. Export as ZIP

### Bundle Statistics

For each bundle:
- **Product Count**: Number of items
- **Total Value**: Sum of individual prices
- **Customer Savings**: Dollar amount saved
- **Savings %**: Discount percentage
- **Est. Conversion**: Predicted sales rate

---

## Scheduler

### Schedule Types

**Once:**
- Single execution at specific date/time
- Use for: One-time product drops

**Daily:**
- Repeats every day at set time
- Use for: Daily social media posts

**Weekly:**
- Repeats on specific day of week
- Use for: Weekly product releases

**Interval:**
- Repeats every X minutes
- Use for: Frequent monitoring tasks

**Smart:**
- Automatically picks optimal time
- Business hours only
- Best for: Pinterest pins (8-11pm, 2-4pm optimal)

### Task Types

| Task | Description |
|------|-------------|
| **Generate Product** | Auto-create from template |
| **Publish Product** | Push to Etsy/Gumroad |
| **Research Market** | Run market analysis |
| **Create Bundle** | Auto-bundle products |
| **Pinterest Pin** | Schedule pin posts |
| **Backup Data** | Database backup |

### Managing Tasks

**Add Task:**
1. Click **➕ Add Task**
2. Select task type
3. Configure parameters
4. Set schedule
5. Enable/disable

**Monitor:**
- Green dot: Completed
- Yellow dot: Running
- Red dot: Failed
- Gray dot: Pending

**Actions:**
- ▶ Start scheduler
- ⏸ Stop scheduler
- Toggle individual tasks
- Delete old tasks

---

## Exporting

### Export Formats

| Format | Best For | Extension |
|--------|----------|-----------|
| **Markdown** | Raw content, editing | .md |
| **HTML** | Web publishing, preview | .html |
| **PDF** | Final distribution | .pdf |
| **DOCX** | Microsoft Word users | .docx |
| **XLSX** | Spreadsheets, trackers | .xlsx |
| **JSON** | Data interchange | .json |
| **ZIP** | Multiple products | .zip |

### Exporting Single Product

1. Open product from pipeline
2. Click **Export** button
3. Select format
4. Choose output directory
5. Click **Save**

### Exporting Bundles

1. Go to **Bundles** tab
2. Select bundle
3. Click **Export Bundle**
4. Choose format (ZIP recommended)
5. All products exported together

### Batch Export

Select multiple products in pipeline:
1. Hold Ctrl/Cmd and click products
2. Right-click → **Export Selected**
3. Choose format
4. Products exported individually or as ZIP

---

## Settings

### API Configuration

**OpenAI:**
- Get key: https://platform.openai.com/api-keys
- Models: GPT-4o, GPT-4, GPT-3.5

**Anthropic:**
- Get key: https://console.anthropic.com
- Models: Claude 3.5 Sonnet

**Google:**
- Get key: https://makersuite.google.com/app/apikey
- Models: Gemini 1.5 Pro

### Safety Limits

Configure rate limiting:
- Max searches per hour: 20 (default)
- Max products per day: 10 (default)
- Max publishes per hour: 5 (default)

These prevent API overuse and platform bans.

### Performance

- **Auto-save**: Save work automatically
- **Dark mode**: Toggle light/dark theme
- **Max concurrent tasks**: Limit parallel operations
- **Cache size**: Database cache in MB

---

## License Management

### Tiers

| Tier | Devices | Features | Price |
|------|---------|----------|-------|
| **Personal** | 1 | Basic generation | $49-99 |
| **Team** | 5 | +Scheduler, Pinterest | $149-299 |
| **Agency** | 20 | +White-label, Client mgmt | $499-999 |
| **Enterprise** | Unlimited | +API, Custom dev | $1999+ |

### Activation

1. Purchase license from your store
2. Receive license key (format: XXXX-XXXX-XXXX-XXXX)
3. Click status bar ("⚠ Unlicensed")
4. Enter license key
5. Click **Activate**

### Device Management

View activated devices:
- Device name
- Activation date
- Last used

Deactivate old devices to free up slots.

---

## Workflow Examples

### Quick Product Launch (1 hour)

1. **Research** (10 min):
   - Search Etsy for "digital planner"
   - Note top keywords and prices
   - Identify gap in market

2. **Create** (30 min):
   - Select "Daily Planner" template
   - Configure: 30-min morning routine, Minimal style
   - Generate with GPT-4o
   - Review and refine

3. **Export** (5 min):
   - Export as PDF
   - Create listing images (Canva/Figma)

4. **Publish** (15 min):
   - Upload to Etsy
   - Use researched keywords in title
   - Price at market average

5. **Pipeline**:
   - Add to "Listed" stage
   - Set reminder to check sales in 1 week

### Weekly Batch Production (4 hours)

1. **Monday - Research** (1 hour):
   - Identify 5 trending niches
   - Analyze top 10 products in each
   - Document keywords and pricing

2. **Tuesday-Wednesday - Create** (2 hours):
   - Generate 10 products
   - Use different templates
   - Vary styles and parameters

3. **Thursday - Bundle** (30 min):
   - Auto-bundle by category
   - Create 3 themed bundles
   - Set 20-30% discounts

4. **Friday - Schedule** (30 min):
   - Schedule 2 products/day for next week
   - Set Pinterest pins for optimal times
   - Enable auto-backup

### VA Team Operation

1. **Setup**:
   - Install on Team license (5 devices)
   - Create shared API key pool
   - Document brand guidelines

2. **Workflow**:
   - VA generates products from approved templates
   - Manager reviews in "Review" stage
   - Approved products auto-scheduled
   - Sales tracked in "Selling" stage

3. **Quality Control**:
   - Random sample review (10%)
   - Consistency checks
   - Monthly performance reports

---

## Troubleshooting

### Common Issues

**"API Key Invalid"**
- Check key is copied correctly
- Verify key has credits/balance
- Try regenerating key

**"Generation Failed"**
- Check internet connection
- Verify API service status
- Try different AI model

**"Export Failed"**
- Check disk space
- Verify write permissions
- Try different output directory

**"Slow Performance"**
- Close other applications
- Reduce concurrent tasks
- Check database size (auto-vacuum runs monthly)

### Getting Help

1. Check this guide first
2. Review error messages carefully
3. Check GitHub issues: https://github.com/Swiftsoftware143/digital-product-factory-egui/issues
4. Contact support with:
   - Error message
   - Steps to reproduce
   - System info (OS, RAM)

---

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| Ctrl+N | New idea |
| Ctrl+S | Save |
| Ctrl+G | Generate product |
| Ctrl+E | Export |
| Ctrl+1 | Dashboard |
| Ctrl+2 | Pipeline |
| Ctrl+3 | Create |
| Ctrl+4 | Research |
| Ctrl+, | Settings |
| Esc | Close dialog |

---

## Tips for Success

1. **Start with research**: Never create without checking market
2. **Use the pipeline**: Track every idea from start to finish
3. **Bundle strategically**: 3+ products bundled sell 40% better
4. **Schedule consistently**: Regular releases build audience
5. **Track everything**: Use actual_value field for real sales data
6. **Iterate**: Move products back to "Creating" for improvements
7. **Stay legal**: Always use contract generator for client work
8. **Backup regularly**: Enable scheduled backups
9. **Use Industry Presets**: Start with a preset that matches your business model
10. **Focus first**: Perfect one product before expanding (per Business Setup preset)
11. **Create handcrafted-looking images**: Good shops have authentic, non-stock visuals
12. **Test multiple platforms**: Different platforms attract different buyer types

---

*Version 1.1.0 - Native Rust Edition*
*Last Updated: June 2026*

## Changelog

### v1.1.0
- Added **Industry Presets** with 9 pre-configured workflows
- Added **8 new digital product templates** (stickers, art, clip art, coloring pages, logos, Notion templates, printables, POD designs)
- Removed external tool references - Digital Product Factory is now the complete solution
- Updated Business Setup preset with 12-step workflow
- Removed hardcoded pricing from templates (users set their own prices)

### v1.0.0
- Initial release
- Pipeline kanban workflow
- AI product generation (OpenAI, Anthropic, Google)
- Market research
- Contract generator
- Scheduler
- Bundle builder
- Export (PDF, DOCX, XLSX, ZIP)
