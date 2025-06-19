use super::dissassembler::opcodes::*;
use super::ui_logger::*;
use super::ui_utils::*;
use super::Ui;
use crate::emulator::cpu::registers::*;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};
use std::sync::Arc;

impl<'a> Ui<'a> {
    pub(super) fn draw(&mut self, frame: &mut Frame) {
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
                Constraint::Min(58),
                Constraint::Percentage(100)
            ]).split(top_down[0]);

        frame.render_widget(self.draw_disassembly(top_split[0].height, top_split[0].width), top_split[0]);
        frame.render_widget(self.draw_memory(top_split[1].height), top_split[1]);
        frame.render_widget(self.draw_logger(top_split[2].width, top_split[2].height), top_split[2]);
        frame.render_widget(&self.cmd_area, down_split[0]);
        frame.render_widget(self.draw_registers(), down_split[1]);
    }

    fn draw_logger(&self, width: u16, height: u16) -> Paragraph {
        if height < 2 || width < 1 {
            return Paragraph::new("")
        }
        let logger = Arc::clone(&GLOB_LOGGER);

        let lines = (*logger).entries.lock().unwrap().iter().map(|e| format_log(e.clone())).collect::<Vec<Line>>();

        let total_height : u16 = lines.iter().map(|l| {
            ((l.width() as u16 + width.saturating_sub(1)) / width).max(1)
        }).sum();
        let scroll = total_height.saturating_sub(height - 2);

        Paragraph::new(lines).block(Block::default()
                                    .title(Line::from("Log").right_aligned())
                                    .borders(Borders::ALL))
                             .wrap(Wrap {trim: false})
                             .alignment(Alignment::Left)
                            .scroll((scroll, 0))
    }

    fn draw_disassembly(&mut self, height: u16, width: u16) -> Paragraph {
        let mut lines = Vec::new();
        let addr = self.emulator.cpu.ir_pc;
        let (block, _) = self.code_map.get_block(&self.emulator.bus, &self.emulator.cpu);

        // Map the previously executed instructions to a list of lines
        let mut padded_prev = self.debugger.last_instructions.iter()
                .map(|e| Self::get_disassemble_line(&e.1, e.0, false, true))
            .collect::<Vec<Line>>();

        while padded_prev.len() < 5 {
            padded_prev.insert(0, Self::get_disassemble_line(&[0, 0, 0, 0], 0, false, true));
        }

        lines.extend(padded_prev);

        // Add a separator
        lines.push(std::iter::repeat('-').take((width as usize).saturating_sub(2)).collect::<String>().into());

        let index = block.instructions.iter()
            .take_while(
                |i| {
                    let fixed_pc = addr;
                    !(fixed_pc >= i.addr && fixed_pc < i.addr + i.size as u16)
                }
            ).count();
        // Map the right amount of elements of the current block to a list of lines
        lines.extend(
                block.instructions.iter().skip(index).take((height as usize).saturating_sub(6)).map(
                    |i| {
                        let current = addr >= i.addr && addr < i.addr + i.size as u16;
                        Self::get_disassemble_line(&i.full_bytes, i.addr, current, false)
                    } 
                )
            );

        Paragraph::new(lines).block(Block::default()
            .title(Line::from("Disassembly").left_aligned())
            .borders(Borders::ALL)
            .padding(Padding::uniform(1)))
            .alignment(Alignment::Left)
    }

    fn get_disassemble_line(instr: &[u8; 4], addr: u16,  current: bool, previous: bool) -> Line {
        let style = match (current, previous) {
            (_, true) => Style::new().fg(Color::Black).bg(Color::Rgb(74, 74, 74)),
            (true, _) => Style::new().reversed(),
            (false, false) => Style::new()
        };

        let x = vec![
            format!("{:#06X} | ", addr).blue().bold().into(),
            Span::styled(format!("{:<width$}", disassemble(instr), width = 20), style),
            "| ".into(),
            instr.iter().take(get_instruction_length(instr[0]) as usize)
                .map(|x| format!("{:#04X}", x)).collect::<Vec<_>>().join(" ").into(),
        ];

        Line::from(x)
    }

    fn draw_registers(&self) -> Paragraph {
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
                "DE: ".blue().bold().into(), format!("{:#06X}", cpu.read16(Reg16::DE)).into(), "  ".into(),
                "WZ: ".blue().bold().into(), format!("{:#06X}", cpu.read16(Reg16::WZ)).into(),
            ]),
            Line::from(vec![
                "H: ".blue().bold().into(),  format!("{:#04X}", cpu.h).into(), "  ".into(),
                "L: ".blue().bold().into(),  format!("{:#04X}", cpu.l).into(), "  ".into(),
                "HL: ".blue().bold().into(), format!("{:#06X}", cpu.read16(Reg16::HL)).into(), "            ".into(),
            ]),
            Line::from(vec![
                "Z: ".blue().bold().into(), format!("{} ", cpu.get_flag(Flag::Z)).into(),
                "N: ".blue().bold().into(), format!("{} ", cpu.get_flag(Flag::N)).into(),
                "H: ".blue().bold().into(), format!("{} ", cpu.get_flag(Flag::H)).into(),
                "C: ".blue().bold().into(), format!("{} ", cpu.get_flag(Flag::C)).into(),
            ])
        ];

        Paragraph::new(lines).block(Block::default()
                                    .title(Line::from("Registers").right_aligned())
                                    .borders(Borders::ALL)
                                    .padding(Padding::new(1, 1, 1, 0)))
                             .alignment(Alignment::Center)
    }
}
