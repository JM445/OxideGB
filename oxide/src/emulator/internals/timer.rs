use log::{debug, info};
use crate::emulator::memory::Bus;
use crate::emulator::cpu::interrupt::*;

const DIV: u16 = 0xFF04;
const TIMA: u16 = 0xFF05;
const TMA: u16 = 0xFF06;
const TAC: u16 = 0xFF07;

#[derive(Copy, Clone, Default)]
pub struct Timer {
    cycles: u16,
    last_and_result: bool
}

impl Timer {
    
    // Should Be ticked every T cycle 
    pub fn tick(&mut self, bus: &mut Bus) {
        self.cycles = self.cycles.wrapping_add(1);
        if bus.div_written {
            self.cycles = 0;
            bus.div_written = false;
        }

        let tima = bus.read(TIMA);
        let tma  = bus.read(TMA);
        let tac  = bus.read(TAC);
        
        let enable = (tac & 0b100) != 0;
        let taken_bit = self.get_taken_bit(tac);
        let and_result = enable && taken_bit;
        
        if !self.last_and_result && and_result { // TIMA Increments
            if tima == 0xFF {
                bus.set_interrupt(Interrupt::Timer);
                bus.write(TIMA, tma);
                info!("Interrupt Requested: Timer");
            } else {
                bus.write(TIMA, tima.wrapping_add(1));
            }
        }
        
        bus.ioregs[0x04] = (self.cycles >> 8) as u8;
    }
    
    fn get_taken_bit(&self, tac: u8) -> bool {
        match tac & 0b11 {
            0 => self.cycles & (1 << 9) != 0,
            1 => self.cycles & (1 << 3) != 0,
            2 => self.cycles & (1 << 5) != 0,
            3 => self.cycles & (1 << 7) != 0,
            _ => panic!("Unreachable")
        }
    }
}