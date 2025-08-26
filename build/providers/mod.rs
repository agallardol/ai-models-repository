use super::ProviderSpec;

pub mod openrouter;

pub use openrouter::SPEC as OPENROUTER;

// Registry of all providers
pub const ALL: &[ProviderSpec] = &[OPENROUTER];

// To add a new provider:
// 1) Create `build/providers/<name>.rs` (see `template.rs` for a starter).
// 2) Declare it here with `pub mod <name>;` and re-export `SPEC` if desired.
// 3) Add its `SPEC` to the `ALL` slice above.
//
// Example:
// pub mod myprovider;
// pub use myprovider::SPEC as MYPROVIDER;
// pub const ALL: &[ProviderSpec] = &[OPENROUTER, MYPROVIDER];
