use std::error::Error;

// Shared provider descriptor used by the build script
pub struct ProviderSpec {
    pub name: &'static str,
    pub file_name: &'static str,
    pub fetch: fn() -> Result<String, Box<dyn Error>>,
}

pub mod providers;
