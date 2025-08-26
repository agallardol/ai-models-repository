//! Providers module: add new providers in submodules (e.g., `openrouter`).

pub mod openrouter;

/// Supported provider identifiers.
pub enum Provider {
    OpenRouter,
}
