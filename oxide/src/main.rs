mod emulator;

use crate::emulator::*;

fn main() {
    let _bus = Emulator::new("./ROMs/Tetris.gb".to_string());
}
