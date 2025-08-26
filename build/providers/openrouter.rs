use std::{collections::HashMap, env};

use serde::{Deserialize, Serialize};

use crate::build_support::ProviderSpec;

pub const SPEC: ProviderSpec = ProviderSpec {
    name: "openrouter",
    file_name: "models_openrouter.json",
    fetch: fetch_models_map_json,
};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
struct OpenRouterResponse {
    #[serde(default)]
    data: Vec<OpenRouterModel>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
struct OpenRouterModel {
    #[serde(default)]
    id: String,
    #[serde(default)]
    canonical_slug: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    created: Option<u64>,
    #[serde(default)]
    description: String,
    #[serde(default)]
    context_length: Option<u64>,
    #[serde(default)]
    architecture: Option<Architecture>,
    #[serde(default)]
    pricing: Option<Pricing>,
    #[serde(default)]
    top_provider: Option<TopProvider>,
    #[serde(default)]
    per_request_limits: Option<serde_json::Value>,
    #[serde(default)]
    supported_parameters: Vec<String>,
    #[serde(flatten)]
    extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
struct Architecture {
    #[serde(default)]
    input_modalities: Vec<String>,
    #[serde(default)]
    output_modalities: Vec<String>,
    #[serde(default)]
    tokenizer: String,
    #[serde(default)]
    instruct_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
struct Pricing {
    #[serde(default)]
    prompt: Option<String>,
    #[serde(default)]
    completion: Option<String>,
    #[serde(default)]
    request: Option<String>,
    #[serde(default)]
    image: Option<String>,
    #[serde(default)]
    web_search: Option<String>,
    #[serde(default)]
    internal_reasoning: Option<String>,
    #[serde(default)]
    input_cache_read: Option<String>,
    #[serde(default)]
    input_cache_write: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
struct TopProvider {
    #[serde(default)]
    context_length: Option<u64>,
    #[serde(default)]
    max_completion_tokens: Option<u64>,
    #[serde(default)]
    is_moderated: Option<bool>,
}

pub fn fetch_models_map_json() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://openrouter.ai/api/v1/models";
    let api_key = env::var("OPENROUTER_API_KEY").ok();

    let client = reqwest::blocking::Client::new();
    let mut req = client.get(url);
    if let Some(k) = api_key {
        req = req.bearer_auth(k);
    }

    let resp = req.send()?;
    if !resp.status().is_success() {
        return Err(format!("HTTP {} from {}", resp.status(), url).into());
    }

    let json: OpenRouterResponse = resp.json()?;
    let mut map: HashMap<String, OpenRouterModel> = HashMap::with_capacity(json.data.len());

    for model in json.data.into_iter() {
        let key = model.id.clone();
        map.insert(key, model);
    }

    let serialized = serde_json::to_string_pretty(&map)?;
    Ok(serialized)
}
