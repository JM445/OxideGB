use once_cell::sync::OnceCell;
use std::sync::Arc;

pub static GLOB_SETTINGS : OnceCell<Arc<Settings>> = OnceCell::new();

#[derive(Default, Debug)]
pub struct Settings {
    pub print_serial: bool,
    pub tui_enabled: bool,
}