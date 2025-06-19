pub mod emulator;
pub mod debugger;
mod settings;

use crate::debugger::tui::ui_logger::UiLogger;
use crate::debugger::{*};
use crate::emulator::*;

use clap::{Parser, ValueEnum};
use debugger::tui::tui_main;
use debugger::DummyDebugger;
use std::fmt;
use std::sync::Arc;

use self::settings::*;

#[macro_export]
macro_rules! emu_print {
    ($($arg:tt)*) => {{
        use std::io::Write;
        if crate::settings::GLOB_SETTINGS.get().unwrap().tui_enabled {
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("emu_output.log")
                .unwrap();
            write!(file, $($arg)*).unwrap();
        } else {
            print!($($arg)*);
        }
    }};
}

#[derive(Parser)]
#[command(version, about, name = "OxideGB")]
struct Cli {
    /// Which debugger to use
    #[arg(short, long, default_value_t = DebugMode::None)]
    debug: DebugMode,

    /// Boot rom binary file
    #[arg(short, long, default_value_t = String::new())]
    boot: String,
    
    /// If enabled, then the content of serial data register is printed when modified
    #[arg(short = 'p', long)]
    serial_print: bool,
    
    /// If enabled, then a log is printed each tick with GameBoy Doctor format
    #[arg(long = "doctor")]
    doctor_log: bool,

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

fn set_settings(cli: &Cli) {
    let tui_enabled = match cli.debug {
        DebugMode::Full => true,
        _ => false,
    };
    
    GLOB_SETTINGS.set(Arc::new(Settings {
        print_serial: cli.serial_print,
        tui_enabled,
        doctor_logs: cli.doctor_log
    })).expect("Settings already initialized !");
}

fn main() {
    let cli = Cli::parse();
    set_settings(&cli);
    match cli.debug {
        DebugMode::Full => {
            UiLogger::init();
            if let Err(e) = tui_main(cli.rom_path, cli.boot) {
                println!("Error while starting emulator: {e}");
            }
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
