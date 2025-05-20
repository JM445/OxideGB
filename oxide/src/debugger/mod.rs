mod full_debugger;

use full_debugger::*;

use crate::emulator::memory::*;
use crate::emulator::cpu::*;
use crate::emulator::cpu::micro_ops::*;
use crate::emulator::ppu::*;

#[allow(unused_imports)]
use log::{debug, info, warn};

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
    IrPrefetch(u8),
}


impl DebuggerKind {
    pub fn on_cpu_event(&mut self, event: DebugEvent, cpu: &Cpu, bus: &Bus) {
        match self {
            DebuggerKind::Dummy(d) => d.on_cpu_event(event, cpu, bus),
            DebuggerKind::Log(d)   => d.on_cpu_event(event, cpu, bus),
            DebuggerKind::Full(d)  => d.on_cpu_event(event, cpu, bus),
        }
    }

    pub fn on_ppu_event(&mut self, event: DebugEvent, ppu: &Ppu, bus: &Bus) {
        match self {
            DebuggerKind::Dummy(d) => d.on_ppu_event(event, ppu, bus),
            DebuggerKind::Log(d)   => d.on_ppu_event(event, ppu, bus),
            DebuggerKind::Full(d)  => d.on_ppu_event(event, ppu, bus),
        }
    }
}

#[derive(Debug, Default)]
pub struct LogDebugger {}

impl Debugger for LogDebugger {
    fn on_cpu_event(&mut self, event: DebugEvent, _cpu: &Cpu, _bus: &Bus) {
        debug!("DummyDebugger: CPU Event received: {event:?}");
    }

    fn on_ppu_event(&mut self, event: DebugEvent, _ppu: &Ppu, _bus: &Bus) {
        debug!("DummyDebugger: PPU Event received: {event:?}");
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
