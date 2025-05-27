use crate::emulator::*;
use super::*;

use self::dissassembler::*;

use std::{fmt, io};
use std::path::Path;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    widgets::{Block, Borders, Paragraph, Padding},
    text::{Line, Span},
    Frame,
    style::Stylize,
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

        let top_down = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(100),
                Constraint::Min(8)
            ]).split(area);

        let down_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(100),
                Constraint::Min(44)
            ]).split(top_down[1]);

        let top_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(50),
                Constraint::Percentage(100)
            ]).split(top_down[0]);

        frame.render_widget(self.draw_dissassembly(top_split[0].height), top_split[0]);
        frame.render_widget(
            Block::default()
                .title(Line::from("Memory").right_aligned())
                .borders(Borders::ALL), top_split[1]);
        frame.render_widget(
            Block::default()
                .title(Line::from("CMD").left_aligned())
                .borders(Borders::ALL), down_split[0]);
        frame.render_widget(self.draw_registers(), down_split[1]);
    }

    pub fn draw_dissassembly(&self, size: u16) -> Paragraph {
        let mut lines = Vec::new();
        let mut cur_sp = self.emulator.cpu.sp;
        let mem = &self.emulator.bus;

        for _ in 0..size {
            let bytes = mem.get_instruction(cur_sp);
            let len = get_instruction_length(bytes[0]) as u16;
            lines.push(Line::from(vec![
                format!("{:#06X} | ", cur_sp).into(),
                format!("{:<width$}", disassemble(&bytes), width=20).into(),
                "| ".into(),
                bytes.iter().take(len as usize).map(|x| format!("{:#04X}", x)).collect::<Vec<_>>().join(" ").into(),
            ]));
            cur_sp += len;
        }
        Paragraph::new(lines).block(Block::default()
                                    .title(Line::from("Disassembly").left_aligned())
                                    .borders(Borders::ALL)
                                    .padding(Padding::uniform(1)))
                             .alignment(Alignment::Left)
    }

    pub fn draw_registers(&self) -> Paragraph {
        let cpu = &self.emulator.cpu;
        let lines : Vec<Line>= vec![
            Line::from(vec![
                "A: ".blue().bold().into(),  format!("{:#04X}", cpu.a).into(), "  ".into(),
                "F: ".blue().bold().into(),  format!("{:#04X}", cpu.f).into(), "  ".into(),
                "AF: ".blue().bold().into(), format!("{:#06X}", cpu.read16(Reg16::AF)).into(), "  ".into(),
                "PC: ".blue().bold().into(), format!("{:#06X}", cpu.pc).into(),
            ]),
            Line::from(vec![
                "B: ".blue().bold().into(),  format!("{:#04X}", cpu.b).into(), "  ".into(),
                "C: ".blue().bold().into(),  format!("{:#04X}", cpu.c).into(), "  ".into(),
                "BC: ".blue().bold().into(), format!("{:#06X}", cpu.read16(Reg16::BC)).into(), "  ".into(),
                "SP: ".blue().bold().into(), format!("{:#06X}", cpu.sp).into(),
            ]),
            Line::from(vec![
                "D: ".blue().bold().into(),  format!("{:#04X}", cpu.d).into(), "  ".into(),
                "E: ".blue().bold().into(),  format!("{:#04X}", cpu.e).into(), "  ".into(),
                "DE: ".blue().bold().into(), format!("{:#06X}", cpu.read16(Reg16::DE)).into(), "             ".into(),
            ]),
            Line::from(vec![
                "H: ".blue().bold().into(),  format!("{:#04X}", cpu.h).into(), "  ".into(),
                "L: ".blue().bold().into(),  format!("{:#04X}", cpu.l).into(), "  ".into(),
                "HL: ".blue().bold().into(), format!("{:#06X}", cpu.read16(Reg16::HL)).into(), "            ".into(),
            ]),
        ];

        Paragraph::new(lines).block(Block::default()
                                    .title(Line::from("Registers").right_aligned())
                                    .borders(Borders::ALL)
                                    .padding(Padding::uniform(1)))
                             .alignment(Alignment::Center)
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut term = ratatui::init();

        while !self.exit {
            term.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        ratatui::restore();
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.exit = true;
            }
            _ => {}
        };

        Ok(())
    }
}

pub fn tui_main<P: AsRef<Path>>(rom_path: P) -> Result<(), String> {
    let mut ui = Ui::new(Emulator::new(rom_path)?);
    ui.run();
    Ok(())
}
