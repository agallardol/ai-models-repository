//! Template for adding a new provider to the build script.
//!
//! How to use:
//! - Copy this file to `build/providers/<your_name>.rs`.
//! - Replace placeholder names, URLs, types, and env vars.
//! - Add `pub mod <your_name>;` and include `<your_name>::SPEC` in `ALL` in `build/providers/mod.rs`.
//! - Run `MODELS_REPOSITORY_BUILD=1 cargo build` to refresh all providers.

use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::build_support::ProviderSpec;

// 1) Describe this provider in a SPEC constant.
pub const SPEC: ProviderSpec = ProviderSpec {
    name: "yourprovider",
    file_name: "models_yourprovider.json",
    fetch: fetch_models_map_json,
};

// 2) Define response/data types you care about.
//    Keep fields `#[serde(default)]` so unknown/missing data doesn’t break parsing.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
struct ApiResponse {
    #[serde(default)]
    data: Vec<Model>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
struct Model {
    #[serde(default)]
    id: String,
    // Add more fields as needed …
}

// 3) Implement the fetcher. It should return pretty JSON for a map or list.
pub fn fetch_models_map_json() -> Result<String, Box<dyn Error>> {
    // Example: replace with the real endpoint and optional auth.
    let url = "https://api.example.com/v1/models";

    // Optional: read an API key if the provider needs it.
    // let api_key = std::env::var("YOURPROVIDER_API_KEY").ok();

    let client = reqwest::blocking::Client::new();
    let mut req = client.get(url);
    // if let Some(k) = api_key { req = req.bearer_auth(k); }

    let resp = req.send()?;
    if !resp.status().is_success() {
        return Err(format!("HTTP {} from {}", resp.status(), url).into());
    }

    // Parse into your types, then reshape as needed.
    let json: ApiResponse = resp.json()?;

    // Option A: return as a map keyed by id
    // use std::collections::HashMap;
    // let mut map: HashMap<String, Model> = HashMap::with_capacity(json.data.len());
    // for m in json.data.into_iter() {
    //     map.insert(m.id.to_ascii_lowercase(), m);
    // }
    // let serialized = serde_json::to_string_pretty(&map)?;

    // Option B: return the raw list
    let serialized = serde_json::to_string_pretty(&json.data)?;

    Ok(serialized)
}

