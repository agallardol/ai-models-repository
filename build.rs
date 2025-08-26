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

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set by Cargo");
    let model_data_dir = PathBuf::from(manifest_dir).join("model_data");

    // Known providers live under build/providers. Add more modules there.
    let providers = build_support::providers::ALL;

    // If the build env is present, rebuild all providers.
    let rebuild_all = env::var_os("MODELS_REPOSITORY_BUILD").is_some();

    for p in providers.iter() {
        let model_data_path = model_data_dir.join(p.file_name);
        let out_path = out_dir.join(p.file_name);

        if rebuild_all {
            match (p.fetch)() {
                Ok(json) => {
                    if let Err(e) = fs::write(&model_data_path, &json) {
                        println!(
                            "cargo:warning=Failed to write {}: {e}",
                            model_data_path.display()
                        );
                    }
                    if let Err(e) = fs::write(&out_path, json) {
                        println!("cargo:warning=Failed to write {}: {e}", out_path.display());
                    }
                }
                Err(e) => {
                    println!("cargo:warning=Failed to fetch {}: {e}", p.name);
                    panic!("Failed to fetch {}: {e}", p.name);
                }
            }
        } else {
            let json = fs::read_to_string(&model_data_path).unwrap_or_else(|_| "{}".to_string());
            if let Err(e) = fs::write(&out_path, json) {
                println!("cargo:warning=Failed to write {}: {e}", out_path.display());
            }
        }
    }
}
