pub mod emulator;
pub mod debugger;
mod settings;
mod gui;

use crate::debugger::tui::ui_logger::UiLogger;
use crate::debugger::*;
use crate::emulator::*;

use self::settings::GLOB_SETTINGS;
use self::settings::Settings;
use crate::emulator::internals::iomanager::IoManager;
use crate::emulator::ppu::Frame;
use clap::{Parser, ValueEnum};
use crossbeam_channel::{bounded, Sender, Receiver};
use debugger::tui::tui_main;
use debugger::DummyDebugger;
use std::fmt;
use std::sync::atomic::{AtomicU32, AtomicU8};
use std::sync::Arc;
use log::error;
use crate::gui::start_gui;

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
            std::io::stdout().flush().unwrap()
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
    let mut settings= Settings::default();
    settings.doctor_logs = cli.doctor_log;
    settings.print_serial = cli.serial_print;
    settings.tui_enabled = tui_enabled;
    GLOB_SETTINGS.set(Arc::new(settings)).expect("Settings already initialized !");
}

fn launch_worker(cli: Cli, tx_frame: Sender<Frame>, joystate: Arc<AtomicU8>, fps: Arc<AtomicU32>) -> std::thread::JoinHandle<()> {
    let io_manager = IoManager::new(tx_frame, joystate, fps);

    std::thread::spawn(move || {
        let emu_res = Emulator::new(cli.rom_path, cli.boot, io_manager);
        if let Err(e) = emu_res {
            println!("Error while creating the emulator: {e}");
            return;
        }
        
        let mut emu= emu_res.unwrap();
        match cli.debug {
            DebugMode::Full => {
                UiLogger::init();
                if let Err(e) = tui_main(emu) {
                    println!("Error while starting emulator: {e}");
                }
                return;
            }
            DebugMode::None => {
                let mut dbg = DummyDebugger::default();
                loop {
                    emu.tick(&mut dbg);
                }
            }
            DebugMode::Log => {
                println!("Starting emulator in log mode");
                env_logger::init();
                let mut dbg = LogDebugger::default();
                loop {
                    emu.tick(&mut dbg);
                }
            }
        }
    })
}

fn main() {
    let cli = Cli::parse();
    set_settings(&cli);
    
    let (tx_frame, rx_frame) : (Sender<Frame>, Receiver<Frame>) = bounded(2);
    let joystate = Arc::new(AtomicU8::new(0));
    let fps = Arc::new(AtomicU32::new(0));
    
    let _ = launch_worker(cli, tx_frame, joystate.clone(), fps.clone());

    if let Err(e) = start_gui(rx_frame, joystate, fps) {
        error!("SDL Error: {e}");
    }
}
