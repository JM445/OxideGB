use crate::emulator::memory::regdefines::JOYP;
use crate::emulator::memory::Bus;
use crate::emulator::ppu::{Frame, Ppu};
use crossbeam_channel::Sender;
use log::warn;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

/* Joypad Inputs bits:
 * A      -> 0
 * B      -> 1
 * Select -> 2
 * Start  -> 3
 * Right  -> 4
 * Left   -> 5
 * Up     -> 6
 * Down   -> 7
 */


pub struct IoManager {
    pub tx_frame: Sender<Frame>,
    pub joyp: Arc<AtomicU8>,
}


impl IoManager {
    pub fn new(tx_frame: Sender<Frame>, joyp: Arc<AtomicU8>) -> IoManager {
        IoManager {
            tx_frame,
            joyp
        }
    }
    
    pub fn send_frame(&mut self, frame: Frame) {
        if self.tx_frame.try_send(frame).is_err() {
            warn!("Dropped a frame as UI is not ready")
        }
    }
    
    pub fn get_joystate(&self) -> u8 {
        self.joyp.load(Ordering::Relaxed)
    }

    pub fn tick(&mut self, bus: &mut Bus, frame: Option<Frame>) {
        // Joypad register computation
        let joystate: u8 = self.joyp.load(Ordering::Relaxed); // Get state from sdl thread
        let sel = bus.read(JOYP) & 0x30;                 // Get Register selection bits
        let buttons = (sel & 0b0010_0000) == 0;         // Is buttons selected
        let dpad = (sel & 0b0001_0000) == 0;            // Is DPad selected
        let mut result: u8 = 0;
        
        if buttons {result |=  joystate       & 0xF}                
        if dpad    {result |= (joystate >> 4) & 0xF}
        
        result = ((!result) & 0xF) | sel | 0xC0;             // Recompute the register
        bus.write(JOYP, result);
    }
}