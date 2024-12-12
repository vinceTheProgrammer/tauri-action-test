use gstreamer as gst;
use std::ptr;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows::Win32::System::LibraryLoader::SetDllDirectoryW;

fn set_dll_directory(path: &str) {
    let wide: Vec<u16> = OsStr::new(path).encode_wide().chain(Some(0)).collect();
    unsafe {
        SetDllDirectoryW(wide.as_ptr());
    }
}

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
    // Set the DLL directory to "dlls" (relative to the executable)
    set_dll_directory("dlls");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_gstreamer_version])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
