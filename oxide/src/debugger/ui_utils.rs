use ratatui::{
    layout::{Rect},
    text::Line
};

use super::ui_logger::LogEntry;

pub fn get_centered_area(source_area: Rect, width: u16) -> Rect{
    let left = (source_area.width.saturating_sub(width)) / 2;
    Rect {
        x: source_area.x + left,
        y: source_area.y,
        width: width.min(source_area.width),
        height: source_area.height
    }
}

pub fn format_log(log: LogEntry) -> Line<'static> {
    Line::from(log.message)
}
