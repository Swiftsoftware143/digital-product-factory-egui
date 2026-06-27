//! Multi-LLM router for AI generation

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone)]
pub struct LLMRouter {
    client: Client,
    openai_key: String,
    anthropic_key: String,
    google_key: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LLMProfile {
    Creative,      // GPT-4o for creative writing
    Structured,    // Claude for structured data
    Professional,  // GPT-4 for professional tone
    Technical,     // Gemini for technical content
    Visual,        // GPT-4 Vision for image analysis
    Fast,          // GPT-3.5 for quick tasks
}

impl LLMProfile {
    pub fn model(&self) -> &'static str {
        match self {
            LLMProfile::Creative => "gpt-4o",
            LLMProfile::Structured => "claude-3-5-sonnet-20241022",
            LLMProfile::Professional => "gpt-4",
            LLMProfile::Technical => "gemini-1.5-pro",
            LLMProfile::Visual => "gpt-4o",
            LLMProfile::Fast => "gpt-3.5-turbo",
        }
    }
    
    pub fn provider(&self) -> Provider {
        match self {
            LLMProfile::Creative | LLMProfile::Professional | LLMProfile::Visual | LLMProfile::Fast => Provider::OpenAI,
            LLMProfile::Structured => Provider::Anthropic,
            LLMProfile::Technical => Provider::Google,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Provider {
    OpenAI,
    Anthropic,
    Google,
}

#[derive(Debug, Clone)]
pub struct GenerationRequest {
    pub profile: LLMProfile,
    pub prompt: String,
    pub system_prompt: Option<String>,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Clone)]
pub struct GenerationResponse {
    pub content: String,
    pub model: String,
    pub tokens_used: u32,
}

impl LLMRouter {
    pub fn new(openai_key: String, anthropic_key: String, google_key: String) -> Self {
        Self {
            client: Client::new(),
            openai_key,
            anthropic_key,
            google_key,
        }
    }
    
    pub async fn generate(&self, request: GenerationRequest) -> Result<GenerationResponse, String> {
        match request.profile.provider() {
            Provider::OpenAI => self.call_openai(request).await,
            Provider::Anthropic => self.call_anthropic(request).await,
            Provider::Google => self.call_google(request).await,
        }
    }
    
    async fn call_openai(&self, request: GenerationRequest) -> Result<GenerationResponse, String> {
        let system_prompt = request.system_prompt.unwrap_or_else(|| {
            "You are a helpful assistant for creating digital products.".to_string()
        });
        
        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.openai_key))
            .json(&json!({
                "model": request.profile.model(),
                "messages": [
                    {"role": "system", "content": system_prompt},
                    {"role": "user", "content": request.prompt}
                ],
                "temperature": request.temperature,
                "max_tokens": request.max_tokens,
            }))
            .send()
            .await
            .map_err(|e| format!("OpenAI request failed: {}", e))?;
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;
        
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("No content in response")?
            .to_string();
        
        let tokens = json["usage"]["total_tokens"].as_u64().unwrap_or(0) as u32;
        
        Ok(GenerationResponse {
            content,
            model: request.profile.model().to_string(),
            tokens_used: tokens,
        })
    }
    
    async fn call_anthropic(&self, request: GenerationRequest) -> Result<GenerationResponse, String> {
        let system_prompt = request.system_prompt.unwrap_or_else(|| {
            "You are a helpful assistant for creating digital products.".to_string()
        });
        
        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.anthropic_key)
            .header("anthropic-version", "2023-06-01")
            .json(&json!({
                "model": request.profile.model(),
                "max_tokens": request.max_tokens,
                "system": system_prompt,
                "messages": [
                    {"role": "user", "content": request.prompt}
                ],
                "temperature": request.temperature,
            }))
            .send()
            .await
            .map_err(|e| format!("Anthropic request failed: {}", e))?;
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| format!("Failed to parse Anthropic response: {}", e))?;
        
        let content = json["content"][0]["text"]
            .as_str()
            .ok_or("No content in response")?
            .to_string();
        
        let tokens = json["usage"]["input_tokens"].as_u64().unwrap_or(0) as u32 +
                     json["usage"]["output_tokens"].as_u64().unwrap_or(0) as u32;
        
        Ok(GenerationResponse {
            content,
            model: request.profile.model().to_string(),
            tokens_used: tokens,
        })
    }
    
    async fn call_google(&self, request: GenerationRequest) -> Result<GenerationResponse, String> {
        // Google Gemini API implementation
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            request.profile.model(),
            self.google_key
        );
        
        let response = self.client
            .post(&url)
            .json(&json!({
                "contents": [{
                    "parts": [{"text": request.prompt}]
                }],
                "generationConfig": {
                    "temperature": request.temperature,
                    "maxOutputTokens": request.max_tokens,
                }
            }))
            .send()
            .await
            .map_err(|e| format!("Google API request failed: {}", e))?;
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| format!("Failed to parse Google response: {}", e))?;
        
        let content = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or("No content in response")?
            .to_string();
        
        Ok(GenerationResponse {
            content,
            model: request.profile.model().to_string(),
            tokens_used: 0, // Google doesn't always return token counts
        })
    }
    
    /// Auto-select best profile based on task
    pub fn auto_select_profile(task: &str) -> LLMProfile {
        let task_lower = task.to_lowercase();
        
        if task_lower.contains("creative") || task_lower.contains("write") || task_lower.contains("story") {
            LLMProfile::Creative
        } else if task_lower.contains("structure") || task_lower.contains("data") || task_lower.contains("json") {
            LLMProfile::Structured
        } else if task_lower.contains("technical") || task_lower.contains("code") {
            LLMProfile::Technical
        } else if task_lower.contains("professional") || task_lower.contains("business") {
            LLMProfile::Professional
        } else if task_lower.contains("image") || task_lower.contains("visual") {
            LLMProfile::Visual
        } else {
            LLMProfile::Fast
        }
    }
}
