use crate::debugger::DebugEvent::DmaTransferStarted;
use crate::debugger::Debugger;
use crate::emulator::memory::Bus;
use crate::emulator::memory::regdefines::*;

#[derive(Debug, Default)]
pub struct OamDma {
    cycle: u16,
    source: u16,
}

impl OamDma {
    pub fn tick<T>(&mut self, bus: &mut Bus, dbg: &mut T)
    where T: Debugger {
        if bus.dma_pending {
            bus.dma_pending = false;
            self.source = (bus.read(DMA) as u16) << 8;
            bus.dma_ongoing = true;
            self.cycle = 0;
            dbg.on_dma_event(DmaTransferStarted(self.source), &self, bus);
        }

        if bus.dma_ongoing {
            bus.ram.write(0xFE00 + self.cycle, bus.dma_read(self.source + self.cycle));
            self.cycle += 1;
            if self.cycle >= 160 {
                bus.dma_ongoing = false;
            }
        }
    }
}