use std::iter::Map;
use crate::message::Messages;
use crate::registers::Registers;

pub struct CPU {
    pub reg: Registers,
    pub memory: [u16; 4096],
}

pub struct CPUState {
    pub registers: Map<String, u16>,
    pub memory: [u16; 4096],
}

impl CPU {
    pub fn new() -> Self {
        Self {
            reg: Registers::new(),
            memory: [0; 4096],
        }
    }

    pub fn load_program_into_memory(&mut self, assembled_program: &Vec<String>) -> Messages {
        let mut messages = Messages::new();
        for (address, instruction_str) in assembled_program.iter().enumerate() {
            if let Ok(instruction) = u16::from_str_radix(instruction_str, 16) {
                if address < self.memory.len() {
                    self.memory[address] = instruction;
                    messages.debug(&format!("Loaded {} into memory[{}]", instruction_str, address));
                }
            } else {
                messages.error(&format!("Invalid instruction format: {}", instruction_str));
            }
        }

        messages
    }

    pub fn set_program_start(&mut self, start: u16) -> Messages {
        let mut messages = Messages::new();
        self.reg.pc.set(start);
        messages.info(&format!("Program will start at address {start}"));
        messages
    }

    pub fn reset(&mut self, assembled_program: &Vec<String>) {
        self.set_program_start(0);
        self.reg.s.set(1);
        self.reg.ir.set(1);
        self.load_program_into_memory(assembled_program);
    }

    // Main CPU tick function - Figure 5-15 chapter 5 page 158 (169 in reader)
    pub fn tick(&mut self, messages: &mut Messages) {
        let tick = self.reg.sc.get();
        let interrupt_raised = self.reg.r.get();
        let opcode = self.get_opcode();    // Bits 12-14
        let i_bit = self.get_indirect_bit(); // Bit 15 (indirect bit)

        if tick < 3 && interrupt_raised == 1 {
            self.interrupt(tick, messages);
        } else if tick < 2 && interrupt_raised == 0 {
            self.instruction_fetch(tick, messages);
        } else if tick == 2 && interrupt_raised == 0 {
            self.instruction_decode(messages);
        } else if tick == 3 && opcode != 7 {
            self.fetch_operand(i_bit, messages);
        } else if tick > 3 && opcode != 7 {
            self.execute_mri(opcode, tick, messages);
        } else if tick == 3 && opcode == 7 {
            let instruction = self.get_instruction_bit();
            if i_bit == 1 {
                self.execute_io(instruction, messages);
            } else {
                self.execute_rri(instruction, messages);
            }
        }
    }

    // Returns bits 12-14 in IR
    fn get_opcode(&self) -> u16 {
        (self.reg.ir.get() >> 12) & 7
    }

    // Returns bit 15 in IR
    fn get_indirect_bit(&self) -> u16 {
        (self.reg.ir.get() >> 15) & 1
    }

    // Returns index of "1" bit in bits 0-11 that specifies the operation for
    // register reference and input / output instructions
    fn get_instruction_bit(&self) -> u8 {
        let ir_value = self.reg.ir.get();
        // Convert to binary string and reverse it to find first '1' bit
        let ir_string = format!("{:016b}", ir_value);
        let reversed: String = ir_string.chars().rev().collect();

        // Find the index of the first '1' in the reversed string (bits 0-11)
        if let Some(index) = reversed.chars().take(12).position(|c| c == '1') {
            index as u8
        } else {
            0
        }
    }

    fn interrupt(&mut self, tick: u16, messages: &mut Messages) {
        if tick == 0 {
            // "RT0: AR <- 0, TR <- PC"
            messages.debug("INTERRUPT RT0 : AR <- 0, TR <- PC");
            self.reg.ar.clear();
            self.reg.tr.set(self.reg.pc.get());
            self.reg.sc.increment();
        } else if tick == 1 {
            // "RT1: M[AR] <- TR, PC <- 0"
            messages.debug("INTERRUPT RT1 : M[AR] <- TR, PC <- 0");
            self.write_to_memory(self.reg.tr.get());
            self.reg.pc.clear();
            self.reg.sc.increment();
        } else if tick == 2 {
            // "RT2: PC <- PC + 1, IEN <- 0, R <- 0, SC <- 0"
            messages.debug("INTERRUPT RT2 : PC <- PC + 1, IEN <- 0, R <- 0, SC <- 0");
            self.reg.pc.increment();
            self.reg.ien.clear();
            self.reg.r.clear();
            self.reg.sc.clear();
        }
    }

    fn instruction_fetch(&mut self, tick: u16, messages: &mut Messages) {
        if tick == 0 {
            // "R'T0: AR <- PC"
            messages.debug("FETCH R'T0 : AR <- PC");
            self.reg.ar.set(self.reg.pc.get());
            self.reg.sc.increment();
        } else if tick == 1 {
            // "R'T1: IR <- M[AR], PC <- PC + 1"
            messages.debug("FETCH R'T1 : IR <- M[AR], PC <- PC + 1");
            self.reg.ir.set(self.read_from_memory());
            self.reg.pc.increment();
            self.reg.sc.increment();
        }
    }

    fn instruction_decode(&mut self, messages: &mut Messages) {
        // "R'T2: AR <- IR(0-11)"
        messages.debug("DECODE R'T2 : AR <- IR(0-11)");
        let address_bits = self.reg.ir.get() & 0x0FFF; // Extract bits 0-11
        self.reg.ar.set(address_bits);
        self.reg.sc.increment();
    }

    fn fetch_operand(&mut self, i_bit: u16, messages: &mut Messages) {
        // For when the operand is indirect
        if i_bit == 1 {
            // "D7'IT3: AR <- M[AR]"
            messages.debug("INDIRECT D7'IT3 : AR <- M[AR]");
            self.reg.ar.set(self.read_from_memory());
        }
        // else: "D7'I'T3: NOOP" // No operation

        self.reg.sc.increment();
    }

    // Memory access helper functions
    pub fn read_from_memory(&self) -> u16 {
        let address = self.reg.ar.get() as usize;
        if address < self.memory.len() {
            self.memory[address]
        } else {
            0
        }
    }

    pub fn write_to_memory(&mut self, value: u16) {
        let address = self.reg.ar.get() as usize;
        if address < self.memory.len() {
            self.memory[address] = value;
        }
    }

    fn execute_mri(&mut self, opcode: u16, tick: u16, messages: &mut Messages) {
        match opcode {
            0 => self.and(messages, tick),
            1 => self.add(messages, tick),
            2 => self.lda(messages, tick),
            3 => self.sta(messages),
            4 => self.bun(messages),
            5 => self.bsa(messages, tick),
            6 => self.isz(messages, tick),
            _ => {
                messages.error(&format!("Unknown MRI opcode: {}", opcode));
            }
        }
    }

    fn execute_rri(&mut self, instruction: u8, messages: &mut Messages) {
        match instruction {
            11 => self.cla(messages),
            10 => self.cle(messages),
            9 => self.cma(messages),
            8 => self.cme(messages),
            7 => self.cir(messages),
            6 => self.cil(messages),
            5 => self.inc(messages),
            4 => self.spa(messages),
            3 => self.sna(messages),
            2 => self.sza(messages),
            1 => self.sze(messages),
            0 => self.hlt(messages),
            _ => {
                messages.error(&format!("Unknown RRI instruction: {}", instruction));
            }
        }

        self.reg.sc.clear();
    }

    fn execute_io(&mut self, instruction: u8, messages: &mut Messages) {
        match instruction {
            11 => self.inp(messages),
            10 => self.out(messages),
            9 => self.ski(messages),
            8 => self.sko(messages),
            7 => self.ion(messages),
            6 => self.iof(messages),
            _ => {
                messages.error(&format!("Unknown IO instruction: {}", instruction));
            }
        }

        self.reg.sc.clear();
    }

    // MRI (Memory Reference Instructions)
    // Figure 5-11 chapter 5 page 150 (161 in reader)

    pub fn and(&mut self, messages: &mut Messages, tick: u16) {
        if tick == 4 {
            messages.debug("AND D0T4 DR <- M[AR]");
            let memory_value = self.read_from_memory();
            self.reg.dr.set(memory_value);
            self.reg.sc.increment();
        } else if tick == 5 {
            messages.debug("AND D0T5 : AC <- AC & DR, SC <- 0");
            self.reg.ac.logic_and(self.reg.dr.get());
            self.reg.sc.clear();
        }
    }

    pub fn add(&mut self, messages: &mut Messages, tick: u16) {
        if tick == 4 {
            messages.debug("ADD D1T4 : DR <- M[AR]");
            let memory_value = self.read_from_memory();
            self.reg.dr.set(memory_value);
            self.reg.sc.increment();
        } else if tick == 5 {
            messages.debug("ADD D1T5 : AC <- AC + DR, E <- Cout, SC <- 0");
            let carry = self.reg.ac.add(self.reg.dr.get());
            self.reg.e.set(carry);
            self.reg.sc.clear();
        }
    }

    pub fn lda(&mut self, messages: &mut Messages, tick: u16) {
        if tick == 4 {
            messages.debug("LDA D2T4 : DR <- M[AR]");
            let memory_value = self.read_from_memory();
            self.reg.dr.set(memory_value);
            self.reg.sc.increment();
        } else if tick == 5 {
            messages.debug("LDA D2T5 : AC <- DR, SC <- 0");
            self.reg.ac.set(self.reg.dr.get());
            self.reg.sc.clear();
        }
    }

    pub fn sta(&mut self, messages: &mut Messages) {
        messages.debug("STA D3T4 : M[AR] <- AC, SC <- 0");
        self.write_to_memory(self.reg.ac.get());
        self.reg.sc.clear();
    }

    pub fn bun(&mut self, messages: &mut Messages) {
        messages.debug("BUN D4T4 :PC <- AR, SC <- 0");
        self.reg.pc.set(self.reg.ar.get());
        self.reg.sc.clear();
    }

    pub fn bsa(&mut self, messages: &mut Messages, tick: u16) {
        if tick == 4 {
            messages.debug("BSA D5T4 : M[AR] <- PC, AR <- AR + 1");
            self.write_to_memory(self.reg.pc.get());
            self.reg.ar.increment();
            self.reg.sc.increment();
        } else if tick == 5 {
            messages.debug("BSA D5T5 :PC <- AR, SC <- 0");
            self.reg.pc.set(self.reg.ar.get());
            self.reg.sc.clear();
        }
    }

    pub fn isz(&mut self, messages: &mut Messages, tick: u16) {
        if tick == 4 {
            messages.debug("ISZ D6T4 : DR <- M[AR]");
            let memory_value = self.read_from_memory();
            self.reg.dr.set(memory_value);
            self.reg.sc.increment();
        } else if tick == 5 {
            messages.debug("ISZ D6T5 : DR <- DR + 1");
            self.reg.dr.increment();
            self.reg.sc.increment();
        } else if tick == 6 {
            messages.debug("ISZ D6T6 : M[AR] <- DR, if (DR = 0) then (PC <- PC + 1), SC <- 0");
            self.write_to_memory(self.reg.dr.get());
            if self.reg.dr.get() == 0 {
                self.reg.pc.increment();
            }
            self.reg.sc.clear();
        }
    }

    // RRI (Register Reference Instructions)
    // Table 5-6 Chapter 5 page 159 (170 in reader)

    pub fn cla(&mut self, messages: &mut Messages) {
        messages.debug("CLA D7I'T3rB11 : AC <- 0, SC <- 0");
        self.reg.ac.clear();
    }

    pub fn cle(&mut self, messages: &mut Messages) {
        messages.debug("CLE D7I'T3rB10 : E <- 0, SC <- 0");
        self.reg.e.clear();
    }

    pub fn cma(&mut self, messages: &mut Messages) {
        messages.debug("CMA D7I'T3rB9 : AC <- AC', SC <- 0");
        self.reg.ac.complement();
    }

    pub fn cme(&mut self, messages: &mut Messages) {
        messages.debug("CME D7I'T3rB8 : E <- E', SC <- 0");
        self.reg.e.complement();
    }

    pub fn cir(&mut self, messages: &mut Messages) {
        messages.debug("CIR D7I'T3rB7 : AC <- shr AC, AC(15) <- E, E <- AC(0), SC <- 0");
        let new_e = self.reg.ac.shift_right(self.reg.e.get());
        self.reg.e.set(new_e);
    }

    pub fn cil(&mut self, messages: &mut Messages) {
        messages.debug("CIL D7I'T3rB6 : AC <- shl AC, AC(0) <- E, E <- AC(15), SC <- 0");
        let new_e = self.reg.ac.shift_left(self.reg.e.get());
        self.reg.e.set(new_e);
    }

    pub fn inc(&mut self, messages: &mut Messages) {
        messages.debug("INC D7I'T3rB5 : AC <- AC + 1, SC <- 0");
        self.reg.ac.increment();
    }

    pub fn spa(&mut self, messages: &mut Messages) {
        messages.debug("SPA D7I'T3rB4 : if (AC(15) = 0) then (PC <- PC + 1), SC <- 0");
        // 0x8000 is (binary) 1000 0000 0000 0000. This will determine if the first bit is 0 or not
        if (self.reg.ac.get() & 0x8000) == 0 {
            self.reg.pc.increment();
        }
    }

    pub fn sna(&mut self, messages: &mut Messages) {
        messages.debug("SNA D7I'T3rB3 : if (AC(15) = 1) then (PC <- PC + 1), SC <- 0");
        // 0x8000 is (binary) 1000 0000 0000 0000. This will determine if the first bit is 1 or not
        // Note: Fixed the bug from C# version - this should check if bit 15 is 1, not 0
        if (self.reg.ac.get() & 0x8000) != 0 {
            self.reg.pc.increment();
        }
    }

    pub fn sza(&mut self, messages: &mut Messages) {
        messages.debug("SZA D7I'T3rB2 : if (AC = 0) then (PC <- PC + 1), SC <- 0");
        if self.reg.ac.get() == 0 {
            self.reg.pc.increment();
        }
    }

    pub fn sze(&mut self, messages: &mut Messages) {
        messages.debug("SZE D7I'T3rB1 : if (E = 0) then (PC <- PC + 1), SC <- 0");
        if self.reg.e.get() == 0 {
            self.reg.pc.increment();
        }
    }

    pub fn hlt(&mut self, messages: &mut Messages) {
        messages.debug("HLT D7I'T3rB0 : S <- 0, SC <- 0");
        messages.error("Halting");
        self.reg.s.set(0);
    }

    // IO (Input/Output Instructions)
    // Table 5-6 Chapter 5 page 159 (170 in reader)

    pub fn inp(&mut self, messages: &mut Messages) {
        messages.debug("INP :");
        // Implementation will be added when I/O system is implemented
    }

    pub fn out(&mut self, messages: &mut Messages) {
        messages.debug("OUT :");
        // Implementation will be added when I/O system is implemented
    }

    pub fn ski(&mut self, messages: &mut Messages) {
        messages.debug("SKI :");
        // Implementation will be added when I/O system is implemented
    }

    pub fn sko(&mut self, messages: &mut Messages) {
        messages.debug("SKO :");
        // Implementation will be added when I/O system is implemented
    }

    pub fn ion(&mut self, messages: &mut Messages) {
        messages.debug("ION D7IT3pB7 : IEN <- 1, SC <- 0");
        self.reg.ien.set(1);
    }

    pub fn iof(&mut self, messages: &mut Messages) {
        messages.debug("IOF D7IT3pB6 : IEN <- 0, SC <- 0");
        self.reg.ien.set(0);
    }
}