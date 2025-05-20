use super::memory::*;

use crate::debugger::DebuggerKind;

#[derive(Debug, Default)]
pub struct Ppu {}

impl Ppu {
    pub fn tick(&mut self, bus: &mut Bus, dbg: &mut DebuggerKind) {
        ()
    }
}
