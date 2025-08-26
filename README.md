# AI Model Catalog

Build-time AI model catalog for Rust. Fetches, normalizes, and embeds Large Language Model (LLM) metadata from providers (starting with OpenRouter) directly into your binary for zero-runtime lookups.

Find models by id, context window, pricing, capabilities (e.g., tools, structured outputs), tokenizer, and more—without network calls at runtime.

Keywords: AI model catalog for Rust, LLM registry, OpenRouter models, model metadata, context window, pricing, tokenizer, supported parameters, OpenAI, Anthropic, Google Gemini, Mistral, compile-time model index.

> Note: The crate is named `ai-model-catalog` (import as `ai_model_catalog`).

## Why this exists

The AI model landscape changes daily. New models, updated context lengths, different pricing, evolving parameters. Keeping your own registry in sync across multiple providers is error-prone and time-consuming.

This library:

- **Build-time fetch and embed**: Pulls the latest provider catalogs at build-time so your app ships with a fresh snapshot.
- **Consistent, normalized schema**: One data model across providers for simple, predictable lookups.
- **Zero runtime dependencies**: No network calls in production; fast, reliable, cache-friendly.

## Features

- **OpenRouter catalog**: Model ids, names, descriptions, context length, supported parameters, pricing, tokenizer, and top provider limits.
- **Tiny API surface**: Simple helpers to list ids and fetch a model by id.
- **Type-safe structs**: `OpenRouterModel`, `Architecture`, `Pricing`, `TopProvider`.
- **Id normalization**: Accepts `openrouter/<id>` or plain `<id>`, case-insensitive.
- **Opt-in data refresh**: Controlled via `AI_MODEL_CATALOG_BUILD` and `OPENROUTER_API_KEY`.

## Installation

Crates.io (soon):

```toml
[dependencies]
ai-model-catalog = "0.1"
```

Git (alternative):

```toml
[dependencies]
ai-model-catalog = { git = "https://github.com/agallardol/ai-model-catalog" }
```

Import path mapping: crate hyphens become underscores in Rust code.

- Crate name: `ai-model-catalog` → import path: `ai_model_catalog`

## Quickstart

Using the future crate name (recommended, once published):

```rust
use ai_model_catalog::{get_openrouter_model, list_openrouter_model_ids};

fn main() {
    // List all OpenRouter model IDs
    for model_id in list_openrouter_model_ids() {
        println!("{}", model_id);
    }

    // Fetch a specific model (id normalization is handled automatically)
    if let Some(model) = get_openrouter_model("openrouter/auto") {
        println!("Model: {}", model.name);
        println!("Context length: {:?}", model.context_length);
        if let Some(arch) = &model.architecture {
            println!("Tokenizer: {}", arch.tokenizer);
        }
        println!("Supported params: {:?}", model.supported_parameters);
    }
}
```

Another minimal example:

```rust
use ai_model_catalog::{get_openrouter_model, list_openrouter_model_ids};

fn main() {
    let total = list_openrouter_model_ids().count();
    println!("{} OpenRouter models embedded", total);

    if let Some(model) = get_openrouter_model("google/gemini-2.5-flash") {
        println!("{} => context {:?}", model.id, model.context_length);
    }
}
```

## Build-time data refresh

By default, the crate compiles with an empty index. To embed the latest OpenRouter catalog at build time, set the following environment variables:

```bash
# Embed public models only
AI_MODEL_CATALOG_BUILD=1 cargo build

# Embed public + your account's private models (if any)
AI_MODEL_CATALOG_BUILD=1 OPENROUTER_API_KEY="your-api-key" cargo build
```

Notes:

- Without `AI_MODEL_CATALOG_BUILD=1`, the embedded lists will be empty (API still works, just returns no data).
- The build script writes provider JSON to `OUT_DIR`, which is then compiled into your binary.
- Rebuild whenever you want to refresh the embedded catalog.

## API

Re-exports at crate root for convenience:

- `get_openrouter_model(id: &str) -> Option<&'static OpenRouterModel>`
- `list_openrouter_model_ids() -> impl Iterator<Item = &'static str>`
- Types: `OpenRouterModel`, `Architecture`, `Pricing`, `TopProvider`

Id normalization rules for `get_openrouter_model`:

- Case-insensitive
- Accepts ids with or without the `openrouter/` prefix

### Common recipes

List models that support tools:

```rust
use ai_model_catalog::{list_openrouter_model_ids, get_openrouter_model};

let tool_models: Vec<_> = list_openrouter_model_ids()
    .filter_map(|id| get_openrouter_model(id))
    .filter(|m| m.supported_parameters.iter().any(|p| p == "tools"))
    .map(|m| m.id.clone())
    .collect();
```

Find models with a context window >= 128k tokens:

```rust
use ai_model_catalog::{list_openrouter_model_ids, get_openrouter_model};

let big_context: Vec<_> = list_openrouter_model_ids()
    .filter_map(|id| get_openrouter_model(id))
    .filter(|m| m.context_length.unwrap_or(0) >= 128_000)
    .map(|m| (m.id.clone(), m.context_length))
    .collect();
```

## Data source and freshness

- Provider: **OpenRouter** (more providers coming)
- Contents: ids, names, descriptions, `context_length`, `supported_parameters`, `pricing`, tokenizer, top provider limits
- Freshness: controlled by you at build time; re-run the build to refresh

## Roadmap

- Additional providers (OpenAI, Anthropic, Mistral, Google, etc.)
- Unified cross-provider model id canonicalization
- Optional feature flags per provider
- Prebuilt, versioned snapshots

## Contributing

Contributions are very welcome. To add a provider, follow these steps:

1. Use `build/providers/template.rs` as a starting point.
2. Create `build/providers/<name>.rs` and implement its `SPEC` and `fetch` logic.
3. Register the provider in `build/providers/mod.rs` and add its `SPEC` to `ALL`.

Please run formatting and tests before submitting PRs.

## FAQ

**Why build-time instead of runtime?**

- Deterministic behavior in production, no network flakiness
- Much faster startup and queries (pure in-memory lookups)
- Simpler ops; no cache invalidation needed

**How do I include private models?**

- Provide `OPENROUTER_API_KEY` during build; your snapshot will include what your key can see

**Will the crate work without the env vars?**

- Yes, but the catalog will be empty. This is desirable for CI or offline builds.

**What about naming?**

- The repo and crate are "AI Model Catalog" (`ai-model-catalog` / `ai_model_catalog`).

---

If you build tools that select, compare, or route across LLMs, AI Model Catalog gives you a fast, zero-runtime dependency registry of model capabilities and limits.