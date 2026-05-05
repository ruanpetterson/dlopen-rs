use std::fs;
use std::path::Path;

use animal_core::Animal;

const ANIMAL_PLUGIN_DIR: &str = env!("ANIMAL_PLUGIN_DIR");
const ANIMAL_PLUGIN_PROFILE: &str = env!("ANIMAL_PLUGIN_PROFILE");

fn main() {
    let mut plugins = vec![];

    let path = Path::new(ANIMAL_PLUGIN_DIR).join(ANIMAL_PLUGIN_PROFILE);

    for entry in fs::read_dir(&path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let Some(ext) = path.extension() else {
            continue;
        };

        let Some("dylib" | "so") = ext.to_str() else {
            continue;
        };

        let plugin = <dyn Animal>::load_library(path.to_str().unwrap()).build();
        plugins.push(plugin);
    }

    for plugin in plugins {
        println!("{}", plugin.say());
    }
}
