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

        let top_down = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(80),
                Constraint::Min(4)
            ]).split(area);

        let down_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(80),
                Constraint::Length(40)
            ]).split(top_down[1]);

        let top_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(80),
                Constraint::Min(0)
            ]).split(top_down[0]);

        frame.render_widget(
            Block::default()
                .title(Line::from("Disassembly"))
                .left_aligned()
                .borders(Borders::ALL), top_split[0]);
        frame.render_widget(
            Block::default()
                .title(Line::from("Memory").right_aligned())
                .borders(Borders::ALL), top_split[1])
        frame.render_widget(
            Block::default()
                .title(Line::from("CMD").right_aligned())
                .borders(Borders::ALL), down_split[0])
        frame.render_widget(
            Block::default()
                .title(Line::from("Registers").right_aligned())
                .borders(Borders::ALL), down_split[1])
    }

    pub fn run(&self) {
        let mut term = ratatui::init();

        while !self.exit {
            term.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        ratatui::restore();
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

pub fn test_render() {

}

pub fn tui_main<P: AsRef<Path>>(rom_path: P) -> Result<(), String> {
    let mut ui = Ui::new(Emulator::new(rom_path));
    Ok(())
}
