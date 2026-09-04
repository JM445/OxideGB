use crate::emulator::ppu::Frame;
use crossbeam_channel::Sender;
use log::warn;
use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::Instant;
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
    pub fps: Arc<AtomicU32>,
    pub frame_count: u32,
    pub last_sample: Instant
}


impl IoManager {
    pub fn new(tx_frame: Sender<Frame>, joyp: Arc<AtomicU8>, fps: Arc<AtomicU32>) -> IoManager {
        IoManager {
            tx_frame,
            joyp,
            fps,
            frame_count: 0,
            last_sample: Instant::now(),
        }
    }
    
    pub fn send_frame(&mut self, frame: Frame) {
        if self.tx_frame.try_send(frame).is_err() {
            warn!("Dropped a frame as UI is not ready")
        }
        self.frame_count += 1;
        // FPS Counting
        if Instant::now().duration_since(self.last_sample).as_secs() >= 1 {
            self.last_sample = Instant::now();
            self.fps.store(self.frame_count, Ordering::Relaxed);
            self.frame_count = 0;
        }
    }
    
    pub fn get_joystate(&self) -> u8 {
        self.joyp.load(Ordering::Relaxed)
    }
}