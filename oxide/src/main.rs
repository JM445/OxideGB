pub mod emulator;
pub mod debugger;

use crate::emulator::*;
use crate::debugger::{*, full_debugger::*};

use std::fmt;
use clap::{Parser,ValueEnum};
use debugger::DummyDebugger;

#[derive(Parser)]
#[command(version, about, name = "OxideGB")]
struct Cli {
    #[arg(short, long, default_value_t = DebugMode::None)]
    debug: DebugMode
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DebugMode {
    /// No debugger
    None,

    /// Full debugging CLI
    Full,

    /// Log events only
    Log
}

impl fmt::Display for DebugMode {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            DebugMode::None => "none",
            DebugMode::Log  => "log",
            DebugMode::Full => "full",
        };

        write!(f,"{}", s)
    }
}

fn main() {
    let cli = Cli::parse();
    let mut emu = Emulator::new("../ROMs/Tests/test.gb".to_string()).unwrap();

    env_logger::init();

    let mut dbg = match cli.debug {
        DebugMode::None => DebuggerKind::Dummy(DummyDebugger::default()),
        DebugMode::Log => DebuggerKind::Log(LogDebugger::default()),
        DebugMode::Full => DebuggerKind::Full(FullDebugger::default()),
    };

    emu.tick(&mut dbg);
    emu.tick(&mut dbg);
    emu.tick(&mut dbg);
    emu.tick(&mut dbg);
    emu.tick(&mut dbg);
    emu.tick(&mut dbg);
}
