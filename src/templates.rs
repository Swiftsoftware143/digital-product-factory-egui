//! Template system for digital products

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: TemplateCategory,
    pub tags: Vec<String>,
    pub trending_score: u32,
    pub seasonal_peak: Option<String>,
    pub prompt_template: String,
    pub output_format: OutputFormat,
    pub parameters: Vec<TemplateParameter>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TemplateCategory {
    Planner,
    Journal,
    Spreadsheet,
    Guide,
    Resume,
    Cookbook,
    Business,
    Legal,
    Other,
}

impl TemplateCategory {
    pub fn name(&self) -> &'static str {
        match self {
            TemplateCategory::Planner => "Planner",
            TemplateCategory::Journal => "Journal",
            TemplateCategory::Spreadsheet => "Spreadsheet",
            TemplateCategory::Guide => "Guide",
            TemplateCategory::Resume => "Resume",
            TemplateCategory::Cookbook => "Cookbook",
            TemplateCategory::Business => "Business",
            TemplateCategory::Legal => "Legal",
            TemplateCategory::Other => "Other",
        }
    }
    
    pub fn all() -> Vec<TemplateCategory> {
        vec![
            TemplateCategory::Planner,
            TemplateCategory::Journal,
            TemplateCategory::Spreadsheet,
            TemplateCategory::Guide,
            TemplateCategory::Resume,
            TemplateCategory::Cookbook,
            TemplateCategory::Business,
            TemplateCategory::Legal,
        ]
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OutputFormat {
    Markdown,
    Html,
    Pdf,
    Docx,
    Xlsx,
    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateParameter {
    pub name: String,
    pub description: String,
    pub param_type: ParameterType,
    pub required: bool,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    Text,
    Number,
    Select(Vec<String>),
    Boolean,
    Color,
    Date,
}

pub struct TemplateRegistry {
    templates: HashMap<String, Template>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            templates: HashMap::new(),
        };
        registry.load_builtin_templates();
        registry
    }
    
    fn load_builtin_templates(&mut self) {
        let templates = vec![
            Template {
                id: "planner_daily".to_string(),
                name: "Daily Planner".to_string(),
                description: "A comprehensive daily planner with schedule, tasks, and notes".to_string(),
                category: TemplateCategory::Planner,
                tags: vec!["planner".to_string(), "daily".to_string(), "productivity".to_string()],
                trending_score: 95,
                seasonal_peak: Some("january".to_string()),
                prompt_template: "Create a daily planner with the following sections:
1. Morning routine ({morning_routine})
2. Hourly schedule (6am-10pm)
3. Top 3 priorities
4. Task list
5. Notes section
6. Evening reflection

Style: {style}
Color scheme: {color_scheme}".to_string(),
                output_format: OutputFormat::Pdf,
                parameters: vec![
                    TemplateParameter {
                        name: "morning_routine".to_string(),
                        description: "Morning routine duration".to_string(),
                        param_type: ParameterType::Select(vec!["15 min".to_string(), "30 min".to_string(), "1 hour".to_string()]),
                        required: true,
                        default: Some("30 min".to_string()),
                    },
                    TemplateParameter {
                        name: "style".to_string(),
                        description: "Planner style".to_string(),
                        param_type: ParameterType::Select(vec!["Minimal".to_string(), "Decorative".to_string(), "Professional".to_string()]),
                        required: true,
                        default: Some("Minimal".to_string()),
                    },
                    TemplateParameter {
                        name: "color_scheme".to_string(),
                        description: "Color scheme".to_string(),
                        param_type: ParameterType::Color,
                        required: false,
                        default: Some("#4A90D9".to_string()),
                    },
                ],
            },
            Template {
                id: "gratitude_journal".to_string(),
                name: "Gratitude Journal".to_string(),
                description: "Daily gratitude practice journal with prompts".to_string(),
                category: TemplateCategory::Journal,
                tags: vec!["journal".to_string(), "gratitude".to_string(), "wellness".to_string()],
                trending_score: 88,
                seasonal_peak: Some("november".to_string()),
                prompt_template: "Create a gratitude journal with:
1. Daily gratitude prompts ({prompt_count} prompts)
2. Reflection section
3. Weekly summary
4. Monthly review

Theme: {theme}
Pages: {page_count}".to_string(),
                output_format: OutputFormat::Pdf,
                parameters: vec![
                    TemplateParameter {
                        name: "prompt_count".to_string(),
                        description: "Number of daily prompts".to_string(),
                        param_type: ParameterType::Select(vec!["3".to_string(), "5".to_string(), "10".to_string()]),
                        required: true,
                        default: Some("3".to_string()),
                    },
                    TemplateParameter {
                        name: "theme".to_string(),
                        description: "Journal theme".to_string(),
                        param_type: ParameterType::Select(vec!["Nature".to_string(), "Minimal".to_string(), "Colorful".to_string()]),
                        required: true,
                        default: Some("Nature".to_string()),
                    },
                    TemplateParameter {
                        name: "page_count".to_string(),
                        description: "Number of pages".to_string(),
                        param_type: ParameterType::Select(vec!["30".to_string(), "90".to_string(), "365".to_string()]),
                        required: true,
                        default: Some("90".to_string()),
                    },
                ],
            },
            Template {
                id: "budget_tracker".to_string(),
                name: "Budget Tracker".to_string(),
                description: "Monthly budget spreadsheet with categories and charts".to_string(),
                category: TemplateCategory::Spreadsheet,
                tags: vec!["budget".to_string(), "finance".to_string(), "spreadsheet".to_string()],
                trending_score: 92,
                seasonal_peak: Some("january".to_string()),
                prompt_template: "Create a budget tracker spreadsheet with:
1. Income tracking
2. Expense categories: {categories}
3. Monthly summary
4. Charts and graphs
5. Annual overview

Currency: {currency}
Complexity: {complexity}".to_string(),
                output_format: OutputFormat::Xlsx,
                parameters: vec![
                    TemplateParameter {
                        name: "categories".to_string(),
                        description: "Expense categories".to_string(),
                        param_type: ParameterType::Text,
                        required: true,
                        default: Some("Housing, Food, Transport, Entertainment, Savings".to_string()),
                    },
                    TemplateParameter {
                        name: "currency".to_string(),
                        description: "Currency symbol".to_string(),
                        param_type: ParameterType::Select(vec!["$".to_string(), "€".to_string(), "£".to_string(), "¥".to_string()]),
                        required: true,
                        default: Some("$".to_string()),
                    },
                    TemplateParameter {
                        name: "complexity".to_string(),
                        description: "Complexity level".to_string(),
                        param_type: ParameterType::Select(vec!["Simple".to_string(), "Detailed".to_string(), "Advanced".to_string()]),
                        required: true,
                        default: Some("Detailed".to_string()),
                    },
                ],
            },
            Template {
                id: "freelance_contract".to_string(),
                name: "Freelance Contract".to_string(),
                description: "Professional freelance service agreement".to_string(),
                category: TemplateCategory::Legal,
                tags: vec!["contract".to_string(), "freelance".to_string(), "legal".to_string()],
                trending_score: 85,
                seasonal_peak: None,
                prompt_template: "Create a freelance contract with:
1. Parties: {client_name} and {freelancer_name}
2. Services: {service_description}
3. Payment: {payment_terms}
4. Timeline: {timeline}
5. Revisions: {revision_count}
6. Jurisdiction: {jurisdiction}

Include standard clauses for intellectual property, termination, and dispute resolution.".to_string(),
                output_format: OutputFormat::Docx,
                parameters: vec![
                    TemplateParameter {
                        name: "client_name".to_string(),
                        description: "Client name".to_string(),
                        param_type: ParameterType::Text,
                        required: true,
                        default: None,
                    },
                    TemplateParameter {
                        name: "freelancer_name".to_string(),
                        description: "Freelancer name".to_string(),
                        param_type: ParameterType::Text,
                        required: true,
                        default: None,
                    },
                    TemplateParameter {
                        name: "service_description".to_string(),
                        description: "Description of services".to_string(),
                        param_type: ParameterType::Text,
                        required: true,
                        default: None,
                    },
                    TemplateParameter {
                        name: "payment_terms".to_string(),
                        description: "Payment terms".to_string(),
                        param_type: ParameterType::Select(vec!["50% upfront, 50% on completion".to_string(), "100% upfront".to_string(), "100% on completion".to_string(), "Monthly billing".to_string()]),
                        required: true,
                        default: Some("50% upfront, 50% on completion".to_string()),
                    },
                    TemplateParameter {
                        name: "timeline".to_string(),
                        description: "Project timeline".to_string(),
                        param_type: ParameterType::Text,
                        required: true,
                        default: Some("30 days".to_string()),
                    },
                    TemplateParameter {
                        name: "revision_count".to_string(),
                        description: "Number of revisions included".to_string(),
                        param_type: ParameterType::Select(vec!["1".to_string(), "2".to_string(), "3".to_string(), "Unlimited".to_string()]),
                        required: true,
                        default: Some("2".to_string()),
                    },
                    TemplateParameter {
                        name: "jurisdiction".to_string(),
                        description: "Governing law jurisdiction".to_string(),
                        param_type: ParameterType::Text,
                        required: true,
                        default: Some("California, USA".to_string()),
                    },
                ],
            },
        ];
        
        for template in templates {
            self.templates.insert(template.id.clone(), template);
        }
    }
    
    pub fn get(&self, id: &str) -> Option<&Template> {
        self.templates.get(id)
    }
    
    pub fn list(&self) -> Vec<&Template> {
        self.templates.values().collect()
    }
    
    pub fn by_category(&self, category: TemplateCategory) -> Vec<&Template> {
        self.templates.values()
            .filter(|t| t.category == category)
            .collect()
    }
    
    pub fn search(&self, query: &str) -> Vec<&Template> {
        let query_lower = query.to_lowercase();
        self.templates.values()
            .filter(|t| {
                t.name.to_lowercase().contains(&query_lower) ||
                t.description.to_lowercase().contains(&query_lower) ||
                t.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
    
    pub fn trending(&self, limit: usize) -> Vec<&Template> {
        let mut templates: Vec<_> = self.templates.values().collect();
        templates.sort_by(|a, b| b.trending_score.cmp(&a.trending_score));
        templates.into_iter().take(limit).collect()
    }
}
