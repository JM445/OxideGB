pub mod full_debugger;
pub mod displays;
pub mod tui;

use full_debugger::*;

use crate::emulator::memory::*;
use crate::emulator::cpu::*;
use crate::emulator::cpu::micro_ops::*;
use crate::emulator::cpu::registers::*;
use crate::emulator::ppu::*;

use super::*;

#[allow(unused_imports)]
use log::{debug, info, warn, error};

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
    Register8Change(Reg8, u8),
    Register16Change(Reg16, u16),
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
        info!("LogDebugger: CPU Event received: {event}");
    }

    fn on_ppu_event(&mut self, event: DebugEvent, _ppu: &Ppu, _bus: &Bus) {
        info!("LogDebugger: PPU Event received: {event}");
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
