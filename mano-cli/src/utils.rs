use mano_lib::message::{Messages, Level};
use mano_lib::machine::MachineState;

pub fn print_messages(messages: &Messages, verbose: bool) {
    for (level, msg) in &messages.entries {
        match level {
            Level::Info => println!("{}", msg),
            Level::Error => println!("ERR: {}", msg),
            Level::Debug => if verbose {
                println!("DBG: {}", msg)
            },
        }
    }
}

pub fn print_source_program(program: &[String]) {
    println!("\n=== Source Program ===");
    for (i, line) in program.iter().enumerate() {
        println!("{:4}: {}", i + 1, line);
    }
    println!();
}

pub fn print_assembled_program(program: &[String]) {
    println!("\n=== Assembled Program ===");
    if program.is_empty() {
        println!("(empty)");
    } else {
        for (addr, instruction) in program.iter().enumerate() {
            if !instruction.is_empty() {
                println!("[{:04X}]: {}", addr, instruction);
            }
        }
    }
    println!();
}

pub fn print_machine_state(state: &MachineState) {
    println!("\n=== Final Machine State ===");
    println!("┌─────────────────────────────────────┐");
    println!("│ CPU Registers                       │");
    println!("├─────────────────────────────────────┤");
    println!("│ PC (Program Counter)    : 0x{:04X}    │", state.program_counter);
    println!("│ AC (Accumulator)        : 0x{:04X}    │", state.accumulator);
    println!("│ IR (Instruction Reg)    : 0x{:04X}    │", state.instruction_register);
    println!("│ AR (Address Register)   : 0x{:04X}    │", state.address_register);
    println!("│ DR (Data Register)      : 0x{:04X}    │", state.data_register);
    println!("│ E  (Extend Register)    : 0x{:04X}    │", state.extend_register);
    println!("│ SC (Sequence Counter)   : {:4}      │", state.sequence_counter);
    println!("├─────────────────────────────────────┤");
    println!("│ Status: {:27} │", if state.is_halted { " HALTED" } else { "RUNNING" });
    println!("└─────────────────────────────────────┘");

    // Memory hex dump
    println!("\n=== Memory Contents ===");
    println!("Address   +0   +1   +2   +3   +4   +5   +6   +7   +8   +9   +A   +B   +C   +D   +E   +F");
    println!("────────────────────────────────────────────────────────────────────────────────────────");

    let memory = &state.memory_snapshot;
    let total_lines = (memory.len() + 15) / 16; // Round up to next 16

    for line in 0..total_lines {
        let base_addr = line * 16;
        print!("{:04X}:    ", base_addr);

        for offset in 0..16 {
            let addr = base_addr + offset;
            if addr < memory.len() {
                print!(" {:04X}", memory[addr]);
            } else {
                print!("     ");
            }
        }
        println!();
    }
    println!();
}
