use once_cell::sync::OnceCell;
use std::sync::Arc;
use sdl3::keyboard::Keycode;
use crate::gui::inputs::GBKey;

pub static GLOB_SETTINGS : OnceCell<Arc<Settings>> = OnceCell::new();

#[derive(Debug)]
pub struct Settings {
    pub print_serial: bool,
    pub tui_enabled: bool,
    pub doctor_logs: bool,
    pub show_fps: bool,

    pub colors: Vec<u32>,
    
    pub gb_keys: Vec<(GBKey, Keycode)>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            print_serial: false,
            tui_enabled: false,
            doctor_logs: false,
            show_fps: true,
            colors: vec![0xFF9BBC0F, 0xFF8BAC0F, 0xFF306230, 0xFF0F380F, 0xFFFFFFFF], // Default Color Scheme
            gb_keys: vec![
                (GBKey::DpadU, Keycode::Z),
                (GBKey::DpadL, Keycode::Q),
                (GBKey::DpadD, Keycode::S),
                (GBKey::DpadR, Keycode::D),

                (GBKey::BtnA, Keycode::L),
                (GBKey::BtnB, Keycode::M),
                (GBKey::Start, Keycode::Return),
                (GBKey::Select, Keycode::Backspace),
            ],
        }
    }
}