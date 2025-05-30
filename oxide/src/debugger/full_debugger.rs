use crate::emulator::memory::*;
use crate::emulator::cpu::*;
use crate::emulator::ppu::*;
use crate::emulator::cpu::registers::*;

use super::*;

#[allow(unused_imports)]
use log::{debug, info, warn};

#[derive(Debug, Default)]
pub struct FullDebugger {
    breakpoints: Vec<Breakpoint>,
}

#[derive(Debug, Clone)]
pub enum Breakpoint {
    Ticks(usize),
    Address(u16),
    Instructions(usize),
    Register8Value(Reg8, u8),
    Register16Value(Reg16, u16)
}

impl Debugger for FullDebugger {
    fn on_cpu_event(&mut self, event: DebugEvent, _cpu: &Cpu, _bus: &Bus) {
        debug!("FullDebugger: CPU Event received: {event:?}");
        match event {
            DebugEvent::InstructionEnd(_) => {
                for bp in &mut self.breakpoints {
                    match bp {
                        Breakpoint::Instructions(n) => *n -= 1,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn on_ppu_event(&mut self, event: DebugEvent, _ppu: &Ppu, _bus: &Bus) {
        debug!("FullDebugger: PPU Event received: {event:?}");
    }
}

impl FullDebugger {
    pub fn new() -> Self {
        FullDebugger {
            breakpoints: Vec::new()
        }
    }

    pub fn add_breakpoint(&mut self, brk: Breakpoint) -> usize {
        self.breakpoints.push(brk);
        self.breakpoints.len() - 1
    }

    pub fn should_stop(&mut self, cpu: &Cpu, bus: &Bus) -> bool {
        let mut triggered = false;

        for b in &self.breakpoints {
            let res = match b {
                Breakpoint::Ticks(n) | Breakpoint::Instructions(n) => if n <= &0 {true} else {false},
                Breakpoint::Address(a) => if &cpu.pc == a {true} else {false},
                Breakpoint::Register8Value(r, v) => if cpu.read8(*r) == *v {true} else {false},
                Breakpoint::Register16Value(r, v) => if cpu.read16(*r) == *v {true} else {false},
            };

            if res {
                triggered = true;
                break;
            }
        }

        self.breakpoints.retain(|bp| {
            match bp {
                Breakpoint::Instructions(n) | Breakpoint::Ticks(n) if *n <= 0 => false,
                _ => true
            }
        });
        triggered
    }

    pub fn tick(&mut self) {
        for bp in &mut self.breakpoints {
            match bp {
                Breakpoint::Ticks(n) => *n -= 1,
                _ => {}
            }
        }
    }
}
