use crate::emulator::*;
use super::*;

use std::path::Path;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    text::Line,
    Frame,
};

pub struct Ui {
    exit: bool,
    emulator: Emulator,
}

impl Ui {
    pub fn new(emu: Emulator) -> Ui {
        Ui {
            exit: false,
            emulator: emu
        }
    }
}

impl Ui {
    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

    }
}

pub fn tui_main<P: AsRef<Path>>(rom_path: P) -> Result<(), String> {
    let mut ui = Ui::new(Emulator::new(rom_path));
    Ok(())
}
