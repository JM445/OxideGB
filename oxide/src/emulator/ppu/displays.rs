use super::*;

use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Mode::Mode0 => "Mode 0".to_string(),
            Mode::Mode1 => "Mode 1".to_string(),
            Mode::Mode2 => "Mode 2".to_string(),
            Mode::Mode3 => "Mode 3".to_string()
        };

        write!(f, "{}", s)
    }
}

impl fmt::Display for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("Sprite {{id: {}, x:  {}, y: {}, flags: {:#010b}}}", self.tile_id, self.x, self.y, self.flags);
        write!(f, "{}", s)
    }
}

impl fmt::Display for GBColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            GBColor::OFF => "OFF",
            GBColor::WHITE => "White",
            GBColor::LGREY => "Light Grey",
            GBColor::DGREY => "Dark Grey",
            GBColor::BLACK => "Black",
        };

        write!(f, "{}", s.to_string())
    }
}