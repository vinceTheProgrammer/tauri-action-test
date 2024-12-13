fn main() {
    println!("cargo::rustc-link-arg=-Wl,--exclude-libs=ALL");
    tauri_build::build()
}
