use crate::message::Messages;
use crate::assembler;
use crate::cpu::CPU;
use crate::storage::Storage;

pub enum Command {
    Assemble,
    Tick,
    Memory(u16),
    Clear
}

pub struct Machine {
    storage: Storage,
    cpu: CPU,
}


// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct MachineState {
    pub program_counter: u16,
    pub accumulator: u16,
    pub instruction_register: u16,
    pub address_register: u16,
    pub data_register: u16,
    pub extend_register: u16,
    pub sequence_counter: u16,
    pub is_halted: bool,
    pub is_running: bool,
    pub memory_snapshot: Vec<u16>,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            storage: Storage::new(),
            cpu: CPU::new()
        }
    }

    // pub fn process_command(&mut self, command: Command) -> Messages {
    //     match command {
    //         Command::Assemble     => self.assemble(),
    //         Command::Tick         => self.tick(),
    //         Command::Memory(addr) => Messages::new(), // self.get_memory_at_address(addr),
    //         Command::Clear        => self.clear_program(),
    //     }
    // }

    // Public interface

    pub fn prime(&mut self, program: Vec<String>) -> Messages{
        let mut messages = Messages::new();

        self.load_program(program, &mut messages);
        self.assemble(&mut messages);
        self.reset(&mut messages);

        messages
    }

    pub fn tick(&mut self, messages: &mut Messages) {
        if self.is_halted() {
            messages.error("Machine halted, please reset");
            return;
        }
        self.cpu.tick(messages);
    }

    pub fn get_state(&self) -> MachineState {
        MachineState {
            program_counter: self.cpu.reg.pc.get(),
            accumulator: self.cpu.reg.ac.get(),
            instruction_register: self.cpu.reg.ir.get(),
            address_register: self.cpu.reg.ar.get(),
            data_register: self.cpu.reg.dr.get(),
            extend_register: self.cpu.reg.e.get(),
            sequence_counter: self.cpu.reg.sc.get(),
            is_halted: self.cpu.reg.s.get() == 0,
            is_running: !self.is_halted(),
            memory_snapshot: self.cpu.memory[0..32].to_vec(), // First 32 words for display
        }
    }

    pub fn is_halted(&self) -> bool {
        self.cpu.reg.s.get() == 0
    }

    pub fn is_primed(&self) -> bool {
        // TODO: Implement proper primed detection
        // Is there a point to this function?

        false
    }

    pub fn get_source_program(&self) -> &[String] {
        &self.storage.source_program
    }

    pub fn get_assembled_program(&self) -> &[String] {
        &self.storage.assembled_program
    }

    // Private functions

    fn load_program(&mut self, program: Vec<String>, messages: &mut Messages) {
        self.storage.load_program(program, messages);
    }

    fn assemble(&mut self, messages: &mut Messages) {
        assembler::assemble(&mut self.storage, messages);
    }

    fn reset(&mut self, messages: &mut Messages) {
        self.cpu.reset(&self.storage.assembled_program);
        messages.info("Machine reset");
    }

    // pub fn get_memory_at_address(&self, address: u16) -> u16 {
    //     let value = if (address as usize) < self.cpu.memory.len() {
    //         self.cpu.memory[address as usize]
    //     } else {
    //         0
    //     };
    //     value
    // }
    //
    // pub fn load_memory_range(&self, start: u16, count: u16) -> Vec<u16> {
    //     let start = start as usize;
    //     let end = (start + count as usize).min(self.cpu.memory.len());
    //     self.cpu.memory[start..end].to_vec()
    // }
}