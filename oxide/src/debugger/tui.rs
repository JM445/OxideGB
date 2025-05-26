use crate::emulator::*;
use super::*;

use ratatui::{
    widgets::Block,
}

pub struct Ui {
    exit: bool,
    asm: Block,
    regs: Block,
    mem: Block,
    cmd: Block,
}

impl Ui {
    pub fn new() {
        Ui {
            exit: false,
            asm: Block{},
            regs: Self::def_regs(),
            mem: Block{},
        }
    }

    fn def_regs() -> Block {
        Block::default()
            .title(Line::from("Registers").left_aligned())
            .borders(Borders::ALL)
    }

    fn def_asm() -> Block {
        Block::default()
            .title(Line::from("Dissassembly").left_aligned())
            .borders(Borders::ALL)
    }

    fn def_memory() -> Block {
        Block::default()
            .title(Line::from("Memory Content").left_aligned())
            .borders(Borders::ALL)
    }

    fn def_cmd() -> Block {
        Block::default()
            .title(Line::from("Command Line").left_aligned())
            .borders(Borders::ALL)
    }

}

pub fn tui_main<P: AsRef<Path>>(rom_path: P) -> Result<(), String> {
    let mut emulator = Emulator::new();
    let mut ui = Ui::new();

}
