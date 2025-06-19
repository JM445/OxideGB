use log::error;
use num_traits::{Num, PrimInt};
use ratatui::{
    layout::Rect,
    text::Line
};
use std::str::FromStr;

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

pub fn parse_hex_or_dec<T>(num: &str) -> Result<T, <T as Num>::FromStrRadixErr>
where T: Num + Copy,
{
    if let Some(hex) = num.strip_prefix("0x") {
        T::from_str_radix(hex, 16)
    } else {
        T::from_str_radix(num, 10)
    }
}

pub fn parse_numeric<T>(words: &[&str], pos: usize) -> Option<T>
where T: PrimInt + FromStr {
    if words.len() > pos {
        if let Ok(n) = words[1].parse() {
            Some(n)
        } else {
            error!("Error: Invalid tick number: {}", words[1]);
            return None;
        }
    } else {
        Some(T::one())
    }
}
