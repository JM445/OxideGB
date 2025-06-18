#[cfg(test)]
mod tests {
    use crate::emulator::Emulator;
    use crate::debugger::DummyDebugger; // Assuming this path is correct

    use std::env;
    use std::path::PathBuf;

    #[test]
    fn test_cpu_instructions_rom() {
        // 1. Instantiate Emulator
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut rom_path_buf = PathBuf::from(manifest_dir);
        rom_path_buf.push("../ROMs/Tests/cpu_tests.gb");
        let rom_path_str = rom_path_buf.to_str().unwrap();

        let boot_rom_path = ""; // No boot ROM

        let mut emulator = Emulator::new(rom_path_str, boot_rom_path).unwrap_or_else(|e| {
            panic!("Failed to create emulator for test_cpu_instructions_rom: {}. ROM path was: {}", e, rom_path_str);
        });

        // 2. Run Emulator for a set number of CPU ticks/cycles.
        // The instructions in cpu_tests.asm take 72 Z80 cycles.
        // The emulator's `tick()` processes micro-ops. A single Z80 instruction
        // is broken down into multiple micro-ops. 300-1000 ticks should be sufficient.
        // The previous run with 1000 ticks worked.
        let mut dbg = DummyDebugger::default();
        for _ in 0..1000 {
            emulator.tick(&mut dbg);
        }

        // 3. Assert CPU register states
        assert_eq!(emulator.cpu.a, 0x34, "Register A after test ROM. Expected: 0x34, Actual: {:#04X}", emulator.cpu.a);
        assert_eq!(emulator.cpu.b, 0x34, "Register B after test ROM. Expected: 0x34, Actual: {:#04X}", emulator.cpu.b);
        // After `ld H, $AB` and `ld HL, $C000`, H should be $C0.
        assert_eq!(emulator.cpu.h, 0xC0, "Register H after test ROM. Expected: 0xC0, Actual: {:#04X}", emulator.cpu.h);
        assert_eq!(emulator.cpu.l, 0x00, "Register L after test ROM. Expected: 0x00, Actual: {:#04X}", emulator.cpu.l);

        // Verify other registers are not unexpectedly changed (basic check)
        // C and E were targets of LD r,r with initial zeroed values (assuming default CPU state is zeroed)
        // This depends on Cpu::new() initializing registers to 0.
        // From cpu/mod.rs, Cpu::new() does initialize them to 0.
        // `ld B, C` (C is 0) => B becomes 0, then `ld B, [HL]` => B becomes 0x34
        // `ld A, E` (E is 0) => A becomes 0, then `ld A, $12`, then `ld A, $34`
        assert_eq!(emulator.cpu.c, 0x00, "Register C after test ROM. Expected: 0x00, Actual: {:#04X}", emulator.cpu.c);
        assert_eq!(emulator.cpu.d, 0x00, "Register D after test ROM. Expected: 0x00, Actual: {:#04X}", emulator.cpu.d);
        assert_eq!(emulator.cpu.e, 0x00, "Register E after test ROM. Expected: 0x00, Actual: {:#04X}", emulator.cpu.e);


        // 4. Assert memory state
        let val_at_c000 = emulator.bus.read(0xC000);
        assert_eq!(val_at_c000, 0x34, "Memory at 0xC000 after test ROM. Expected: 0x34, Actual: {:#04X}", val_at_c000);
    }
}
