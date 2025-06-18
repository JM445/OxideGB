
#[cfg(test)]
mod tests {
    use crate::emulator::cpu::*;
    
    use std::num::Wrapping;
    #[test]
    fn test_alu_add() {
        // Standard ZNHC flags (3,2,1,0)
        // Z: res_8bit == 0
        // N: 0 for add
        // H: (left_nibble + right_nibble + carry_nibble) > 0xF
        // C: (left_8bit + right_8bit + carry_in_bit) > 0xFF

        // 1. No carry, no half-carry
        // 10 + 5 + 0 = 15. Flags: Z0 N0 H0 C0 (0b0000)
        let (res, flags) = Cpu::alu_add(Wrapping(10), Wrapping(5), Wrapping(0));
        assert_eq!(res & 0xFF, 15, "10+5+0 res");
        assert_eq!(flags, 0b0000, "10+5+0 flags");

        // 0x10 + 0x20 + 0 = 0x30. Flags: Z0 N0 H0 C0 (0b0000)
        let (res, flags) = Cpu::alu_add(Wrapping(0x10), Wrapping(0x20), Wrapping(0));
        assert_eq!(res & 0xFF, 0x30, "0x10+0x20+0 res");
        assert_eq!(flags, 0b0000, "0x10+0x20+0 flags");

        // 2. Half-carry, no carry
        // 0x0F + 0x01 + 0 = 0x10. Flags: Z0 N0 H1 C0 (0b0010)
        let (res, flags) = Cpu::alu_add(Wrapping(0x0F), Wrapping(0x01), Wrapping(0));
        assert_eq!(res & 0xFF, 0x10, "0x0F+0x01+0 res");
        assert_eq!(flags, 0b0010, "0x0F+0x01+0 flags");

        // 0x08 + 0x08 + 0 = 0x10. Flags: Z0 N0 H1 C0 (0b0010)
        let (res, flags) = Cpu::alu_add(Wrapping(0x08), Wrapping(0x08), Wrapping(0));
        assert_eq!(res & 0xFF, 0x10, "0x08+0x08+0 res");
        assert_eq!(flags, 0b0010, "0x08+0x08+0 flags");

        // 3. Carry, with/without half-carry
        // 0xFF + 0x01 + 0 = 0x00. H: (0xF+0x1+0)>0xF (yes). C: (0xFF+0x01+0)>0xFF (yes). Flags: Z1 N0 H1 C1 (0b1011)
        let (res, flags) = Cpu::alu_add(Wrapping(0xFF), Wrapping(0x01), Wrapping(0));
        assert_eq!(res & 0xFF, 0x00, "0xFF+0x01+0 res");
        assert_eq!(flags, 0b1011, "0xFF+0x01+0 flags");

        // 0x80 + 0x80 + 0 = 0x00. H: (0x0+0x0+0)>0xF (no). C: (0x80+0x80+0)>0xFF (yes). Flags: Z1 N0 H0 C1 (0b1001)
        let (res, flags) = Cpu::alu_add(Wrapping(0x80), Wrapping(0x80), Wrapping(0));
        assert_eq!(res & 0xFF, 0x00, "0x80+0x80+0 res");
        assert_eq!(flags, 0b1001, "0x80+0x80+0 flags");

        // 0xF0 + 0x10 + 0 = 0x00. H: (0x0+0x0+0)>0xF (no). C: (0xF0+0x10+0)>0xFF (yes). Flags: Z1 N0 H0 C1 (0b1001)
        let (res, flags) = Cpu::alu_add(Wrapping(0xF0), Wrapping(0x10), Wrapping(0));
        assert_eq!(res & 0xFF, 0x00, "0xF0+0x10+0 res");
        assert_eq!(flags, 0b1001, "0xF0+0x10+0 flags");

        // 4. With Carry In
        // 10 + 5 + 1 = 16. H: (0xA+0x5+1)>0xF (yes). C: (10+5+1)>0xFF (no). Flags: Z0 N0 H1 C0 (0b0010)
        let (res, flags) = Cpu::alu_add(Wrapping(10), Wrapping(5), Wrapping(1));
        assert_eq!(res & 0xFF, 16, "10+5+1 res");
        assert_eq!(flags, 0b0010, "10+5+1 flags");

        // 0x0F + 0x01 + 1 = 0x11. H: (0xF+0x1+1)>0xF (yes). C: (0x0F+0x01+1)>0xFF (no). Flags: Z0 N0 H1 C0 (0b0010)
        let (res, flags) = Cpu::alu_add(Wrapping(0x0F), Wrapping(0x01), Wrapping(1));
        assert_eq!(res & 0xFF, 0x11, "0x0F+0x01+1 res");
        assert_eq!(flags, 0b0010, "0x0F+0x01+1 flags");

        // 0xFF + 0x00 + 1 = 0x00. H: (0xF+0x0+1)>0xF (yes). C: (0xFF+0x00+1)>0xFF (yes). Flags: Z1 N0 H1 C1 (0b1011)
        let (res, flags) = Cpu::alu_add(Wrapping(0xFF), Wrapping(0x00), Wrapping(1));
        assert_eq!(res & 0xFF, 0x00, "0xFF+0x00+1 res");
        assert_eq!(flags, 0b1011, "0xFF+0x00+1 flags");

        // 0xFE + 0x01 + 1 = 0x00. H: (0xE+0x1+1)>0xF (yes). C: (0xFE+0x01+1)>0xFF (yes). Flags: Z1 N0 H1 C1 (0b1011)
        let (res, flags) = Cpu::alu_add(Wrapping(0xFE), Wrapping(0x01), Wrapping(1));
        assert_eq!(res & 0xFF, 0x00, "0xFE+0x01+1 res");
        assert_eq!(flags, 0b1011, "0xFE+0x01+1 flags");

        // 5. Result Zero
        // 0x00 + 0x00 + 0 = 0x00. H: (0+0+0)>0xF (no). C: (0+0+0)>0xFF (no). Flags: Z1 N0 H0 C0 (0b1000)
        let (res, flags) = Cpu::alu_add(Wrapping(0x00), Wrapping(0x00), Wrapping(0));
        assert_eq!(res & 0xFF, 0x00, "0x00+0x00+0 res");
        assert_eq!(flags, 0b1000, "0x00+0x00+0 flags");
    }

    #[test]
    fn test_alu_sub() {
        // Standard ZNHC flags (3,2,1,0)
        // Z: res_8bit == 0
        // N: 1 for sub
        // H: (left_8bit & 0x0F) < (right_8bit & 0x0F) + borrow
        // C: (left_8bit as u16) < (right_8bit as u16) + (borrow as u16)

        // 1. Basic Subtraction (no borrow, no half-borrow)
        // 10 - 5 - 0 = 5. H: (A < 5+0) is false. C: (10 < 5+0) is false. Flags: Z0 N1 H0 C0 (0b0100)
        let (res, flags) = Cpu::alu_sub(Wrapping(10), Wrapping(5), Wrapping(0));
        assert_eq!(res & 0xFF, 5, "10-5-0 res");
        assert_eq!(flags, 0b0100, "10-5-0 flags");

        // 0x30 - 0x10 - 0 = 0x20. H: (0 < 0+0) is false. C: (0x30 < 0x10+0) is false. Flags: Z0 N1 H0 C0 (0b0100)
        let (res, flags) = Cpu::alu_sub(Wrapping(0x30), Wrapping(0x10), Wrapping(0));
        assert_eq!(res & 0xFF, 0x20, "0x30-0x10-0 res");
        assert_eq!(flags, 0b0100, "0x30-0x10-0 flags");

        // 2. Half-Borrow (no full borrow)
        // 0x10 - 0x01 - 0 = 0x0F. H: (0 < 1+0) is true. C: (0x10 < 0x01+0) is false. Flags: Z0 N1 H1 C0 (0b0110)
        let (res, flags) = Cpu::alu_sub(Wrapping(0x10), Wrapping(0x01), Wrapping(0));
        assert_eq!(res & 0xFF, 0x0F, "0x10-0x01-0 res");
        assert_eq!(flags, 0b0110, "0x10-0x01-0 flags");

        // 0x24 - 0x05 - 0 = 0x1F. H: (4 < 5+0) is true. C: (0x24 < 0x05+0) is false. Flags: Z0 N1 H1 C0 (0b0110)
        let (res, flags) = Cpu::alu_sub(Wrapping(0x24), Wrapping(0x05), Wrapping(0));
        assert_eq!(res & 0xFF, 0x1F, "0x24-0x05-0 res");
        assert_eq!(flags, 0b0110, "0x24-0x05-0 flags");

        // 3. Full Borrow
        // 0x05 - 0x0A - 0 = 0xFB. H: (5 < A+0) is true. C: (0x05 < 0x0A+0) is true. Flags: Z0 N1 H1 C1 (0b0111)
        let (res, flags) = Cpu::alu_sub(Wrapping(0x05), Wrapping(0x0A), Wrapping(0));
        assert_eq!(res & 0xFF, 0xFB, "0x05-0x0A-0 res");
        assert_eq!(flags, 0b0111, "0x05-0x0A-0 flags");

        // 0x00 - 0x01 - 0 = 0xFF. H: (0 < 1+0) is true. C: (0x00 < 0x01+0) is true. Flags: Z0 N1 H1 C1 (0b0111)
        let (res, flags) = Cpu::alu_sub(Wrapping(0x00), Wrapping(0x01), Wrapping(0));
        assert_eq!(res & 0xFF, 0xFF, "0x00-0x01-0 res");
        assert_eq!(flags, 0b0111, "0x00-0x01-0 flags");

        // 0x40 - 0x80 - 0 = 0xC0. H: (0 < 0+0) is false. C: (0x40 < 0x80+0) is true. Flags: Z0 N1 H0 C1 (0b0101)
        let (res, flags) = Cpu::alu_sub(Wrapping(0x40), Wrapping(0x80), Wrapping(0));
        assert_eq!(res & 0xFF, 0xC0, "0x40-0x80-0 res");
        assert_eq!(flags, 0b0101, "0x40-0x80-0 flags");

        // 4. Result Zero
        // 0x0A - 0x0A - 0 = 0x00. H: (A < A+0) is false. C: (0x0A < 0x0A+0) is false. Flags: Z1 N1 H0 C0 (0b1100)
        let (res, flags) = Cpu::alu_sub(Wrapping(0x0A), Wrapping(0x0A), Wrapping(0));
        assert_eq!(res & 0xFF, 0x00, "0x0A-0x0A-0 res");
        assert_eq!(flags, 0b1100, "0x0A-0x0A-0 flags");

        // 5. With Incoming Borrow (carry_in_as_borrow = 1)
        // 10 - 5 - 1 = 4. H: (A < 5+1) is false. C: (10 < 5+1) is false. Flags: Z0 N1 H0 C0 (0b0100)
        let (res, flags) = Cpu::alu_sub(Wrapping(10), Wrapping(5), Wrapping(1));
        assert_eq!(res & 0xFF, 4, "10-5-1 res");
        assert_eq!(flags, 0b0100, "10-5-1 flags");

        // 0x10 - 0x01 - 1 = 0x0E. H: (0 < 1+1) is true. C: (0x10 < 0x01+1) is false. Flags: Z0 N1 H1 C0 (0b0110)
        let (res, flags) = Cpu::alu_sub(Wrapping(0x10), Wrapping(0x01), Wrapping(1));
        assert_eq!(res & 0xFF, 0x0E, "0x10-0x01-1 res");
        assert_eq!(flags, 0b0110, "0x10-0x01-1 flags");

        // 0x00 - 0x00 - 1 = 0xFF. H: (0 < 0+1) is true. C: (0x00 < 0x00+1) is true. Flags: Z0 N1 H1 C1 (0b0111)
        let (res, flags) = Cpu::alu_sub(Wrapping(0x00), Wrapping(0x00), Wrapping(1));
        assert_eq!(res & 0xFF, 0xFF, "0x00-0x00-1 res");
        assert_eq!(flags, 0b0111, "0x00-0x00-1 flags");
    }

    #[test]
    fn test_alu_and() {
        // Standard ZNHC flags (3,2,1,0). For AND: N=0, H=1, C=0. Z is result-based.
        // Expected flags: 0bZ010.

        // Case 1: 0x0F & 0xF0 = 0x00. res=0x00. Z=1. Expected flags: 0b1010.
        let (res, flags) = Cpu::alu_and(Wrapping(0x0F), Wrapping(0xF0));
        assert_eq!(res & 0xFF, 0x00, "Result for 0x0F & 0xF0");
        assert_eq!(flags, 0b1010, "Flags for 0x0F & 0xF0");

        // Case 2: 0x12 & 0x00 = 0x00. res=0x00. Z=1. Expected flags: 0b1010.
        let (res, flags) = Cpu::alu_and(Wrapping(0x12), Wrapping(0x00));
        assert_eq!(res & 0xFF, 0x00, "Result for 0x12 & 0x00");
        assert_eq!(flags, 0b1010, "Flags for 0x12 & 0x00");

        // Case 3: 0x34 & 0xFF = 0x34. res=0x34. Z=0. Expected flags: 0b0010.
        let (res, flags) = Cpu::alu_and(Wrapping(0x34), Wrapping(0xFF));
        assert_eq!(res & 0xFF, 0x34, "Result for 0x34 & 0xFF");
        assert_eq!(flags, 0b0010, "Flags for 0x34 & 0xFF");

        // Case 4: 0x5A & 0x3C = 0x18. res=0x18. Z=0. Expected flags: 0b0010.
        let (res, flags) = Cpu::alu_and(Wrapping(0x5A), Wrapping(0x3C));
        assert_eq!(res & 0xFF, 0x18, "Result for 0x5A & 0x3C");
        assert_eq!(flags, 0b0010, "Flags for 0x5A & 0x3C");

        // Case 5: 0x00 & 0xFF = 0x00. res=0x00. Z=1. Expected flags: 0b1010.
        let (res, flags) = Cpu::alu_and(Wrapping(0x00), Wrapping(0xFF));
        assert_eq!(res & 0xFF, 0x00, "Result for 0x00 & 0xFF");
        assert_eq!(flags, 0b1010, "Flags for 0x00 & 0xFF");

        // Case 6: (Covered by Case 1) 0x0F & 0xF0 = 0x00. res=0x00. Z=1. Expected flags: 0b1010.
        // Re-asserting for clarity, same as case 1.
        let (res, flags) = Cpu::alu_and(Wrapping(0x0F), Wrapping(0xF0));
        assert_eq!(res & 0xFF, 0x00, "Result for 0x0F & 0xF0 (Z based on res)");
        assert_eq!(flags, 0b1010, "Flags for 0x0F & 0xF0 (Z based on res)");

        // Case 7: 0xAF & 0xF1 = 0xA1. res=0xA1. Z=0. Expected flags: 0b0010.
        let (res, flags) = Cpu::alu_and(Wrapping(0xAF), Wrapping(0xF1));
        assert_eq!(res & 0xFF, 0xA1, "Result for 0xAF & 0xF1");
        assert_eq!(flags, 0b0010, "Flags for 0xAF & 0xF1");

        // Case 8: (New, variant of Case 4 from prompt) 0x55 & 0xAA = 0x00. res=0x00. Z=1. Expected flags: 0b1010.
        let (res, flags) = Cpu::alu_and(Wrapping(0x55), Wrapping(0xAA));
        assert_eq!(res & 0xFF, 0x00, "Result for 0x55 & 0xAA");
        assert_eq!(flags, 0b1010, "Flags for 0x55 & 0xAA");
    }

    #[test]
    fn test_alu_or() {
        // Standard ZNHC flags (3,2,1,0). For OR: N=0, H=0, C=0. Z is result-based.
        // Expected flags: 0bZ000.

        // Case 1: 0x0F | 0xF0 = 0xFF. res=0xFF. Z=0. Expected flags: 0b0000.
        let (res, flags) = Cpu::alu_or(Wrapping(0x0F), Wrapping(0xF0));
        assert_eq!(res & 0xFF, 0xFF, "Result for 0x0F | 0xF0");
        assert_eq!(flags, 0b0000, "Flags for 0x0F | 0xF0");

        // Case 2: 0x12 | 0x00 = 0x12. res=0x12. Z=0. Expected flags: 0b0000.
        let (res, flags) = Cpu::alu_or(Wrapping(0x12), Wrapping(0x00));
        assert_eq!(res & 0xFF, 0x12, "Result for 0x12 | 0x00");
        assert_eq!(flags, 0b0000, "Flags for 0x12 | 0x00");

        // Case 3: 0x34 | 0xFF = 0xFF. res=0xFF. Z=0. Expected flags: 0b0000.
        let (res, flags) = Cpu::alu_or(Wrapping(0x34), Wrapping(0xFF));
        assert_eq!(res & 0xFF, 0xFF, "Result for 0x34 | 0xFF");
        assert_eq!(flags, 0b0000, "Flags for 0x34 | 0xFF");

        // Case 4: 0x00 | 0x00 = 0x00. res=0x00. Z=1. Expected flags: 0b1000.
        let (res, flags) = Cpu::alu_or(Wrapping(0x00), Wrapping(0x00));
        assert_eq!(res & 0xFF, 0x00, "Result for 0x00 | 0x00");
        assert_eq!(flags, 0b1000, "Flags for 0x00 | 0x00");

        // Case 5: 0x00 | 0x55 = 0x55. res=0x55. Z=0. Expected flags: 0b0000.
        let (res, flags) = Cpu::alu_or(Wrapping(0x00), Wrapping(0x55));
        assert_eq!(res & 0xFF, 0x55, "Result for 0x00 | 0x55");
        assert_eq!(flags, 0b0000, "Flags for 0x00 | 0x55");

        // Case 6: 0xAA | 0x55 = 0xFF. res=0xFF. Z=0. Expected flags: 0b0000.
        let (res, flags) = Cpu::alu_or(Wrapping(0xAA), Wrapping(0x55));
        assert_eq!(res & 0xFF, 0xFF, "Result for 0xAA | 0x55");
        assert_eq!(flags, 0b0000, "Flags for 0xAA | 0x55");

        // Case 7: 0xCC | 0x00 = 0xCC. res=0xCC. Z=0. Expected flags: 0b0000.
        let (res, flags) = Cpu::alu_or(Wrapping(0xCC), Wrapping(0x00));
        assert_eq!(res & 0xFF, 0xCC, "Result for 0xCC | 0x00");
        assert_eq!(flags, 0b0000, "Flags for 0xCC | 0x00");
    }

    #[test]
    fn test_alu_xor() {
        // Standard ZNHC flags (3,2,1,0). For XOR: N=0, H=0, C=0. Z is result-based.
        // Expected flags: 0bZ000.

        // Case 1: 0x0F ^ 0xF0 = 0xFF. res=0xFF. Z=0. Expected flags: 0b0000.
        let (res, flags) = Cpu::alu_xor(Wrapping(0x0F), Wrapping(0xF0));
        assert_eq!(res & 0xFF, 0xFF, "Result for 0x0F ^ 0xF0");
        assert_eq!(flags, 0b0000, "Flags for 0x0F ^ 0xF0");

        // Case 2: 0x12 ^ 0x00 = 0x12. res=0x12. Z=0. Expected flags: 0b0000.
        let (res, flags) = Cpu::alu_xor(Wrapping(0x12), Wrapping(0x00));
        assert_eq!(res & 0xFF, 0x12, "Result for 0x12 ^ 0x00");
        assert_eq!(flags, 0b0000, "Flags for 0x12 ^ 0x00");

        // Case 3: 0x34 ^ 0xFF = 0xCB. res=0xCB. Z=0. Expected flags: 0b0000.
        let (res, flags) = Cpu::alu_xor(Wrapping(0x34), Wrapping(0xFF));
        assert_eq!(res & 0xFF, 0xCB, "Result for 0x34 ^ 0xFF");
        assert_eq!(flags, 0b0000, "Flags for 0x34 ^ 0xFF");

        // Case 4: 0x5A ^ 0x5A = 0x00. res=0x00. Z=1. Expected flags: 0b1000.
        let (res, flags) = Cpu::alu_xor(Wrapping(0x5A), Wrapping(0x5A));
        assert_eq!(res & 0xFF, 0x00, "Result for 0x5A ^ 0x5A");
        assert_eq!(flags, 0b1000, "Flags for 0x5A ^ 0x5A");

        // Case 5: 0x00 ^ 0x55 = 0x55. res=0x55. Z=0. Expected flags: 0b0000.
        let (res, flags) = Cpu::alu_xor(Wrapping(0x00), Wrapping(0x55));
        assert_eq!(res & 0xFF, 0x55, "Result for 0x00 ^ 0x55");
        assert_eq!(flags, 0b0000, "Flags for 0x00 ^ 0x55");

        // Case 6: (variant of Case 4) 0xAA ^ 0xAA = 0x00. res=0x00. Z=1. Expected flags: 0b1000.
        let (res, flags) = Cpu::alu_xor(Wrapping(0xAA), Wrapping(0xAA));
        assert_eq!(res & 0xFF, 0x00, "Result for 0xAA ^ 0xAA");
        assert_eq!(flags, 0b1000, "Flags for 0xAA ^ 0xAA");

        // Case 7: 0xAF ^ 0xF1 = 0x5E. res=0x5E. Z=0. Expected flags: 0b0000.
        let (res, flags) = Cpu::alu_xor(Wrapping(0xAF), Wrapping(0xF1));
        assert_eq!(res & 0xFF, 0x5E, "Result for 0xAF ^ 0xF1");
        assert_eq!(flags, 0b0000, "Flags for 0xAF ^ 0xF1");

        // Case 8: (New) 0x00 ^ 0x00 = 0x00. res=0x00. Z=1. Expected flags: 0b1000.
        let (res, flags) = Cpu::alu_xor(Wrapping(0x00), Wrapping(0x00));
        assert_eq!(res & 0xFF, 0x00, "Result for 0x00 ^ 0x00");
        assert_eq!(flags, 0b1000, "Flags for 0x00 ^ 0x00");
    }

    #[test]
    fn test_alu_rsh() {
        // Flags: Z00C (Z determined by result, N=0, H=0, C=shifted-out bit)

        // Test ShiftType::R (Rotate Right through Carry)
        // fn alu_rsh(shift: ShiftType, val: Wrapping<u8>, old_c: u8) -> (u16, u8)
        // res = (val >> 1) | (old_c << 7), new_c = val & 1
        let (res, flags) = Cpu::alu_rsh(ShiftType::R, Wrapping(0x8A), 0); // 10001010, old_c=0
        assert_eq!(res & 0xFF, 0x45, "RSH R: 0x8A, old_c=0 -> res"); // 01000101
        assert_eq!(flags, 0b0000, "RSH R: 0x8A, old_c=0 -> flags"); // C=0 (1000101(0))

        let (res, flags) = Cpu::alu_rsh(ShiftType::R, Wrapping(0x8A), 1); // 10001010, old_c=1
        assert_eq!(res & 0xFF, 0xC5, "RSH R: 0x8A, old_c=1 -> res"); // 11000101
        assert_eq!(flags, 0b0000, "RSH R: 0x8A, old_c=1 -> flags"); // C=0 (1000101(0))

        let (res, flags) = Cpu::alu_rsh(ShiftType::R, Wrapping(0x01), 0); // 00000001, old_c=0
        assert_eq!(res & 0xFF, 0x00, "RSH R: 0x01, old_c=0 -> res"); // 00000000
        assert_eq!(flags, 0b1001, "RSH R: 0x01, old_c=0 -> flags"); // Z=1, C=1 (0000000(1))

        let (res, flags) = Cpu::alu_rsh(ShiftType::R, Wrapping(0x00), 1); // 00000000, old_c=1
        assert_eq!(res & 0xFF, 0x80, "RSH R: 0x00, old_c=1 -> res"); // 10000000
        assert_eq!(flags, 0b0000, "RSH R: 0x00, old_c=1 -> flags"); // C=0 (0000000(0))

        let (res, flags) = Cpu::alu_rsh(ShiftType::R, Wrapping(0x00), 0); // 00000000, old_c=0
        assert_eq!(res & 0xFF, 0x00, "RSH R: 0x00, old_c=0 -> res"); // 00000000
        assert_eq!(flags, 0b1000, "RSH R: 0x00, old_c=0 -> flags"); // Z=1, C=0 (0000000(0))

        // Test ShiftType::RC (Rotate Right Circular)
        // res = (val >> 1) | (val << 7), new_c = val & 1. Note: (val << 7) is (val & 1) << 7
        let (res, flags) = Cpu::alu_rsh(ShiftType::RC, Wrapping(0x8A), 0); // 10001010, (old_c ignored)
        assert_eq!(res & 0xFF, 0x45, "RSH RC: 0x8A -> res"); // 01000101
        assert_eq!(flags, 0b0000, "RSH RC: 0x8A -> flags"); // C=0 (1000101(0))

        let (res, flags) = Cpu::alu_rsh(ShiftType::RC, Wrapping(0x01), 0); // 00000001
        assert_eq!(res & 0xFF, 0x80, "RSH RC: 0x01 -> res"); // 10000000
        assert_eq!(flags, 0b0001, "RSH RC: 0x01 -> flags"); // C=1 (0000000(1))

        let (res, flags) = Cpu::alu_rsh(ShiftType::RC, Wrapping(0x00), 0); // 00000000
        assert_eq!(res & 0xFF, 0x00, "RSH RC: 0x00 -> res"); // 00000000
        assert_eq!(flags, 0b1000, "RSH RC: 0x00 -> flags"); // Z=1, C=0

        // Test ShiftType::SL (Shift Right Logical) SRL
        // res = val >> 1, new_c = val & 1
        let (res, flags) = Cpu::alu_rsh(ShiftType::SL, Wrapping(0x8A), 0); // 10001010 (old_c ignored)
        assert_eq!(res & 0xFF, 0x45, "RSH SL: 0x8A -> res"); // 01000101
        assert_eq!(flags, 0b0000, "RSH SL: 0x8A -> flags"); // C=0

        let (res, flags) = Cpu::alu_rsh(ShiftType::SL, Wrapping(0x01), 0); // 00000001
        assert_eq!(res & 0xFF, 0x00, "RSH SL: 0x01 -> res"); // 00000000
        assert_eq!(flags, 0b1001, "RSH SL: 0x01 -> flags"); // Z=1, C=1

        let (res, flags) = Cpu::alu_rsh(ShiftType::SL, Wrapping(0xFF), 0); // 11111111
        assert_eq!(res & 0xFF, 0x7F, "RSH SL: 0xFF -> res"); // 01111111
        assert_eq!(flags, 0b0001, "RSH SL: 0xFF -> flags"); // C=1

        let (res, flags) = Cpu::alu_rsh(ShiftType::SL, Wrapping(0x00), 0); // 00000000
        assert_eq!(res & 0xFF, 0x00, "RSH SL: 0x00 -> res");
        assert_eq!(flags, 0b1000, "RSH SL: 0x00 -> flags"); // Z=1, C=0

        // Test ShiftType::SA (Shift Right Arithmetic) SRA
        // res = (val >> 1) | (val & 0x80), new_c = val & 1
        let (res, flags) = Cpu::alu_rsh(ShiftType::SA, Wrapping(0x8A), 0); // 10001010 (old_c ignored)
        assert_eq!(res & 0xFF, 0xC5, "RSH SA: 0x8A -> res"); // 11000101
        assert_eq!(flags, 0b0000, "RSH SA: 0x8A -> flags"); // C=0

        let (res, flags) = Cpu::alu_rsh(ShiftType::SA, Wrapping(0x0A), 0); // 00001010
        assert_eq!(res & 0xFF, 0x05, "RSH SA: 0x0A -> res"); // 00000101
        assert_eq!(flags, 0b0000, "RSH SA: 0x0A -> flags"); // C=0

        let (res, flags) = Cpu::alu_rsh(ShiftType::SA, Wrapping(0x01), 0); // 00000001
        assert_eq!(res & 0xFF, 0x00, "RSH SA: 0x01 -> res"); // 00000000
        assert_eq!(flags, 0b1001, "RSH SA: 0x01 -> flags"); // Z=1, C=1

        let (res, flags) = Cpu::alu_rsh(ShiftType::SA, Wrapping(0x81), 0); // 10000001
        assert_eq!(res & 0xFF, 0xC0, "RSH SA: 0x81 -> res"); // 11000000
        assert_eq!(flags, 0b0001, "RSH SA: 0x81 -> flags"); // C=1

        let (res, flags) = Cpu::alu_rsh(ShiftType::SA, Wrapping(0x00), 0); // 00000000
        assert_eq!(res & 0xFF, 0x00, "RSH SA: 0x00 -> res");
        assert_eq!(flags, 0b1000, "RSH SA: 0x00 -> flags"); // Z=1, C=0
    }

    #[test]
    fn test_alu_lsh() {
        // Flags: Z00C for R, RC, SL (Z by result, N=0, H=0, C=shifted-out bit)
        // Flags: Z000 for SA (Z by original val, N=0, H=0, C=0)

        // Test ShiftType::R (Rotate Left through Carry)
        // res = (val << 1) | old_c, new_c = (val & 0x80) >> 7
        let (res, flags) = Cpu::alu_lsh(ShiftType::R, Wrapping(0x51), 0); // 01010001, old_c=0
        assert_eq!(res & 0xFF, 0xA2, "LSH R: 0x51, old_c=0 -> res"); // 10100010
        assert_eq!(flags, 0b0000, "LSH R: 0x51, old_c=0 -> flags"); // C=0 from 01010001 MSB

        let (res, flags) = Cpu::alu_lsh(ShiftType::R, Wrapping(0x51), 1); // 01010001, old_c=1
        assert_eq!(res & 0xFF, 0xA3, "LSH R: 0x51, old_c=1 -> res"); // 10100011
        assert_eq!(flags, 0b0000, "LSH R: 0x51, old_c=1 -> flags"); // C=0

        let (res, flags) = Cpu::alu_lsh(ShiftType::R, Wrapping(0x80), 0); // 10000000, old_c=0
        assert_eq!(res & 0xFF, 0x00, "LSH R: 0x80, old_c=0 -> res"); // 00000000
        assert_eq!(flags, 0b1001, "LSH R: 0x80, old_c=0 -> flags"); // Z=1, C=1

        let (res, flags) = Cpu::alu_lsh(ShiftType::R, Wrapping(0x00), 1); // 00000000, old_c=1
        assert_eq!(res & 0xFF, 0x01, "LSH R: 0x00, old_c=1 -> res"); // 00000001
        assert_eq!(flags, 0b0000, "LSH R: 0x00, old_c=1 -> flags"); // C=0

        let (res, flags) = Cpu::alu_lsh(ShiftType::R, Wrapping(0x00), 0); // 00000000, old_c=0
        assert_eq!(res & 0xFF, 0x00, "LSH R: 0x00, old_c=0 -> res"); // 00000000
        assert_eq!(flags, 0b1000, "LSH R: 0x00, old_c=0 -> flags"); // Z=1, C=0

        // Test ShiftType::RC (Rotate Left Circular)
        // res = (val << 1) | ((val & 0x80) >> 7), new_c = (val & 0x80) >> 7
        let (res, flags) = Cpu::alu_lsh(ShiftType::RC, Wrapping(0x51), 0); // 01010001 (old_c ignored)
        assert_eq!(res & 0xFF, 0xA2, "LSH RC: 0x51 -> res"); // 10100010
        assert_eq!(flags, 0b0000, "LSH RC: 0x51 -> flags"); // C=0

        let (res, flags) = Cpu::alu_lsh(ShiftType::RC, Wrapping(0x81), 0); // 10000001
        assert_eq!(res & 0xFF, 0x03, "LSH RC: 0x81 -> res"); // 00000011 (10000001 -> 00000010 | 1 = 00000011)
        assert_eq!(flags, 0b0001, "LSH RC: 0x81 -> flags"); // C=1

        let (res, flags) = Cpu::alu_lsh(ShiftType::RC, Wrapping(0x00), 0); // 00000000
        assert_eq!(res & 0xFF, 0x00, "LSH RC: 0x00 -> res");
        assert_eq!(flags, 0b1000, "LSH RC: 0x00 -> flags"); // Z=1, C=0

        // Test ShiftType::SL (Shift Left Logical/Arithmetic - SLA in Game Boy context)
        // res = val << 1, new_c = (val & 0x80) >> 7
        let (res, flags) = Cpu::alu_lsh(ShiftType::SL, Wrapping(0x51), 0); // 01010001 (old_c ignored)
        assert_eq!(res & 0xFF, 0xA2, "LSH SL: 0x51 -> res"); // 10100010
        assert_eq!(flags, 0b0000, "LSH SL: 0x51 -> flags"); // C=0

        let (res, flags) = Cpu::alu_lsh(ShiftType::SL, Wrapping(0x80), 0); // 10000000
        assert_eq!(res & 0xFF, 0x00, "LSH SL: 0x80 -> res"); // 00000000
        assert_eq!(flags, 0b1001, "LSH SL: 0x80 -> flags"); // Z=1, C=1

        let (res, flags) = Cpu::alu_lsh(ShiftType::SL, Wrapping(0xFF), 0); // 11111111
        assert_eq!(res & 0xFF, 0xFE, "LSH SL: 0xFF -> res"); // 11111110
        assert_eq!(flags, 0b0001, "LSH SL: 0xFF -> flags"); // C=1

        let (res, flags) = Cpu::alu_lsh(ShiftType::SL, Wrapping(0x00), 0); // 00000000
        assert_eq!(res & 0xFF, 0x00, "LSH SL: 0x00 -> res");
        assert_eq!(flags, 0b1000, "LSH SL: 0x00 -> flags"); // Z=1, C=0

        // Test ShiftType::SA (Invalid Shift Left Arithmetic - returns val, flags Z000)
        // res = val.0, c = 0. Flags = Z000 (Z based on val.0 == 0)
        let (res, flags) = Cpu::alu_lsh(ShiftType::SA, Wrapping(0x55), 0); // 01010101, old_c ignored
        assert_eq!(res & 0xFF, 0x55, "LSH SA: 0x55 -> res");
        assert_eq!(flags, 0b0000, "LSH SA: 0x55 -> flags"); // Z=0 (val is 0x55), C=0

        let (res, flags) = Cpu::alu_lsh(ShiftType::SA, Wrapping(0x00), 0); // 00000000
        assert_eq!(res & 0xFF, 0x00, "LSH SA: 0x00 -> res");
        assert_eq!(flags, 0b1000, "LSH SA: 0x00 -> flags"); // Z=1 (val is 0x00), C=0

        let (res, flags) = Cpu::alu_lsh(ShiftType::SA, Wrapping(0x8F), 1); // 10001111, old_c ignored
        assert_eq!(res & 0xFF, 0x8F, "LSH SA: 0x8F -> res");
        assert_eq!(flags, 0b0000, "LSH SA: 0x8F -> flags"); // Z=0 (val is 0x8F), C=0
    }

    #[test]
    fn test_alu_swap() {
        // Flags: Z000 (Z determined by result, N=0, H=0, C=0)

        // Case 1: Basic swap (0xAB -> 0xBA)
        // Result 0xBA is not zero. Z = 0.
        // Expected flags: 0b0000
        let (res, flags) = Cpu::alu_swap(Wrapping(0xAB));
        assert_eq!(res & 0xFF, 0xBA, "SWAP: 0xAB -> 0xBA");
        assert_eq!(flags, 0b0000, "Flags for SWAP 0xAB");

        // Case 2: Swap resulting in zero (0x00 -> 0x00)
        // Result 0x00 is zero. Z = 1.
        // Expected flags: 0b1000
        let (res, flags) = Cpu::alu_swap(Wrapping(0x00));
        assert_eq!(res & 0xFF, 0x00, "SWAP: 0x00 -> 0x00");
        assert_eq!(flags, 0b1000, "Flags for SWAP 0x00");

        // Case 3: Swap with identical nibbles (0xCC -> 0xCC)
        // Result 0xCC is not zero. Z = 0.
        // Expected flags: 0b0000
        let (res, flags) = Cpu::alu_swap(Wrapping(0xCC));
        assert_eq!(res & 0xFF, 0xCC, "SWAP: 0xCC -> 0xCC");
        assert_eq!(flags, 0b0000, "Flags for SWAP 0xCC");

        // Case 4: Swap with one nibble zero (0x0F -> 0xF0)
        // Result 0xF0 is not zero. Z = 0.
        // Expected flags: 0b0000
        let (res, flags) = Cpu::alu_swap(Wrapping(0x0F));
        assert_eq!(res & 0xFF, 0xF0, "SWAP: 0x0F -> 0xF0");
        assert_eq!(flags, 0b0000, "Flags for SWAP 0x0F");

        // Case 5: Swap with other nibble zero (0xA0 -> 0x0A)
        // Result 0x0A is not zero. Z = 0.
        // Expected flags: 0b0000
        let (res, flags) = Cpu::alu_swap(Wrapping(0xA0));
        assert_eq!(res & 0xFF, 0x0A, "SWAP: 0xA0 -> 0x0A");
        assert_eq!(flags, 0b0000, "Flags for SWAP 0xA0");
    }

    #[test]
    fn test_alu_bit() {
        // Flags: Z010 (Z determined by tested bit's value, N=0, H=1, C=0)
        // The first value returned by alu_bit (res_tuple.0) is 1 if bit is 0, 0 if bit is 1.
        // This first value becomes the Z flag.

        // Case 1: Test bit 0 of 0x01 (bit is 1)
        // val=0x01 (...._...1), bit=0. Tested bit is 1.
        // res_val (Z) should be 0. Expected flags: 0b0010.
        let (res_val, flags) = Cpu::alu_bit(Wrapping(0x01), 0);
        assert_eq!(res_val, 0, "BIT 0, 0x01 -> res_val");
        assert_eq!(flags, 0b0010, "Flags for BIT 0, 0x01");

        // Case 2: Test bit 0 of 0xFE (bit is 0)
        // val=0xFE (...._...0), bit=0. Tested bit is 0.
        // res_val (Z) should be 1. Expected flags: 0b1010.
        let (res_val, flags) = Cpu::alu_bit(Wrapping(0xFE), 0);
        assert_eq!(res_val, 1, "BIT 0, 0xFE -> res_val");
        assert_eq!(flags, 0b1010, "Flags for BIT 0, 0xFE");

        // Case 3: Test bit 7 of 0x80 (bit is 1)
        // val=0x80 (1..._....), bit=7. Tested bit is 1.
        // res_val (Z) should be 0. Expected flags: 0b0010.
        let (res_val, flags) = Cpu::alu_bit(Wrapping(0x80), 7);
        assert_eq!(res_val, 0, "BIT 7, 0x80 -> res_val");
        assert_eq!(flags, 0b0010, "Flags for BIT 7, 0x80");

        // Case 4: Test bit 7 of 0x7F (bit is 0)
        // val=0x7F (0..._....), bit=7. Tested bit is 0.
        // res_val (Z) should be 1. Expected flags: 0b1010.
        let (res_val, flags) = Cpu::alu_bit(Wrapping(0x7F), 7);
        assert_eq!(res_val, 1, "BIT 7, 0x7F -> res_val");
        assert_eq!(flags, 0b1010, "Flags for BIT 7, 0x7F");

        // Case 5: Test bit 3 of 0x08 (bit is 1)
        // val=0x08 (...._1...), bit=3. Tested bit is 1.
        // res_val (Z) should be 0. Expected flags: 0b0010.
        let (res_val, flags) = Cpu::alu_bit(Wrapping(0x08), 3);
        assert_eq!(res_val, 0, "BIT 3, 0x08 -> res_val");
        assert_eq!(flags, 0b0010, "Flags for BIT 3, 0x08");

        // Case 6: Test bit 3 of 0xF7 (bit is 0)
        // val=0xF7 (...._0...), bit=3. Tested bit is 0.
        // res_val (Z) should be 1. Expected flags: 0b1010.
        let (res_val, flags) = Cpu::alu_bit(Wrapping(0xF7), 3);
        assert_eq!(res_val, 1, "BIT 3, 0xF7 -> res_val");
        assert_eq!(flags, 0b1010, "Flags for BIT 3, 0xF7");

        // Case 7: Test with val=0x00 for any bit (all bits are 0)
        // val=0x00, bit=4. Tested bit is 0.
        // res_val (Z) should be 1. Expected flags: 0b1010.
        let (res_val, flags) = Cpu::alu_bit(Wrapping(0x00), 4);
        assert_eq!(res_val, 1, "BIT 4, 0x00 -> res_val");
        assert_eq!(flags, 0b1010, "Flags for BIT 4, 0x00");

        // Case 8: Test with val=0xFF for any bit (all bits are 1)
        // val=0xFF, bit=5. Tested bit is 1.
        // res_val (Z) should be 0. Expected flags: 0b0010.
        let (res_val, flags) = Cpu::alu_bit(Wrapping(0xFF), 5);
        assert_eq!(res_val, 0, "BIT 5, 0xFF -> res_val");
        assert_eq!(flags, 0b0010, "Flags for BIT 5, 0xFF");
    }

    #[test]
    fn test_alu_rsb() {
        // Flags: Always 0b0000

        // Resetting bits (set = 0)
        // val=0x01, bit=0, set=0 -> res=0x00
        let (res, flags) = Cpu::alu_rsb(Wrapping(0x01), 0, 0);
        assert_eq!(res & 0xFF, 0x00, "RSB: val=0x01, bit=0, set=0");
        assert_eq!(flags, 0b0000, "Flags for RSB val=0x01, bit=0, set=0");

        // val=0x80, bit=7, set=0 -> res=0x00
        let (res, flags) = Cpu::alu_rsb(Wrapping(0x80), 7, 0);
        assert_eq!(res & 0xFF, 0x00, "RSB: val=0x80, bit=7, set=0");
        assert_eq!(flags, 0b0000, "Flags for RSB val=0x80, bit=7, set=0");

        // val=0x0F, bit=3, set=0 -> res=0x07
        let (res, flags) = Cpu::alu_rsb(Wrapping(0x0F), 3, 0);
        assert_eq!(res & 0xFF, 0x07, "RSB: val=0x0F, bit=3, set=0");
        assert_eq!(flags, 0b0000, "Flags for RSB val=0x0F, bit=3, set=0");

        // val=0x00, bit=0, set=0 -> res=0x00 (no change)
        let (res, flags) = Cpu::alu_rsb(Wrapping(0x00), 0, 0);
        assert_eq!(res & 0xFF, 0x00, "RSB: val=0x00, bit=0, set=0");
        assert_eq!(flags, 0b0000, "Flags for RSB val=0x00, bit=0, set=0");

        // val=0xFF, bit=7, set=0 -> res=0x7F
        let (res, flags) = Cpu::alu_rsb(Wrapping(0xFF), 7, 0);
        assert_eq!(res & 0xFF, 0x7F, "RSB: val=0xFF, bit=7, set=0");
        assert_eq!(flags, 0b0000, "Flags for RSB val=0xFF, bit=7, set=0");

        // Setting bits (set = 1)
        // val=0x00, bit=0, set=1 -> res=0x01
        let (res, flags) = Cpu::alu_rsb(Wrapping(0x00), 0, 1);
        assert_eq!(res & 0xFF, 0x01, "RSB: val=0x00, bit=0, set=1");
        assert_eq!(flags, 0b0000, "Flags for RSB val=0x00, bit=0, set=1");

        // val=0x00, bit=7, set=1 -> res=0x80
        let (res, flags) = Cpu::alu_rsb(Wrapping(0x00), 7, 1);
        assert_eq!(res & 0xFF, 0x80, "RSB: val=0x00, bit=7, set=1");
        assert_eq!(flags, 0b0000, "Flags for RSB val=0x00, bit=7, set=1");

        // val=0xF0, bit=3, set=1 -> res=0xF8
        let (res, flags) = Cpu::alu_rsb(Wrapping(0xF0), 3, 1);
        assert_eq!(res & 0xFF, 0xF8, "RSB: val=0xF0, bit=3, set=1");
        assert_eq!(flags, 0b0000, "Flags for RSB val=0xF0, bit=3, set=1");

        // val=0x01, bit=0, set=1 -> res=0x01 (no change)
        let (res, flags) = Cpu::alu_rsb(Wrapping(0x01), 0, 1);
        assert_eq!(res & 0xFF, 0x01, "RSB: val=0x01, bit=0, set=1");
        assert_eq!(flags, 0b0000, "Flags for RSB val=0x01, bit=0, set=1");

        // val=0xFF, bit=7, set=1 -> res=0xFF (no change)
        let (res, flags) = Cpu::alu_rsb(Wrapping(0xFF), 7, 1);
        assert_eq!(res & 0xFF, 0xFF, "RSB: val=0xFF, bit=7, set=1");
        assert_eq!(flags, 0b0000, "Flags for RSB val=0xFF, bit=7, set=1");
    }
}