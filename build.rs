use std::{env, fs, path::PathBuf};

// Split build logic into a small module tree under ./build
#[path = "build/mod.rs"]
mod build_support;

fn main() {
    println!("cargo:rerun-if-env-changed=MODELS_REPOSITORY_BUILD");
    println!("cargo:rerun-if-env-changed=OPENROUTER_API_KEY");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set by Cargo");
    let out_dir = PathBuf::from(out_dir);

    // Known providers live under build/providers. Add more modules there.
    let providers = build_support::providers::ALL;

    // Ensure every provider has a file, default `{}`.
    for p in providers.iter() {
        let path = out_dir.join(p.file_name);
        if let Err(e) = fs::write(&path, "{}") {
            println!("cargo:warning=Failed to initialize {}: {e}", path.display());
        }
    }

    // If the build env is present, rebuild all providers.
    let rebuild_all = env::var_os("MODELS_REPOSITORY_BUILD").is_some();

    // Fetch for selected providers (all when env is present) and overwrite their files.
    for p in providers.iter() {
        if !rebuild_all { continue; }
        match (p.fetch)() {
            Ok(json) => {
                let path = out_dir.join(p.file_name);
                if let Err(e) = fs::write(&path, json) {
                    println!("cargo:warning=Failed to write {}: {e}", path.display());
                }
            }
            Err(e) => {
                println!("cargo:warning=Failed to fetch {}: {e}", p.name);
                panic!("Failed to fetch {}: {e}", p.name);
            }
        }
    }
}
// Provider types and implementations live under ./build
