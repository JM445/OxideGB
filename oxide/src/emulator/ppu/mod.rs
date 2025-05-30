use super::memory::*;

use crate::debugger::Debugger;

#[derive(Debug, Default)]
pub struct Ppu {}

impl Ppu {
    pub fn tick<T>(&mut self, bus: &mut Bus, dbg: &mut T)
    where T: Debugger {
        ()
    }
}
