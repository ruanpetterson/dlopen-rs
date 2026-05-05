use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde::Deserialize;

#[derive(Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    name: String,
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let profile = env::var("PROFILE").unwrap(); // "debug" or "release"

    let plugins_dir = manifest_dir.join("plugins");
    fs::create_dir_all(&plugins_dir).unwrap();

    let animals_dir = manifest_dir.join("crates").join("animal");

    println!("cargo::rerun-if-changed={}", animals_dir.display());

    for entry in fs::read_dir(&animals_dir).unwrap() {
        let entry = entry.unwrap();

        let plugin_dir = entry.path();
        if !plugin_dir.is_dir() {
            continue;
        }

        let manifest_path = plugin_dir.join("Cargo.toml");
        if !manifest_path.exists() {
            continue;
        }

        println!("cargo::rerun-if-changed={}", manifest_path.display());
        println!(
            "cargo::rerun-if-changed={}",
            plugin_dir.join("src").display()
        );

        let package_name = {
            let cargo_toml_text = fs::read_to_string(&manifest_path).unwrap();
            let cargo_toml: CargoToml = toml::from_str(&cargo_toml_text).unwrap();
            cargo_toml.package.name
        };

        let mut cmd = Command::new("cargo");
        cmd.current_dir(&manifest_dir)
            .env("CARGO_TARGET_DIR", &plugins_dir)
            .args(["build", "-p", &package_name]);

        if profile == "release" {
            cmd.arg("--release");
        }

        let status = cmd.status().unwrap();

        if !status.success() {
            panic!("failed to build plugin package `{package_name}`");
        }

        println!(
            "cargo::rustc-env=ANIMAL_PLUGIN_DIR={}",
            plugins_dir.display(),
        );
        println!("cargo::rustc-env=ANIMAL_PLUGIN_PROFILE={profile}");

        println!("cargo::warning=Built plugin {package_name}");
    }
}
