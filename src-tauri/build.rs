use std::env;
use std::fs::{self, copy};
use std::path::Path;

fn main() {
    
    if cfg!(windows) {
        let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let lib_dir = Path::new(&dir).join("lib").join("windows").join("64");

        // Iterate over all .dll files in the lib_dir and copy them to the root directory
        for entry in fs::read_dir(&lib_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "dll") {
                let file_name = path.file_name().unwrap();
                let dest_path = Path::new(&dir).join(file_name);
                println!("cargo:rerun-if-changed={}", path.display()); // Watch for changes
                copy(&path, &dest_path).unwrap();
            }
        }

        // Add library directory to the linker search path
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
    }

    tauri_build::build()
}
