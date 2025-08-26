use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

// The build script writes `models_openrouter.json` to OUT_DIR.
const MODELS_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/models_openrouter.json"));

static MODELS: Lazy<HashMap<String, OpenRouterModel>> = Lazy::new(|| {
    serde_json::from_str::<HashMap<String, OpenRouterModel>>(MODELS_JSON).unwrap_or_default()
});

#[derive(Debug, Clone, Deserialize, Default)]
pub struct OpenRouterModel {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub canonical_slug: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub created: Option<u64>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub context_length: Option<u64>,
    #[serde(default)]
    pub architecture: Option<Architecture>,
    #[serde(default)]
    pub pricing: Option<Pricing>,
    #[serde(default)]
    pub top_provider: Option<TopProvider>,
    #[serde(default)]
    pub per_request_limits: Option<serde_json::Value>,
    #[serde(default)]
    pub supported_parameters: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Architecture {
    #[serde(default)]
    pub input_modalities: Vec<String>,
    #[serde(default)]
    pub output_modalities: Vec<String>,
    #[serde(default)]
    pub tokenizer: String,
    #[serde(default)]
    pub instruct_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Pricing {
    #[serde(default)]
    pub prompt: Option<String>,
    #[serde(default)]
    pub completion: Option<String>,
    #[serde(default)]
    pub request: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub web_search: Option<String>,
    #[serde(default)]
    pub internal_reasoning: Option<String>,
    #[serde(default)]
    pub input_cache_read: Option<String>,
    #[serde(default)]
    pub input_cache_write: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TopProvider {
    #[serde(default)]
    pub context_length: Option<u64>,
    #[serde(default)]
    pub max_completion_tokens: Option<u64>,
    #[serde(default)]
    pub is_moderated: Option<bool>,
}

/// Returns the OpenRouter model metadata by a model id.
/// Normalizes key: lowercases and strips `openrouter/` prefix.
pub fn get_model(id: &str) -> Option<&'static OpenRouterModel> {
    let lc = id.to_ascii_lowercase();
    let key = lc.strip_prefix("openrouter/").unwrap_or(&lc);
    MODELS.get(key)
}

/// Returns all known OpenRouter model ids (normalized) embedded at build time.
pub fn list_model_ids() -> impl Iterator<Item = &'static str> {
    static IDS: Lazy<Vec<&'static str>> = Lazy::new(|| {
        MODELS
            .keys()
            .map(|k| Box::leak(k.clone().into_boxed_str()) as &'static str)
            .collect()
    });
    IDS.iter().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_handles_normalization() {
        assert!(get_model("openrouter/some-model").is_none());
        assert!(get_model("SOME-MODEL").is_none());
    }

    #[test]
    fn deserializes_minimal_model() {
        let json = r#"{
            "id": "google/gemini-2.5-pro-preview",
            "canonical_slug": "google/gemini-2_5-pro-preview",
            "name": "Gemini 2.5 Pro Preview",
            "description": "A capable multimodal model",
            "supported_parameters": [],
            "architecture": {
                "input_modalities": ["text"],
                "output_modalities": ["text"],
                "tokenizer": "wordpiece",
                "instruct_type": null
            }
        }"#;
        let model: OpenRouterModel = serde_json::from_str(json).expect("parse minimal model");
        assert_eq!(model.id, "google/gemini-2.5-pro-preview");
        assert_eq!(model.name, "Gemini 2.5 Pro Preview");
        assert!(model.context_length.is_none());
        let arch = model.architecture.expect("architecture present");
        assert_eq!(arch.input_modalities, vec!["text"]);
        assert_eq!(arch.output_modalities, vec!["text"]);
        assert_eq!(arch.tokenizer, "wordpiece");
        assert!(arch.instruct_type.is_none());
    }

    #[test]
    fn deserializes_full_pricing_and_limits() {
        let json = r#"{
            "id": "openai/gpt-4o",
            "canonical_slug": "openai/gpt-4o",
            "name": "GPT-4o",
            "description": "Omni model",
            "context_length": 128000,
            "supported_parameters": ["tools", "max_tokens", "temperature"],
            "pricing": {
                "prompt": "0.005",
                "completion": "0.015",
                "request": "0",
                "image": "0.01",
                "web_search": "0.02",
                "internal_reasoning": "0",
                "input_cache_read": "0.001",
                "input_cache_write": "0.002"
            },
            "top_provider": {
                "context_length": 128000,
                "max_completion_tokens": 4096,
                "is_moderated": true
            }
        }"#;
        let model: OpenRouterModel = serde_json::from_str(json).expect("parse full model");
        assert_eq!(model.context_length, Some(128000));
        let pricing = model.pricing.expect("pricing present");
        assert_eq!(pricing.prompt.as_deref(), Some("0.005"));
        assert_eq!(pricing.completion.as_deref(), Some("0.015"));
        let tp = model.top_provider.expect("top provider present");
        assert_eq!(tp.context_length, Some(128000));
        assert_eq!(tp.max_completion_tokens, Some(4096));
        assert_eq!(tp.is_moderated, Some(true));
    }

    #[test]
    fn supported_parameters_examples_present() {
        let json = r#"{
            "id": "perplexity/sonar-reasoning",
            "canonical_slug": "perplexity/sonar-reasoning",
            "name": "Sonar Reasoning",
            "description": "Reasoning-optimized model",
            "supported_parameters": [
                "tools", "tool_choice", "max_tokens", "temperature", "top_p",
                "reasoning", "include_reasoning", "structured_outputs", "response_format",
                "stop", "frequency_penalty", "presence_penalty", "seed"
            ],
            "architecture": {
                "input_modalities": ["text"],
                "output_modalities": ["text"],
                "tokenizer": "bpe",
                "instruct_type": "openai"
            }
        }"#;
        let model: OpenRouterModel = serde_json::from_str(json).expect("parse supported params");
        let params = model.supported_parameters;
        assert!(params.contains(&"tools".to_string()));
        assert!(params.contains(&"tool_choice".to_string()));
        assert!(params.contains(&"structured_outputs".to_string()));
        assert!(params.contains(&"response_format".to_string()));
        assert!(params.contains(&"seed".to_string()));
    }
}
