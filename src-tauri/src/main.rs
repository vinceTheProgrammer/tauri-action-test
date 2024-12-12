use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows::Win32::System::LibraryLoader::SetDllDirectoryW;

fn set_dll_directory(path: &str) {
    let wide: Vec<u16> = OsStr::new(path).encode_wide().chain(Some(0)).collect();
    unsafe {
        SetDllDirectoryW(wide.as_ptr());
    }
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Set the DLL directory to "dlls" (relative to the executable)
    set_dll_directory("dlls");
    
    tauri_action_test_lib::run()
}
