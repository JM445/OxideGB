use once_cell::sync::OnceCell;
use std::sync::Arc;

pub static GLOB_SETTINGS : OnceCell<Arc<Settings>> = OnceCell::new();

#[derive(Debug)]
pub struct Settings {
    pub print_serial: bool,
    pub tui_enabled: bool,
    pub doctor_logs: bool,

    pub colors: Vec<u32>
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            print_serial: false,
            tui_enabled: false,
            doctor_logs: false,
            colors: vec![0xFF9BBC0F, 0xFF8BAC0F, 0xFF306230, 0xFF0F380F, 0xFFFFFFFF] // Default Color Scheme
        }
    }
}