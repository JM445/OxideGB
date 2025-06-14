use super::*;
use log::error;
use ratatui::style::{Color, Stylize};
use ratatui::text::Span;
use ratatui::widgets::{Padding, Paragraph};

use super::ui_utils::*;
impl <'a> Ui<'a> {
    pub(super) fn parse_mem(&mut self, words: &[&str]) {
        if words.len() < 1 {
            error!("Error: Invalid memory command argument count !");
            return;
        }

        match words[0] {
            "clear" => self.memory_watches.clear(),
            "add" => self.parse_mem_add(&words[1..]),
            "del" => self.parse_mem_del(&words[1..]),
            _ => ()
        }
    }
    
    fn parse_mem_add(&mut self, words: &[&str]) {
        if words.len() != 2 {
            error!("Error: Invalid memory add. Usage: mem add addr size");
            return;
        }
        
        let res_addr: Result<u16, _> = parse_hex_or_dec(words[0]);
        let res_size: Result<usize, _> = parse_hex_or_dec(words[1]);

        if let (Ok(addr), Ok(size)) = (res_addr, res_size) {
            self.memory_watches.push_back((addr, size))
        } else {
            error!("Error: Invalid memory add argument.")
        }
    }

    fn parse_mem_del(&mut self, words: &[&str]) {
        if words.len() != 1 {
            error!("Error: Invalid memory del. Usage: mem del addr");
            return;
        }

        if let Ok(addr) = words[0].parse::<u16>() {
            self.memory_watches.retain(|(e, _)| *e != addr);
        } else {
            error!("Error: Invalid memory del argument.");
        }
    }

    // A line is 54 chars
    pub (super) fn draw_memory(&self, height: u16) -> Paragraph{
        let mut res : Vec<Line> = Vec::new();

        for (addr, size) in &self.memory_watches {
            let real_start = addr & !0xF;
            let real_size = ((*addr as usize + size + 15) & !15) - real_start as usize;
            let nb_lines = real_size / 16;
            let mut header = format!("----- Memory: {:#06X}", addr);

            header.extend(std::iter::repeat('-').take(54usize.saturating_sub(header.len())));
            res.push(header.into());

            for j in 0..nb_lines {
                let mut line = vec![
                    Span::from(format!("{:04X} | ", real_start as usize + (j * 16)))
                        .style(Style::new().fg(Color::Blue).bold())
                ];

                for x in 0..16 {
                    let pos = real_start as usize + (j * 16) + x;
                    let style = if pos >= *addr as usize && pos < *addr as usize + size {
                        Style::new().reversed()
                    } else {
                        Style::new()
                    };
                    line.push(Span::from(
                        format!("{:02X} ", self.emulator.bus.read(pos as u16)))
                        .style(style));
                }

                res.push(Line::from(line));
            }
        }

        Paragraph::new(res).block(Block::default()
            .title(Line::from("Memory").centered())
            .borders(Borders::ALL)
            .padding(Padding::uniform(1)))
    }
}

