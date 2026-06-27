//! Template registry and engine

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub trending_score: u32,
}

pub struct TemplateRegistry {
    templates: Vec<Template>,
}

impl TemplateRegistry {
    pub fn load() -> Self {
        // TODO: Load from embedded JSON or database
        let templates = vec![
            Template {
                id: "planner_minimal".to_string(),
                name: "Minimal Planner".to_string(),
                description: "Clean, modern planner design".to_string(),
                category: "planner".to_string(),
                tags: vec!["minimal".to_string(), "modern".to_string()],
                trending_score: 85,
            },
            Template {
                id: "journal_gratitude".to_string(),
                name: "Gratitude Journal".to_string(),
                description: "Daily gratitude practice journal".to_string(),
                category: "journal".to_string(),
                tags: vec!["wellness".to_string(), "mindfulness".to_string()],
                trending_score: 92,
            },
        ];
        
        Self { templates }
    }
    
    pub fn get(&self, id: &str) -> Option<&Template> {
        self.templates.iter().find(|t| t.id == id)
    }
    
    pub fn list(&self) -> &[Template] {
        &self.templates
    }
    
    pub fn by_category(&self, category: &str) -> Vec<&Template> {
        self.templates.iter().filter(|t| t.category == category).collect()
    }
}
