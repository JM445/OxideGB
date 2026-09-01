# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Working with the user

- **Ask before editing or creating files.** The user writes the implementation code themselves and wants Claude for structural/theoretical advice, code review, and explanations. The exception is boring or repetitive work (e.g. mechanical refactors, generating boilerplate/tables, updating many call sites the same way) — for that kind of task it's fine to just do it, unless the user says otherwise.
- Default to discussing approach and tradeoffs rather than producing a diff.

## Project overview

OxideGB is a Game Boy (DMG) emulator written in Rust, currently under active/WIP development (CPU and memory bus are functional; the PPU's pixel-fetcher/rendering pipeline is mid-implementation — see recent commits). It has three components:

- `oxide/` — the Rust workspace member containing the actual emulator, debugger, and SDL3-based GUI.
- `opcode_writer/` — a standalone Python script (not part of the Cargo build) that fetches an opcode table (`game-boy-opcodes` JSON) from the web and is used as a one-off code-generation helper for the CPU decoder tables.
- `doctor/` and `ROMs/Blargg/` — git submodules: [gameboy-doctor](https://github.com/robert/gameboy-doctor) (a reference-log diffing tool for debugging CPU correctness) and Blargg's test ROMs, respectively. Run `git submodule update --init --recursive` if they're empty.

## Development environment

The project uses a Nix flake (`flake.nix`) + `direnv` (`.envrc`) to provide the Rust toolchain (pinned via `toolchain.toml`, stable channel with `rustfmt`/`clippy`/`rust-src`) plus native deps: SDL3, `rgbds` (GB dev assembler toolchain), and Python 3.12. If not using direnv/nix, SDL3 must be installed and discoverable for linking (the `sdl3` crate links against it).

All Cargo commands below are run from `oxide/`.

## Commands

```bash
cd oxide

cargo build                 # debug build
cargo build --release       # release build
cargo run -- <rom_path>     # run a ROM
cargo test                  # run unit tests (CPU registers/micro-ops, see below)
cargo clippy
cargo fmt
```

CLI (`oxide/src/main.rs`):

```bash
cargo run -- [-d/--debug none|log|full] [-b/--boot <boot_rom_path>] [-p/--serial-print] [--doctor] <rom_path>
```

- `--debug full` launches an interactive ratatui TUI debugger instead of the SDL window's console output; `--debug log` uses `env_logger` (`RUST_LOG=debug` etc.) to print `Debugger` trait events.
- `--doctor` prints one gameboy-doctor-formatted CPU state line per instruction (to stdout, or to `emu_output.log` when the TUI is active — see `emu_print!` in `main.rs`). Feed the resulting log to `doctor/gameboy-doctor` against a Blargg `cpu_instrs` ROM to bisect CPU bugs: `./doctor/gameboy-doctor <logfile> cpu_instrs <N>`.

### Tests

Unit tests currently only cover `oxide/src/emulator/cpu/` (registers and micro-ops). They are **not** inline `#[cfg(test)] mod tests { ... }` blocks — each source file pulls its tests in from a sibling file via `#[path = "tests/<name>.rs"]`, e.g. `registers.rs` → `tests/registers.rs`. When adding tests for a CPU submodule, follow that same `#[path]`-include pattern rather than inlining.

## Architecture

The emulator is driven by a single tick loop stepping four subsystems in lockstep, run on a background thread while SDL owns the main thread for rendering:

- **`main.rs`**: parses CLI args into a process-wide `Settings` singleton (`settings::GLOB_SETTINGS`, a `OnceCell`), then spawns a worker thread that owns the `Emulator` and loops calling `tick()` (selecting `DummyDebugger`/`LogDebugger`/the TUI debugger based on `--debug`). The main thread runs `gui::start_gui`, the SDL3 window/event loop. The two threads communicate over a bounded `crossbeam_channel` of completed `Frame`s (emulator → GUI) and a shared `Arc<AtomicU8>` joypad state (GUI → emulator).

- **`emulator/mod.rs`**: `Emulator` owns `Cpu`, `Bus` (memory), `Ppu`, and `Timer`. `Emulator::tick()` is called once per T-cycle: the CPU only actually steps every 4th call (M-cycle), while the bus's serial port, the PPU, and the timer tick every T-cycle.

- **`emulator/cpu/`**: a micro-op based CPU core, not a giant per-opcode `match` executing everything at once. `decoder.rs` maps each opcode to a `VecDeque<MicroOp>` (built via helpers split across `inline_ld_decoder.rs`, `inline_alu_decoder.rs`, `inline_jump_decoder.rs`, `inline_misc_decoder.rs`, `inline_binop_decoder.rs` by instruction category); `Cpu::tick` pops and executes one `MicroOp` per M-cycle, decoding a fresh instruction via `execute_prefetch` when the queue empties. `registers.rs` defines the `Reg8`/`Reg16` abstractions, `interrupt.rs` handles `IME`/interrupt dispatch, and `Cpu::get_doctor_log` produces the gameboy-doctor log line format.

- **`emulator/memory/`**: `Bus` is the central memory-mapped I/O dispatcher — `read`/`write` match on address ranges to route to boot ROM, cartridge (via the `cartridge::Mbc` trait, currently `NoMbc`/`Mbc1`, dispatched through the `AnyCartridge` enum), work RAM, OAM/VRAM (which lock out CPU access depending on current `Ppu` `Mode`, per real hardware), I/O registers, and HRAM. `MemBlock::from_addr` categorizes addresses into these regions; `regdefines.rs` has the I/O register address constants (`LY`, `LCDC`, `JOYP`, etc.).

- **`emulator/ppu/`**: a dot-based state machine (`Ppu::tick`, called every T-cycle) cycling through `Mode::{Mode0,Mode1,Mode2,Mode3}` (HBlank/VBlank/OAM-scan/pixel-transfer) using scanline/frame dot counters, mirroring real PPU timing (456 dots/line, 70224 dots/frame). `fetcher.rs` implements the pixel fetcher (background/window/sprite pixel FIFO — this is the actively-in-progress piece per recent commits), `pixels.rs` defines `GBColor`/`Sprite`. Completed frames are handed to the `Bus`/`IoManager` to send over the frame channel to the GUI.

- **`emulator/internals/`**: `IoManager` bridges the emulator thread to the outside world — sending completed frames over the crossbeam channel (dropping a frame with a warning if the GUI isn't keeping up) and computing the `JOYP` register from the shared joypad `AtomicU8`. `Timer` implements the DIV/TIMA timer registers.

- **`debugger/`**: a `Debugger` trait (`on_cpu_event`/`on_ppu_event`) is threaded through `Cpu::tick`/`Ppu::tick` as a generic parameter, so instrumentation has no cost when using `DummyDebugger` (a no-op). `LogDebugger` routes `DebugEvent`s through the `log` crate; `FullDebugger`/`tui/` implement an interactive ratatui+crossterm TUI (its own memory view, disassembly view, and a scriptable command parser in `tui/parser.rs`) driven from `debugger::tui::tui_main`. `dissassembler/` turns raw opcodes into human-readable mnemonics for the TUI/log output.

- **`gui/`**: SDL3 window and render loop (`start_gui`). Renders the emulator's `Frame` (a `Box<[GBColor]>` framebuffer) into a streaming texture, mapping each `GBColor` index through the user-configurable `Settings.colors` palette, composited over a static background PNG (`assets/dmg_background.png`) baked in with `include_bytes!`.

- **`settings/`**: `Settings` is a small struct of run flags (`print_serial`, `tui_enabled`, `doctor_logs`, `colors`) set once at startup into the global `GLOB_SETTINGS: OnceCell<Arc<Settings>>` and read from anywhere via `GLOB_SETTINGS.get().unwrap()`.
