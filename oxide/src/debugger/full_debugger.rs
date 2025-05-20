use crate::emulator::memory::*;
use crate::emulator::cpu::*;
use crate::emulator::ppu::*;

use super::*;

#[allow(unused_imports)]
use log::{debug, info, warn};

#[derive(Debug, Default)]
pub struct FullDebugger {

}

impl Debugger for FullDebugger {
    fn on_cpu_event(&mut self, event: DebugEvent, _cpu: &Cpu, _bus: &Bus) {
        debug!("FullDebugger: CPU Event received: {event:?}");
    }

    fn on_ppu_event(&mut self, event: DebugEvent, _ppu: &Ppu, _bus: &Bus) {
        debug!("FullDebugger: PPU Event received: {event:?}");
    }
}
