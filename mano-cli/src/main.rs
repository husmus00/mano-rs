use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use clap::Parser;
use anyhow::Result;
use mano_lib::machine::Machine;
use mano_lib::message::Messages;

mod utils;
use utils::{print_messages, print_source_program, print_assembled_program, print_machine_state};

#[derive(Parser)]
#[command(name = "mano")]
#[command(about = "Mano Machine Simulator")]
struct Cli {
    /// Assembly file to run
    file: String,

    /// Show verbose output (including debug messages)
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Read the assembly program
    let program = read_file(&cli.file)?;

    // Print source program
    print_source_program(&program);

    // Create machine and prime it
    let mut machine = Machine::new();
    let messages = machine.prime(program);

    // Print assembly messages
    print_messages(&messages, cli.verbose);

    // If there were errors, exit
    if messages.has_errors() {
        println!("\nAssembly failed. Exiting.");
        return Ok(());
    }

    // Print assembled program
    print_assembled_program(machine.get_assembled_program());

    // Run the program
    println!("=== Running Program ===\n");

    let mut step_count = 0;
    let max_steps = 10000;

    loop {
        let mut messages = Messages::new();
        machine.tick(&mut messages);

        // Print messages from this tick
        print_messages(&messages, cli.verbose);

        // Check for errors or halt
        if messages.has_errors() {
            println!("\nExecution stopped due to error after {} steps.", step_count + 1);
            break;
        }

        if machine.is_halted() {
            println!("\nProgram halted after {} steps.", step_count + 1);
            break;
        }

        step_count += 1;

        // Safety check to prevent infinite loops
        if step_count >= max_steps {
            println!("\nProgram exceeded {} steps. Stopping to prevent infinite loop.", max_steps);
            break;
        }
    }

    // Print final machine state
    let state = machine.get_state();
    print_machine_state(&state);

    Ok(())
}

fn read_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    let lines: Result<Vec<String>, _> = buf.lines().collect();
    Ok(lines?)
}
