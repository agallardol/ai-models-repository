# AI Models Repository

A build-time generated repository of AI models, providing a static, compile-time available list of models and their metadata from various providers.

## The Problem

Many applications need to be aware of available AI models, but the landscape of models and providers is constantly changing. New models are released, existing ones are updated, and each has a unique set of parameters, including crucial details like context window size.

Managing this complexity yourself is a significant challenge. You need to constantly track changes across different providers, update your application, and ensure you're using the correct parameters for each model.

This library solves that problem by:

*   **Fetching model information at build-time:** Your application starts with a complete and up-to-date picture of the available models.
*   **Standardizing model metadata:** We provide a consistent structure for model information, so you don't have to deal with different API responses from each provider.
*   **Embedding the data:** The model information is embedded directly into your application, making it fast, reliable, and always available.

This means you can focus on building your application, not on the ever-changing world of AI models.

## Getting Started

Using the library is simple. Just add it to your `Cargo.toml`:

```toml
[dependencies]
ai-models-repository = "0.1.0"
```

Then, you can use the library to get information about models. For example, to get information about OpenRouter models:

```rust
use ai_models_repository::{get_openrouter_model, list_openrouter_model_ids};

fn main() {
    // List all OpenRouter model IDs
    for model_id in list_openrouter_model_ids() {
        println!("{}", model_id);
    }

    // Get a specific model
    if let Some(model) = get_openrouter_model("openrouter/auto") {
        println!("Model: {}", model.name);
        println!("Architecture: {:?}", model.architecture);
        println!("Top Provider: {:?}", model.top_provider);
    }
}
```

## Building the Project

To build the project and fetch the latest model information, you need to set the `MODELS_REPOSITORY_BUILD` environment variable.

```bash
MODELS_REPOSITORY_BUILD=1 cargo build
```

If you want to access non-public models from OpenRouter, you also need to set the `OPENROUTER_API_KEY` environment variable.

```bash
MODELS_REPOSITORY_BUILD=1 OPENROUTER_API_KEY="your-api-key" cargo build
```

If you don't set `MODELS_REPOSITORY_BUILD`, the library will build with empty model lists.

## Contributing

This project is open source and contributions are welcome! If you want to add a new provider, please follow these steps:

1.  Use `build/providers/template.rs` as a starting point.
2.  Create `build/providers/<name>.rs` and fill in the `SPEC` and `fetch` functions.
3.  Register your new provider in `build/providers/mod.rs` and add its `SPEC` to the `ALL` list.