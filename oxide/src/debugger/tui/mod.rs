pub mod ui_logger;
pub mod ui_utils;
mod render;
mod parser;
mod mem_view;
mod lexer;

use super::full_debugger::*;
use super::*;
use crate::emulator::*;

use std::collections::VecDeque;
use std::path::Path;
use std::io;

use ratatui::{
    crossterm::event,
    crossterm::event::{Event, KeyCode},
    style::Style,
    text::Line,
    widgets::{Block, Borders}
};

use crate::debugger::dissassembler::CodeMap;
use tui_textarea::{Input, TextArea};

pub struct Ui<'a> {
    exit: bool,
    memory_watches: VecDeque<(u16, usize)>,
    emulator: Emulator,
    cmd_area: TextArea<'a>,
    debugger: FullDebugger,
    last_cmd: Option<String>,
    code_map: CodeMap,
}

impl<'a> Ui<'a> {
    pub fn new(emu: Emulator, dbg: FullDebugger) -> Ui<'a> {
        let mut textarea = TextArea::default();

        textarea.set_cursor_line_style(Style::default());
        textarea.set_block(Block::default()
                .title(Line::from("CMD").left_aligned())
                .borders(Borders::ALL));
        textarea.insert_str("> ");

        Ui {
            exit: false,
            memory_watches: VecDeque::new(),
            emulator: emu,
            cmd_area: textarea,
            debugger: dbg,
            last_cmd: None,
            code_map: CodeMap::new(),
        }
    }


    pub fn run(&mut self) -> io::Result<()> {
        let mut term = ratatui::init();

        while !self.exit {
            term.draw(|frame| self.draw(frame))?;
//            if event::poll(Duration::from_secs(0)).unwrap() {
            self.handle_events()?;
//            }
        }

        ratatui::restore();
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let e = event::read().unwrap();
        let input = Input::from(e.clone());

        if let Event::Key(event) = e {
            if event.kind == event::KeyEventKind::Press {
                match event.code {
                    KeyCode::Enter => self.handle_newline(),
                    _ => {self.cmd_area.input_without_shortcuts(input);},
                };
            }
        }
        Ok(())
    }

    fn handle_newline(&mut self) {
        let line = self.cmd_area.lines().last().unwrap_or(&"".into()).clone();
        self.cmd_area.insert_str("\n> ");
        if line != "" {
            self.parse_line(&line[2..]);
        }
    }

    fn tick(&mut self) {
        loop {
            self.emulator.tick(&mut self.debugger);
            self.debugger.tick();
            if self.debugger.should_stop(&self.emulator.cpu, &self.emulator.bus) {
                break;
            }
        }
    }
}

pub fn tui_main<P: AsRef<Path>>(rom_path: P, boot_path: P) -> Result<(), String> {
    let emu = Emulator::new(rom_path, boot_path)?;
    let dbg =  FullDebugger::new(emu.cpu.pc);
    let mut ui = Ui::new(emu, dbg);

    if let Ok(_) = ui.run() {
        Ok(())
    } else {
        Err("Run error".to_string())
    }
}
