# ai-models-repository

Build-time model metadata is fetched by the Cargo build script when the `MODELS_REPOSITORY_BUILD` environment variable is present (any value). When set, all supported providers are refreshed and their JSON maps are generated under `OUT_DIR`.

- Set `MODELS_REPOSITORY_BUILD=1` when running `cargo build` to fetch all providers.
- Optionally set `OPENROUTER_API_KEY` to access non-public models via OpenRouter.

If `MODELS_REPOSITORY_BUILD` is not set, the build initializes empty JSON files for providers and skips network fetches.

Provider development
- Add a provider using `build/providers/template.rs` as a starting point.
- Create `build/providers/<name>.rs`, fill in `SPEC` and `fetch`.
- Register it in `build/providers/mod.rs` and add its `SPEC` to `ALL`.
