pub mod emulator;
pub mod debugger;

use crate::emulator::*;

fn main() {
    let mut emu = Emulator::new("../ROMs/Tests/test.gb".to_string()).unwrap();

    env_logger::init();

    emu.tick();
    emu.tick();
    emu.tick();
    emu.tick();
    emu.tick();
    emu.tick();
}
