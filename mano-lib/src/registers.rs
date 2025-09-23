pub struct Register {
    value: u16,
    mask: u16,
}

impl Register {
    pub fn new(bits: u8) -> Self {
        let mask = if bits >= 16 { 0xFFFF } else { (1 << bits) - 1 };
        Self { value: 0, mask }
    }
    
    pub fn get(&self) -> u16 {
        self.value
    }
    
    pub fn set(&mut self, value: u16) {
        self.value = value & self.mask;
    }
    
    pub fn clear(&mut self) {
        self.value = 0;
    }
    
    pub fn increment(&mut self) {
        self.set(self.value + 1);
    }
    
    pub fn logic_and(&mut self, value: u16) {
        self.set(self.value & value);
    }
    
    pub fn add(&mut self, value: u16) -> u16 {
        let sum = self.value as u32 + value as u32;
        let carry = if sum > self.mask as u32 { 1 } else { 0 };
        self.set(sum as u16);
        carry
    }
    
    pub fn complement(&mut self) {
        self.set(!self.value);
    }
    
    pub fn shift_right(&mut self, msb: u16) -> u16 {
        let lsb = self.value & 1;
        self.set(self.value >> 1);
        if msb != 0 {
            let msb_mask = if self.mask == 0xFFFF { 0x8000 } else { (self.mask + 1) >> 1 };
            self.set(self.value | msb_mask);
        }
        lsb
    }
    
    pub fn shift_left(&mut self, lsb: u16) -> u16 {
        let msb_mask = if self.mask == 0xFFFF { 0x8000 } else { (self.mask + 1) >> 1 };
        let msb = if self.value & msb_mask != 0 { 1 } else { 0 };
        self.set((self.value << 1) | (lsb & 1));
        msb
    }
}

pub struct Registers {
    pub ar: Register,     // Address register (12-bit)
    pub pc: Register,     // Program counter (12-bit)  
    pub dr: Register,     // Data register (16-bit)
    pub ac: Register,     // Accumulator (16-bit)
    pub ir: Register,     // Instruction register (16-bit)
    pub tr: Register,     // Temporary register (16-bit)
    pub inpr: Register,   // Input register (8-bit)
    pub outr: Register,   // Output register (8-bit)
    pub sc: Register,     // Sequence counter (3-bit)
    pub e: Register,      // Carry bit (1-bit)
    pub s: Register,      // Start/stop (1-bit)
    pub r: Register,      // Interrupt raised (1-bit)
    pub ien: Register,    // Interrupt enable (1-bit)
    pub fgi: Register,    // Input available (1-bit)
    pub fgo: Register,    // Output available (1-bit)
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ar: Register::new(12),
            pc: Register::new(12),
            dr: Register::new(16),
            ac: Register::new(16),
            ir: Register::new(16),
            tr: Register::new(16),
            inpr: Register::new(8),
            outr: Register::new(8),
            sc: Register::new(3),
            e: Register::new(1),
            s: Register::new(1),
            r: Register::new(1),
            ien: Register::new(1),
            fgi: Register::new(1),
            fgo: Register::new(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_overflow_handling() {
        let mut reg = Register::new(3); // 3-bit register, max value = 7
        
        // Test normal set
        reg.set(5);
        assert_eq!(reg.get(), 5);
        
        // Test overflow - should wrap around
        reg.set(10); // 10 & 0b111 = 2
        assert_eq!(reg.get(), 2);
        
        // Test increment with overflow
        reg.set(7); // Max value for 3-bit
        reg.increment(); // Should wrap to 0
        assert_eq!(reg.get(), 0);
    }

    #[test]
    fn test_register_operations() {
        let mut reg = Register::new(16); // Full 16-bit register
        
        reg.set(0xAAAA);
        assert_eq!(reg.get(), 0xAAAA);
        
        // Test complement
        reg.complement();
        assert_eq!(reg.get(), 0x5555);
        
        // Test logic AND
        reg.logic_and(0x0F0F);
        assert_eq!(reg.get(), 0x0505);
    }

    #[test]
    fn test_register_carry() {
        let mut reg = Register::new(4); // 4-bit register, max = 15
        
        reg.set(10);
        let carry = reg.add(3); // 10 + 3 = 13, no carry for 4-bit
        assert_eq!(reg.get(), 13);
        assert_eq!(carry, 0);
        
        reg.set(14);
        let carry = reg.add(5); // 14 + 5 = 19, should overflow and carry
        assert_eq!(reg.get(), 3); // 19 & 0xF = 3
        assert_eq!(carry, 1);
    }

    #[test]
    fn test_registers_initialization() {
        let registers = Registers::new();
        
        // Test that all registers are initialized to 0
        assert_eq!(registers.ar.get(), 0);
        assert_eq!(registers.pc.get(), 0);
        assert_eq!(registers.ac.get(), 0);
        assert_eq!(registers.e.get(), 0);
        
        // Test that bit limits are correct
        // AR is 12-bit, so max should be 4095 (0xFFF)
        // We can't directly access mask, but we can test overflow behavior
    }
}