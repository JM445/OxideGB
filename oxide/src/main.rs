pub mod emulator;
pub mod debugger;

use crate::emulator::*;
use crate::debugger::{*, full_debugger::*};
use crate::debugger::tui::ui_logger::UiLogger;

use std::fmt;
use clap::{Parser,ValueEnum};
use debugger::tui::tui_main;
use debugger::DummyDebugger;

#[derive(Parser)]
#[command(version, about, name = "OxideGB")]
struct Cli {
    /// Which debugger to use.
    #[arg(short, long, default_value_t = DebugMode::None)]
    debug: DebugMode,

    /// Boot rom binary file
    #[arg(short, long, default_value_t = String::new())]
    boot: String,

    /// Path of the GB ROM to load
    rom_path: String,
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
    match cli.debug {
        DebugMode::Full => {
            UiLogger::init();
            let _ = tui_main(cli.rom_path, cli.boot);
            return;
        }
        DebugMode::None => {
            let mut dbg = DummyDebugger::default();
            let mut emu = Emulator::new(cli.rom_path, cli.boot).unwrap();
            loop {
                emu.tick(&mut dbg);
            }
        }
        DebugMode::Log => {
            println!("Starting emulator in log mode");
            env_logger::init();
            let mut dbg = LogDebugger::default();
            let mut emu = Emulator::new(cli.rom_path, cli.boot).unwrap();
            loop {
                emu.tick(&mut dbg);
            }
        }
    }
}
