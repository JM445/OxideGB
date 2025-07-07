pub mod full_debugger;
pub mod displays;
pub mod tui;
pub mod dissassembler;

use full_debugger::*;

use crate::emulator::cpu::micro_ops::*;
use crate::emulator::cpu::registers::*;
use crate::emulator::cpu::*;
use crate::emulator::memory::*;
use crate::emulator::ppu::*;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

pub trait Debugger {
    fn on_cpu_event(&mut self, event: DebugEvent, cpu: &Cpu, bus: &Bus);
    fn on_ppu_event(&mut self, event: DebugEvent, ppu: &Ppu, bus: &Bus);
}

#[derive(Debug)]
pub enum DebuggerKind {
    Dummy(DummyDebugger),
    Full(FullDebugger),
    Log(LogDebugger)
}

#[derive(Debug, Copy, Clone)]
pub enum DebugEvent {
    MicroOpEnd(MicroOp),
    InstructionEnd(u8),
    IrPrefetch(u8, u16),
    Register8Change(Reg8, u8),
    Register16Change(Reg16, u16),
}


#[derive(Debug, Default)]
pub struct LogDebugger {}

impl Debugger for LogDebugger {
    fn on_cpu_event(&mut self, event: DebugEvent, _cpu: &Cpu, _bus: &Bus) {
        debug!("LogDebugger: CPU Event received: {event}");
    }

    fn on_ppu_event(&mut self, event: DebugEvent, _ppu: &Ppu, _bus: &Bus) {
        debug!("LogDebugger: PPU Event received: {event}");
    }
}

#[derive(Debug, Default)]
pub struct DummyDebugger {}

impl Debugger for DummyDebugger {
    fn on_cpu_event(&mut self, _event: DebugEvent, _cpu: &Cpu, _bus: &Bus) {
        ();
    }

    fn on_ppu_event(&mut self, _event: DebugEvent, _ppu: &Ppu, _bus: &Bus) {
        ();
    }
}
