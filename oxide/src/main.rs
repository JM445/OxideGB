pub mod emulator;
pub mod debugger;

use crate::emulator::*;

fn main() {
    let mut emu = Emulator::new("../ROMs/Tests/test.gb".to_string()).unwrap();

    emu.tick();
    emu.tick();
    emu.tick();
    emu.tick();
    emu.tick();
    emu.tick();
}
