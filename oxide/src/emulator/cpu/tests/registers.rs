// oxide/src/emulator/cpu/tests/registers.rs
// This file is included by oxide/src/emulator/cpu/registers.rs

mod tests {
    use crate::emulator::cpu::{Cpu, Flag, Reg16, Reg8};

    // Helper function to initialize a CPU instance with default values for tests
    fn init_cpu() -> Cpu {
        Cpu::new(0) // Assuming Cpu::default() or similar exists and is appropriate
    }

    // Tests will be added here in subsequent steps.

    #[test]
    fn test_reg16_lsb() {
        assert_eq!(Reg16::BC.lsb(), Reg8::C, "BC LSB should be C");
        assert_eq!(Reg16::DE.lsb(), Reg8::E, "DE LSB should be E");
        assert_eq!(Reg16::HL.lsb(), Reg8::L, "HL LSB should be L");
        assert_eq!(Reg16::SP.lsb(), Reg8::SPL, "SP LSB should be SPL");
        assert_eq!(Reg16::PC.lsb(), Reg8::PCL, "PC LSB should be PCL");
        assert_eq!(Reg16::AF.lsb(), Reg8::F, "AF LSB should be F");
        assert_eq!(Reg16::WZ.lsb(), Reg8::Z, "WZ LSB should be Z");
    }

    #[test]
    fn test_reg16_msb() {
        assert_eq!(Reg16::BC.msb(), Reg8::B, "BC MSB should be B");
        assert_eq!(Reg16::DE.msb(), Reg8::D, "DE MSB should be D");
        assert_eq!(Reg16::HL.msb(), Reg8::H, "HL MSB should be H");
        assert_eq!(Reg16::SP.msb(), Reg8::SPH, "SP MSB should be SPH");
        assert_eq!(Reg16::PC.msb(), Reg8::PCH, "PC MSB should be PCH");
        assert_eq!(Reg16::AF.msb(), Reg8::A, "AF MSB should be A");
        assert_eq!(Reg16::WZ.msb(), Reg8::W, "WZ MSB should be W");
    }

    #[test]
    fn test_cpu_flags() {
        let mut cpu = init_cpu();

        // Test each flag individually (set to 1, then to 0)
        // Flag Z
        cpu.set_flag(Flag::Z, 1);
        assert_eq!(cpu.get_flag(Flag::Z), 1, "Flag Z should be 1");
        cpu.set_flag(Flag::Z, 0);
        assert_eq!(cpu.get_flag(Flag::Z), 0, "Flag Z should be 0");

        // Flag N
        cpu.set_flag(Flag::N, 1);
        assert_eq!(cpu.get_flag(Flag::N), 1, "Flag N should be 1");
        cpu.set_flag(Flag::N, 0);
        assert_eq!(cpu.get_flag(Flag::N), 0, "Flag N should be 0");

        // Flag H
        cpu.set_flag(Flag::H, 1);
        assert_eq!(cpu.get_flag(Flag::H), 1, "Flag H should be 1");
        cpu.set_flag(Flag::H, 0);
        assert_eq!(cpu.get_flag(Flag::H), 0, "Flag H should be 0");

        // Flag C
        cpu.set_flag(Flag::C, 1);
        assert_eq!(cpu.get_flag(Flag::C), 1, "Flag C should be 1");
        cpu.set_flag(Flag::C, 0);
        assert_eq!(cpu.get_flag(Flag::C), 0, "Flag C should be 0");

        // Test masking behavior (value & 1)
        // Flag Z with value 2 (binary 10), 2 & 1 = 0
        cpu.set_flag(Flag::Z, 2);
        assert_eq!(cpu.get_flag(Flag::Z), 0, "Flag Z should be 0 after setting with 2");

        // Flag N with value 0xFF (binary ...1111), 0xFF & 1 = 1
        cpu.set_flag(Flag::N, 0xFF);
        assert_eq!(cpu.get_flag(Flag::N), 1, "Flag N should be 1 after setting with 0xFF");

        // Flag H with value 0xFE (binary ...1110), 0xFE & 1 = 0
        cpu.set_flag(Flag::H, 0xFE);
        assert_eq!(cpu.get_flag(Flag::H), 0, "Flag H should be 0 after setting with 0xFE");

        // Test flag independence
        // Set all flags to a known state: Z=1, N=0, H=1, C=0 (0b1010xxxx for F register, if F is initially 0)
        // Assuming F register is initially 0 after init_cpu() or relevant bits are 0.
        // The actual F register value depends on how set_flag is implemented if it sets the whole F or just bits.
        // The tests above confirmed individual bit setting. Now let's confirm non-interference.

        cpu.set_flag(Flag::Z, 1);
        cpu.set_flag(Flag::N, 0);
        cpu.set_flag(Flag::H, 1);
        cpu.set_flag(Flag::C, 0);

        assert_eq!(cpu.get_flag(Flag::Z), 1, "Independence test: Z initial");
        assert_eq!(cpu.get_flag(Flag::N), 0, "Independence test: N initial");
        assert_eq!(cpu.get_flag(Flag::H), 1, "Independence test: H initial");
        assert_eq!(cpu.get_flag(Flag::C), 0, "Independence test: C initial");

        // Change Flag N, others should remain
        cpu.set_flag(Flag::N, 1);
        assert_eq!(cpu.get_flag(Flag::Z), 1, "Independence test: Z after N change");
        assert_eq!(cpu.get_flag(Flag::N), 1, "Independence test: N after N change");
        assert_eq!(cpu.get_flag(Flag::H), 1, "Independence test: H after N change");
        assert_eq!(cpu.get_flag(Flag::C), 0, "Independence test: C after N change");

        // Change Flag C, others should remain
        cpu.set_flag(Flag::C, 1);
        assert_eq!(cpu.get_flag(Flag::Z), 1, "Independence test: Z after C change");
        assert_eq!(cpu.get_flag(Flag::N), 1, "Independence test: N after C change");
        assert_eq!(cpu.get_flag(Flag::H), 1, "Independence test: H after C change");
        assert_eq!(cpu.get_flag(Flag::C), 1, "Independence test: C after C change");

        // Change Flag Z, others should remain
        cpu.set_flag(Flag::Z, 0);
        assert_eq!(cpu.get_flag(Flag::Z), 0, "Independence test: Z after Z change");
        assert_eq!(cpu.get_flag(Flag::N), 1, "Independence test: N after Z change");
        assert_eq!(cpu.get_flag(Flag::H), 1, "Independence test: H after Z change");
        assert_eq!(cpu.get_flag(Flag::C), 1, "Independence test: C after Z change");

        // Change Flag H, others should remain
        cpu.set_flag(Flag::H, 0);
        assert_eq!(cpu.get_flag(Flag::Z), 0, "Independence test: Z after H change");
        assert_eq!(cpu.get_flag(Flag::N), 1, "Independence test: N after H change");
        assert_eq!(cpu.get_flag(Flag::H), 0, "Independence test: H after H change");
        assert_eq!(cpu.get_flag(Flag::C), 1, "Independence test: C after H change");
    }

    #[test]
    fn test_cpu_read_write_8bit_registers() {
        let mut cpu = init_cpu();

        // Test general purpose registers
        let registers_to_test = [
            Reg8::A, Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L, Reg8::W, Reg8::Z,
        ];
        let values_to_test: [u8; 3] = [0x00, 0xFF, 0x5A];

        for &reg in registers_to_test.iter() {
            for &val in values_to_test.iter() {
                cpu.write8(reg, val);
                assert_eq!(cpu.read8(reg), val, "Testing {:?} with value {:#04x}", reg, val);
            }
        }

        // Test F register specifically. Standard behavior is that lower 4 bits of F might be read as 0 or ignored by instructions.
        // However, a direct write to F should store the value, and a direct read should retrieve it.
        // The problem states "The F register is special, only the upper 4 bits are usable (ZNHC).
        // The lower 4 bits are always 0." This implies read8(Reg8::F) should return a value with lower bits zeroed.
        // Let's assume write8(Reg8::F, value) stores 'value', but read8(Reg8::F) returns 'value & 0xF0'.
        // This needs to be based on *expected/standard* behavior.
        // For many Z80-like CPUs (Game Boy CPU is a Z80 variant), the F register's lower bits are indeed masked to 0 on read.
        cpu.write8(Reg8::F, 0xFF); // Attempt to write 1s to all bits
        assert_eq!(cpu.read8(Reg8::F), 0xF0, "Testing Reg8::F write 0xFF, read should be 0xF0 (lower bits masked)");
        cpu.write8(Reg8::F, 0x55); // Attempt to write 01010101
        assert_eq!(cpu.read8(Reg8::F), 0x50, "Testing Reg8::F write 0x55, read should be 0x50");
        cpu.write8(Reg8::F, 0x00);
        assert_eq!(cpu.read8(Reg8::F), 0x00, "Testing Reg8::F write 0x00, read should be 0x00");
         cpu.write8(Reg8::F, 0b10101111); // Z=1, N=0, H=1, C=0, lower nibble 1111
        assert_eq!(cpu.read8(Reg8::F), 0b10100000, "F should be 0xA0 after writing 0b10101111");


        // Test PC High/Low bytes
        cpu.write16(Reg16::PC, 0x1234); // Initial PC value
        cpu.write8(Reg8::PCH, 0xAB);
        assert_eq!(cpu.read16(Reg16::PC), 0xAB34, "PC should be 0xAB34 after PCH write");
        assert_eq!(cpu.read8(Reg8::PCH), 0xAB, "PCH should read 0xAB");
        assert_eq!(cpu.read8(Reg8::PCL), 0x34, "PCL should remain 0x34");

        cpu.write16(Reg16::PC, 0x1234); // Reset PC
        cpu.write8(Reg8::PCL, 0xCD);
        assert_eq!(cpu.read16(Reg16::PC), 0x12CD, "PC should be 0x12CD after PCL write");
        assert_eq!(cpu.read8(Reg8::PCH), 0x12, "PCH should remain 0x12");
        assert_eq!(cpu.read8(Reg8::PCL), 0xCD, "PCL should read 0xCD");

        // Test SP High/Low bytes
        cpu.write16(Reg16::SP, 0x5678); // Initial SP value
        cpu.write8(Reg8::SPH, 0xDE);
        assert_eq!(cpu.read16(Reg16::SP), 0xDE78, "SP should be 0xDE78 after SPH write");
        assert_eq!(cpu.read8(Reg8::SPH), 0xDE, "SPH should read 0xDE");
        assert_eq!(cpu.read8(Reg8::SPL), 0x78, "SPL should remain 0x78");

        cpu.write16(Reg16::SP, 0x5678); // Reset SP
        cpu.write8(Reg8::SPL, 0xFA);
        assert_eq!(cpu.read16(Reg16::SP), 0x56FA, "SP should be 0x56FA after SPL write");
        assert_eq!(cpu.read8(Reg8::SPH), 0x56, "SPH should remain 0x56");
        assert_eq!(cpu.read8(Reg8::SPL), 0xFA, "SPL should read 0xFA");
    }

    #[test]
    fn test_cpu_read_write_16bit_registers() {
        let mut cpu = init_cpu();
        let values_to_test: [u16; 3] = [0x0000, 0xFFFF, 0x1234];

        // Test BC, DE, HL, SP, PC, WZ registers
        let registers_to_test = [
            (Reg16::BC, Reg8::B, Reg8::C),
            (Reg16::DE, Reg8::D, Reg8::E),
            (Reg16::HL, Reg8::H, Reg8::L),
            (Reg16::WZ, Reg8::W, Reg8::Z), // Assuming WZ is like other general 16-bit regs
        ];

        for &(reg16, reg_msb, reg_lsb) in registers_to_test.iter() {
            for &val16 in values_to_test.iter() {
                cpu.write16(reg16, val16);
                assert_eq!(cpu.read16(reg16), val16, "Testing {:?} write/read with {:#06x}", reg16, val16);
                assert_eq!(cpu.read8(reg_msb), (val16 >> 8) as u8, "Testing MSB of {:?} after writing {:#06x}", reg16, val16);
                assert_eq!(cpu.read8(reg_lsb), (val16 & 0xFF) as u8, "Testing LSB of {:?} after writing {:#06x}", reg16, val16);
            }
        }

        // Test SP and PC separately as they don't have direct Reg8 MSB/LSB enum pairs in the same way for the loop above
        // (though they do have PCH/PCL, SPH/SPL which are handled by write8/read8 tests)
        for &val16 in values_to_test.iter() {
            cpu.write16(Reg16::SP, val16);
            assert_eq!(cpu.read16(Reg16::SP), val16, "Testing Reg16::SP write/read with {:#06x}", val16);
            // Individual byte checks for SP through PCH/L SPH/L tested in 8bit tests.
        }
        for &val16 in values_to_test.iter() {
            cpu.write16(Reg16::PC, val16);
            assert_eq!(cpu.read16(Reg16::PC), val16, "Testing Reg16::PC write/read with {:#06x}", val16);
        }

        // Test AF register (special F masking)
        // write16(Reg16::AF, value) implies A = high_byte, F = low_byte & 0xF0
        // read16(Reg16::AF) implies (A << 8) | F
        cpu.write16(Reg16::AF, 0xABCD); // A=AB, F=CD. F becomes C0 after masking.
        assert_eq!(cpu.read8(Reg8::A), 0xAB, "AF write: A should be 0xAB");
        assert_eq!(cpu.read8(Reg8::F), 0xC0, "AF write: F should be 0xC0 (0xCD & 0xF0)");
        assert_eq!(cpu.read16(Reg16::AF), 0xABC0, "AF write/read: expected 0xABC0");

        cpu.write16(Reg16::AF, 0xFF55); // A=FF, F=55. F becomes 50.
        assert_eq!(cpu.read8(Reg8::A), 0xFF, "AF write: A should be 0xFF");
        assert_eq!(cpu.read8(Reg8::F), 0x50, "AF write: F should be 0x50 (0x55 & 0xF0)");
        assert_eq!(cpu.read16(Reg16::AF), 0xFF50, "AF write/read: expected 0xFF50");

        cpu.write16(Reg16::AF, 0x0000);
        assert_eq!(cpu.read8(Reg8::A), 0x00, "AF write: A should be 0x00");
        assert_eq!(cpu.read8(Reg8::F), 0x00, "AF write: F should be 0x00 (0x00 & 0xF0)");
        assert_eq!(cpu.read16(Reg16::AF), 0x0000, "AF write/read: expected 0x0000");

        // Test for potential bug in read16(Reg16::BC)
        // Current code: Reg16::BC => ((self.b as u16) << 8) | (self.b as u16),
        // Expected:     Reg16::BC => ((self.b as u16) << 8) | (self.c as u16),
        cpu.write8(Reg8::B, 0x11);
        cpu.write8(Reg8::C, 0x22);
        // This assertion is for the *expected* behavior.
        // If the bug (reading B twice) exists, read16(Reg16::BC) would return 0x1111.
        assert_eq!(cpu.read16(Reg16::BC), 0x1122, "BC read: expecting B and C, not B twice");

        // A quick re-test for DE and HL to be sure after BC focus
        cpu.write8(Reg8::D, 0x33);
        cpu.write8(Reg8::E, 0x44);
        assert_eq!(cpu.read16(Reg16::DE), 0x3344, "DE read: check composed value");
        cpu.write8(Reg8::H, 0x55);
        cpu.write8(Reg8::L, 0x66);
        assert_eq!(cpu.read16(Reg16::HL), 0x5566, "HL read: check composed value");
    }
}
