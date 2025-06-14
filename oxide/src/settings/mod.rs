
use std::sync::{Arc, Mutex, LazyLock};
use once_cell::sync::OnceCell;

pub static GLOB_SETTINGS : OnceCell<Arc<Settings>> = OnceCell::new();

#[derive(Default, Debug)]
pub struct Settings {
    pub print_serial: bool,
    pub tui_enabled: bool,
}