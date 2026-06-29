//! Tools Reference - Comprehensive list of recommended tools for digital product businesses

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub id: String,
    pub name: String,
    pub category: ToolCategory,
    pub description: String,
    pub use_case: String,
    pub pricing: String,
    pub url: Option<String>,
    pub alternatives: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ToolCategory {
    AiGeneration,
    Research,
    Design,
    Ecommerce,
    Automation,
    EmailMarketing,
    PrintOnDemand,
    SocialMedia,
    Testing,
    Legal,
}

impl ToolCategory {
    pub fn name(&self) -> &'static str {
        match self {
            ToolCategory::AiGeneration => "AI Generation",
            ToolCategory::Research => "Research & Analytics",
            ToolCategory::Design => "Design & Creative",
            ToolCategory::Ecommerce => "E-commerce Platforms",
            ToolCategory::Automation => "Automation & Fulfillment",
            ToolCategory::EmailMarketing => "Email Marketing",
            ToolCategory::PrintOnDemand => "Print on Demand",
            ToolCategory::SocialMedia => "Social Media",
            ToolCategory::Testing => "Testing & Feedback",
            ToolCategory::Legal => "Legal & Contracts",
        }
    }
}

pub struct ToolsRegistry {
    tools: HashMap<String, Tool>,
}

impl ToolsRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
        };
        registry.load_all_tools();
        registry
    }
    
    fn load_all_tools(&mut self) {
        let tools = vec![
            // AI Generation Tools
            Tool {
                id: "marketingblocks".to_string(),
                name: "MarketingBlocks".to_string(),
                category: ToolCategory::AiGeneration,
                description: "AI-powered marketing asset creation".to_string(),
                use_case: "Create landing pages, ads, emails, and more with AI".to_string(),
                pricing: "Paid".to_string(),
                url: Some("https://marketingblocks.ai".to_string()),
                alternatives: vec!["Jasper".to_string(), "Copy.ai".to_string()],
            },
            Tool {
                id: "synthesys".to_string(),
                name: "Synthesys".to_string(),
                category: ToolCategory::AiGeneration,
                description: "AI voice and video generation".to_string(),
                use_case: "Create AI spokesperson videos and voiceovers".to_string(),
                pricing: "Paid".to_string(),
                url: Some("https://synthesys.io".to_string()),
                alternatives: vec!["Synthesia".to_string(), "HeyGen".to_string()],
            },
            Tool {
                id: "artistly".to_string(),
                name: "Artistly".to_string(),
                category: ToolCategory::AiGeneration,
                description: "AI art and image generation".to_string(),
                use_case: "Generate logos, art, and designs for products".to_string(),
                pricing: "Paid".to_string(),
                url: None,
                alternatives: vec!["Midjourney".to_string(), "DALL-E".to_string()],
            },
            Tool {
                id: "chatgpt_canva".to_string(),
                name: "ChatGPT + Canva Plugin".to_string(),
                category: ToolCategory::AiGeneration,
                description: "Generate designs directly in Canva with AI".to_string(),
                use_case: "Create stock photos, designs, and social media graphics".to_string(),
                pricing: "Canva Pro + ChatGPT Plus".to_string(),
                url: Some("https://canva.com".to_string()),
                alternatives: vec!["Midjourney".to_string(),"Stockimg.ai".to_string()],
            },
            Tool {
                id: "midjourney".to_string(),
                name: "Midjourney".to_string(),
                category: ToolCategory::AiGeneration,
                description: "AI image generation via Discord".to_string(),
                use_case: "Create digital art, stickers, POD designs. Use --ar 293:151 for mugs".to_string(),
                pricing: "Subscription".to_string(),
                url: Some("https://midjourney.com".to_string()),
                alternatives: vec!["DALL-E 3".to_string(), "Stable Diffusion".to_string()],
            },
            Tool {
                id: "designbeast".to_string(),
                name: "DesignBeast".to_string(),
                category: ToolCategory::AiGeneration,
                description: "All-in-one design automation".to_string(),
                use_case: "Create logos, mockups, and designs quickly".to_string(),
                pricing: "One-time purchase".to_string(),
                url: None,
                alternatives: vec!["Canva".to_string(), "Adobe Express".to_string()],
            },
            Tool {
                id: "logoai".to_string(),
                name: "Logoai".to_string(),
                category: ToolCategory::AiGeneration,
                description: "AI logo generator".to_string(),
                use_case: "Generate professional logos for $10-25 each".to_string(),
                pricing: "Pay per logo".to_string(),
                url: Some("https://logoai.com".to_string()),
                alternatives: vec!["Looka".to_string(), "Brandmark".to_string()],
            },
            Tool {
                id: "looka".to_string(),
                name: "Looka".to_string(),
                category: ToolCategory::AiGeneration,
                description: "AI-powered brand identity".to_string(),
                use_case: "Create logos and complete brand kits".to_string(),
                pricing: "Freemium".to_string(),
                url: Some("https://looka.com".to_string()),
                alternatives: vec!["Logoai".to_string(), "Wix Logo Maker".to_string()],
            },
            
            // Research Tools
            Tool {
                id: "erank".to_string(),
                name: "eRank".to_string(),
                category: ToolCategory::Research,
                description: "Etsy SEO and market research tool".to_string(),
                use_case: "Research keywords, track competitors, find trending products".to_string(),
                pricing: "Freemium".to_string(),
                url: Some("https://erank.com".to_string()),
                alternatives: vec!["Marmalead".to_string(), "Everbee".to_string()],
            },
            Tool {
                id: "alura".to_string(),
                name: "Alura".to_string(),
                category: ToolCategory::Research,
                description: "Etsy analytics Chrome extension".to_string(),
                use_case: "See sales data for any Etsy shop while browsing".to_string(),
                pricing: "Subscription".to_string(),
                url: Some("https://alura.io".to_string()),
                alternatives: vec!["EtsyHunt".to_string(), "Sale Samurai".to_string()],
            },
            Tool {
                id: "sale_samurai".to_string(),
                name: "Sale Samurai".to_string(),
                category: ToolCategory::Research,
                description: "Etsy SEO optimization tool".to_string(),
                use_case: "Auto-fix SEO, find long-tail keywords, track rankings".to_string(),
                pricing: "Subscription".to_string(),
                url: Some("https://salesamurai.io".to_string()),
                alternatives: vec!["eRank".to_string(), "Marmalead".to_string()],
            },
            
            // Design Tools
            Tool {
                id: "canva".to_string(),
                name: "Canva".to_string(),
                category: ToolCategory::Design,
                description: "Graphic design platform".to_string(),
                use_case: "Create printables, social media, marketing materials".to_string(),
                pricing: "Freemium".to_string(),
                url: Some("https://canva.com".to_string()),
                alternatives: vec!["Adobe Express".to_string(), "Figma".to_string()],
            },
            Tool {
                id: "photoshop".to_string(),
                name: "Adobe Photoshop".to_string(),
                category: ToolCategory::Design,
                description: "Professional image editing".to_string(),
                use_case: "Convert patterns to smart objects, enhance AI images".to_string(),
                pricing: "Subscription".to_string(),
                url: Some("https://adobe.com/photoshop".to_string()),
                alternatives: vec!["GIMP".to_string(), "Affinity Photo".to_string()],
            },
            
            // E-commerce Platforms
            Tool {
                id: "etsy".to_string(),
                name: "Etsy".to_string(),
                category: ToolCategory::Ecommerce,
                description: "Handmade and digital product marketplace".to_string(),
                use_case: "Sell digital downloads, templates, printables".to_string(),
                pricing: "$0.20 listing + 6.5% fee".to_string(),
                url: Some("https://etsy.com/sell".to_string()),
                alternatives: vec!["Gumroad".to_string(), "Creative Market".to_string()],
            },
            Tool {
                id: "gumroad".to_string(),
                name: "Gumroad".to_string(),
                category: ToolCategory::Ecommerce,
                description: "Simple platform for creators".to_string(),
                use_case: "Sell digital products with minimal setup".to_string(),
                pricing: "10% fee".to_string(),
                url: Some("https://gumroad.com".to_string()),
                alternatives: vec!["Lemon Squeezy".to_string(), "Payhip".to_string()],
            },
            Tool {
                id: "ebay".to_string(),
                name: "eBay".to_string(),
                category: ToolCategory::Ecommerce,
                description: "Online auction and shopping site".to_string(),
                use_case: "Sell POD products, less focus on presentation".to_string(),
                pricing: "Listing + final value fees".to_string(),
                url: Some("https://ebay.com".to_string()),
                alternatives: vec!["Etsy".to_string(), "Amazon".to_string()],
            },
            
            // Automation Tools
            Tool {
                id: "slingly".to_string(),
                name: "Slingly".to_string(),
                category: ToolCategory::Automation,
                description: "E-commerce automation platform".to_string(),
                use_case: "Connect multiple marketplaces, automate listings".to_string(),
                pricing: "Subscription".to_string(),
                url: Some("https://slingly.com".to_string()),
                alternatives: vec!["Sellbrite".to_string(),"ChannelAdvisor".to_string()],
            },
            Tool {
                id: "wholesale_robot".to_string(),
                name: "Wholesale Robot".to_string(),
                category: ToolCategory::Automation,
                description: "Wholesale product sourcing".to_string(),
                use_case: "Find suppliers for physical products".to_string(),
                pricing: "Varies".to_string(),
                url: None,
                alternatives: vec!["Alibaba".to_string(),"DHgate".to_string()],
            },
            Tool {
                id: "automatepod".to_string(),
                name: "AutomatePOD".to_string(),
                category: ToolCategory::Automation,
                description: "Print-on-demand automation".to_string(),
                use_case: "Automate POD order processing and fulfillment".to_string(),
                pricing: "Subscription".to_string(),
                url: None,
                alternatives: vec!["Printful".to_string(),"SPOD".to_string()],
            },
            
            // Email Marketing
            Tool {
                id: "sendiio".to_string(),
                name: "Sendiio".to_string(),
                category: ToolCategory::EmailMarketing,
                description: "Email, SMS, and Facebook messenger marketing".to_string(),
                use_case: "Build email list, automate sequences".to_string(),
                pricing: "One-time or subscription".to_string(),
                url: Some("https://sendiio.com".to_string()),
                alternatives: vec!["Mailchimp".to_string(),"ConvertKit".to_string()],
            },
            Tool {
                id: "newoak".to_string(),
                name: "NewOak AI".to_string(),
                category: ToolCategory::EmailMarketing,
                description: "AI-powered email marketing".to_string(),
                use_case: "Automated email campaigns with AI optimization".to_string(),
                pricing: "Subscription".to_string(),
                url: None,
                alternatives: vec!["Klaviyo".to_string(),"ActiveCampaign".to_string()],
            },
            
            // Print on Demand
            Tool {
                id: "printful".to_string(),
                name: "Printful".to_string(),
                category: ToolCategory::PrintOnDemand,
                description: "On-demand printing and fulfillment".to_string(),
                use_case: "Print and ship products automatically".to_string(),
                pricing: "Free to start, pay per product".to_string(),
                url: Some("https://printful.com".to_string()),
                alternatives: vec!["Printify".to_string(),"Gooten".to_string()],
            },
            Tool {
                id: "teelaunch".to_string(),
                name: "TeeLaunch".to_string(),
                category: ToolCategory::PrintOnDemand,
                description: "POD for Shopify and Etsy".to_string(),
                use_case: "T-shirts, mugs, and unique POD products".to_string(),
                pricing: "Free, pay per product".to_string(),
                url: Some("https://teelaunch.com".to_string()),
                alternatives: vec!["Printful".to_string(),"SPOD".to_string()],
            },
            Tool {
                id: "teescape".to_string(),
                name: "TeeScape".to_string(),
                category: ToolCategory::PrintOnDemand,
                description: "POD automation tool".to_string(),
                use_case: "Bulk upload and manage POD designs".to_string(),
                pricing: "Subscription".to_string(),
                url: None,
                alternatives: vec!["Flying Upload".to_string(),"Podly".to_string()],
            },
            Tool {
                id: "subliminator".to_string(),
                name: "Subliminator".to_string(),
                category: ToolCategory::PrintOnDemand,
                description: "All-over print specialist".to_string(),
                use_case: "All-over print hoodies, leggings, unique items".to_string(),
                pricing: "Free, pay per product".to_string(),
                url: Some("https://subliminator.com".to_string()),
                alternatives: vec!["Contrado".to_string(),"Art of Where".to_string()],
            },
            
            // Social Media
            Tool {
                id: "layer_app".to_string(),
                name: "Layer App".to_string(),
                category: ToolCategory::SocialMedia,
                description: "Social media content creation".to_string(),
                use_case: "Create and schedule social media posts".to_string(),
                pricing: "Freemium".to_string(),
                url: None,
                alternatives: vec!["Buffer".to_string(),"Hootsuite".to_string()],
            },
            
            // Testing
            Tool {
                id: "usertesting".to_string(),
                name: "UserTesting".to_string(),
                category: ToolCategory::Testing,
                description: "User experience testing platform".to_string(),
                use_case: "Hire testers to record screen and give feedback".to_string(),
                pricing: "Pay per test".to_string(),
                url: Some("https://usertesting.com".to_string()),
                alternatives: vec!["Userlytics".to_string(),"TryMyUI".to_string()],
            },
            
            // Legal
            Tool {
                id: "bvpp".to_string(),
                name: "BVPP".to_string(),
                category: ToolCategory::Legal,
                description: "Business and marketing contracts".to_string(),
                use_case: "Marketing and distribution contract templates".to_string(),
                pricing: "Varies".to_string(),
                url: None,
                alternatives: vec!["LawDepot".to_string(),"Rocket Lawyer".to_string()],
            },
        ];
        
        for tool in tools {
            self.tools.insert(tool.id.clone(), tool);
        }
    }
    
    pub fn get(&self, id: &str) -> Option<&Tool> {
        self.tools.get(id)
    }
    
    pub fn list(&self) -> Vec<&Tool> {
        self.tools.values().collect()
    }
    
    pub fn by_category(&self, category: ToolCategory) -> Vec<&Tool> {
        self.tools.values()
            .filter(|t| t.category == category)
            .collect()
    }
    
    pub fn search(&self, query: &str) -> Vec<&Tool> {
        let query_lower = query.to_lowercase();
        self.tools.values()
            .filter(|t| {
                t.name.to_lowercase().contains(&query_lower) ||
                t.description.to_lowercase().contains(&query_lower) ||
                t.use_case.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
}
