use gstreamer as gst;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_gstreamer_version() -> String {
    // Initialize GStreamer
    match gst::init() {
        Ok(_) => {
            let version = gst::version_string();
            format!("GStreamer Version: {}", version)
        }
        Err(err) => format!("Failed to initialize GStreamer: {}", err),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_gstreamer_version])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
