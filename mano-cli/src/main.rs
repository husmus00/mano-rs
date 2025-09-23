use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use clap::{Parser, Subcommand};
use anyhow::Result;
use mano_lib::machine::Machine;
use mano_lib::utils::print_messages;

#[derive(Parser)]
#[command(name = "mano")]
#[command(about = "Mano Machine Simulator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a program file
    Run {
        /// Assembly file to run
        file: String,
        /// Show debug output
        #[arg(short, long)]
        debug: bool,
    },
    /// Assemble a program file without running
    Assemble {
        /// Assembly file to assemble
        file: String,
    },
    /// Step through program execution
    Debug {
        /// Assembly file to debug
        file: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file, debug } => run_program(&file, debug)?,
        Commands::Assemble { file } => assemble_program(&file)?,
        Commands::Debug { file } => debug_program(&file)?,
    }

    Ok(())
}

fn run_program(filename: &str, show_debug: bool) -> Result<()> {
    let mut machine = Machine::new();
    let program = read_file(filename)?;

    println!("Loading and assembling program: {}", filename);
    let result = machine.prime(program);

    if show_debug {
        print_messages(&result);
    }

    if result.has_errors() {
        println!("Assembly failed. Errors:");
        for (level, msg) in &result.entries {
            if matches!(level, mano_lib::message::Level::Error) {
                println!("  ERROR: {}", msg);
            }
        }
        return Ok(());
    }

    println!("Assembly completed successfully. Running program...");
    println!();

    let mut step_count = 0;
    loop {
        let result = machine.tick();

        if show_debug {
            println!("=== Step {} ===", step_count + 1);
            print_messages(&result);
            let state = machine.get_state();
            println!("PC: 0x{:04X} | AC: 0x{:04X} | IR: 0x{:04X} | AR: 0x{:04X}",
                     state.program_counter, state.accumulator,
                     state.instruction_register, state.address_register);
            println!();
        }

        if result.has_errors() || machine.is_halted() {
            if machine.is_halted() {
                println!("Program halted successfully after {} steps.", step_count + 1);
            } else {
                println!("Execution stopped due to error after {} steps.", step_count + 1);
                for (level, msg) in &result.entries {
                    if matches!(level, mano_lib::message::Level::Error) {
                        println!("  ERROR: {}", msg);
                    }
                }
            }
            break;
        }

        step_count += 1;
        if step_count > 10000 {
            println!("Program exceeded 10000 steps, stopping to prevent infinite loop.");
            break;
        }
    }

    Ok(())
}

fn assemble_program(filename: &str) -> Result<()> {
    let mut machine = Machine::new();
    let program = read_file(filename)?;

    println!("Assembling program: {}", filename);
    let result = machine.prime(program);
    print_messages(&result);

    if result.has_errors() {
        println!("Assembly failed.");
    } else {
        println!("Assembly completed successfully.");
        println!("\nGenerated machine code:");
        for (addr, instruction) in machine.load_memory_range(0, 16).iter().enumerate() {
            if *instruction != 0 {
                println!("  [{:04X}]: {:04X}", addr, instruction);
            }
        }
    }

    Ok(())
}

fn debug_program(filename: &str) -> Result<()> {
    use std::io::{self, Write};

    let mut machine = Machine::new();
    let program = read_file(filename)?;

    println!("Loading program for debugging: {}", filename);
    let result = machine.prime(program);
    print_messages(&result);

    if result.has_errors() {
        println!("Assembly failed, cannot debug.");
        return Ok(());
    }

    println!("Program loaded successfully. Debug mode active.");
    println!("Commands: [s]tep, [r]un, [m]emory <addr>, [q]uit");
    println!();

    loop {
        let state = machine.get_state();
        println!("PC: 0x{:04X} | AC: 0x{:04X} | IR: 0x{:04X} | AR: 0x{:04X} | SC: {}",
                 state.program_counter, state.accumulator,
                 state.instruction_register, state.address_register,
                 state.sequence_counter);

        if machine.is_halted() {
            println!("Program has halted.");
            break;
        }

        print!("debug> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        match input {
            "s" | "step" => {
                let result = machine.tick();
                print_messages(&result);
                if result.has_errors() {
                    println!("Execution error occurred.");
                    break;
                }
            },
            "r" | "run" => {
                println!("Running to completion...");
                let mut steps = 0;
                loop {
                    let result = machine.tick();
                    if result.has_errors() || machine.is_halted() {
                        print_messages(&result);
                        break;
                    }
                    steps += 1;
                    if steps > 10000 {
                        println!("Program exceeded 10000 steps, stopping.");
                        break;
                    }
                }
            },
            cmd if cmd.starts_with("m ") || cmd.starts_with("memory ") => {
                let parts: Vec<&str> = cmd.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(addr) = u16::from_str_radix(parts[1], 16) {
                        let value = machine.get_memory_at_address(addr);
                        println!("Memory[0x{:04X}] = 0x{:04X} ({})", addr, value, value);
                    } else {
                        println!("Invalid address format. Use hex format (e.g., 1A)");
                    }
                } else {
                    println!("Usage: memory <hex_address>");
                }
            },
            "q" | "quit" => break,
            "" => continue,
            _ => println!("Unknown command. Use: [s]tep, [r]un, [m]emory <addr>, [q]uit"),
        }
        println!();
    }

    Ok(())
}

fn read_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    let lines: Result<Vec<String>, _> = buf.lines().collect();
    Ok(lines?)
}