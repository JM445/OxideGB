mod emulator;

use crate::emulator::bus::Bus;

fn main() {
    let _bus = Bus::new("./ROMs/Tetris.gb".to_string());
}
