mod hardware;
mod config;

use std::ffi::CString;
use std::os::raw::c_char;
use sysinfo::{System, SystemExt};
use crate::config::Config;

#[no_mangle]
pub extern "C" fn get_hardware_stats_json() -> *mut c_char {
    let mut sys = System::new_all(); // Cria um novo System
    let config = Config::default();  // Usa uma configuração padrão
    let stats = hardware::get_hardware_stats(&mut sys, &config);
    let json = serde_json::to_string(&stats).unwrap_or_else(|_| "{}".to_string());
    let c_string = CString::new(json).unwrap_or_else(|_| CString::new("{}").unwrap());
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn free_hardware_stats_json(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr); // Libera a memória
        }
    }
}