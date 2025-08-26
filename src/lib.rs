//! Build-time models repository, organized by provider (OpenRouter first).
//!
//! Set `MODELS_REPOSITORY_BUILD=1` (optionally set `OPENROUTER_API_KEY`) before building
//! to fetch and embed the latest OpenRouter models catalog. At runtime, use
//! provider-specific lookups, e.g., `providers::openrouter::get_model(id)`.

pub mod providers;

// Re-export common OpenRouter types and helpers at the crate root for convenience.
pub use providers::openrouter::{
    get_model as get_openrouter_model, list_model_ids as list_openrouter_model_ids, Architecture,
    OpenRouterModel, Pricing, TopProvider,
};
