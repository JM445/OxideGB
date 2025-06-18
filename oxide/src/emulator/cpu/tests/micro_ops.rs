
#[cfg(test)]
mod tests {
    use crate::emulator::cpu::*;
    
    use std::num::Wrapping;
    #[test]
    fn test_alu_add() {
        // Case 1: 10 + 5, no carry in. Result 15.
        // Expected Flags: Z=0, N=0, H=0, C=0.
        // Expected Flags byte: 0x0.
        let (res, flags) = Cpu::alu_add(Wrapping(10), Wrapping(5), Wrapping(0));
        assert_eq!(res, 15);
        assert_eq!(flags, 0b0000, "Flags for 10+5+0");

        // Case 2: 0x0F + 0x01, no carry in. Result 0x10. (Half-carry)
        // Expected Flags: Z=0, N=0, H=1, C=0.
        // Expected flags byte: 0x2. 
        let (res, flags) = Cpu::alu_add(Wrapping(0x0F), Wrapping(0x01), Wrapping(0));
        assert_eq!(res, 0x10);
        assert_eq!(flags, 0b0010, "Flags for 0x0F+0x01+0");

        // Case 3: 0xFF + 0x01, no carry in. Result 0x00
        // Expected Flags: Z=1, N=0, H=1, C=1.
        // Expected flags byte: 0xB.
        let (res3, flags3) = Cpu::alu_add(Wrapping(0xFF), Wrapping(0x01), Wrapping(0));
        assert_eq!(res3 & 0xFF, 0x00); // Check 8-bit result
        assert_eq!(flags3, 0b1101, "Flags for 0xFF+0x01+0.");

        // Case 4: 10 + 5 + CARRY(1). Result 16.
        // Expected Flags: Z=0, N=0, H=1, C=0.
        // Expected flags byte: 0x2.
        let (res4, flags4) = Cpu::alu_add(Wrapping(10), Wrapping(5), Wrapping(1));
        assert_eq!(res4, 16);
        assert_eq!(flags4, 0b0010, "Flags for 10+5+1");
    }
}