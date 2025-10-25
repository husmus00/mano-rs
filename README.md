# Mano Machine Emulator

A Rust library and emulator for the Mano Machine RISC CPU

This project consists of a library (`mano-lib`) for constructing Mano Machine simulators, and multiple frontends.

# The Mano Machine

The Mano Machine is a hypothetical RISC CPU designed by M. Morris Mano which appears in the 3rd edition of his book [Computer System Architecture](https://www.amazon.com/Computer-System-Architecture-Morris-Mano/dp/0131755633). Oddly enough, I couldn't find any reference to the 1st or 2nd editions of his book. You can read a bit more about the Mano Machine [here](https://wikipedia.org/wiki/Mano_machine).

All rights go to M. Morris Mano and the original publishers of his book.


## Architecture

This project is organized as a Rust workspace with the following components:

### Core Library (`mano-lib`)
- **Machine simulation engine** - Complete implementation of the Mano computer architecture
- **CPU execution** - Instruction fetch, decode, and execute cycles with stepping capability
- **Assembler** - Two-pass assembler for Mano assembly language
- **Storage management** - Program loading and symbol table management
- **Message system** - Structured logging and error reporting

### Frontend Applications

#### CLI Frontend (`mano-cli`)
Command-line interface for the Mano machine simulator.

**Usage:**
```bash
# Run a program
cargo run --bin mano-cli -- run example_program.txt

# Assemble only (no execution)
cargo run --bin mano-cli -- assemble example_program.txt

# Debug mode (step-by-step execution)
cargo run --bin mano-cli -- debug example_program.txt
```

**Features:**
- Direct program execution
- Assembly-only mode for validation
- Interactive debugging with step-by-step execution
- Memory inspection
- Optional debug output

#### TUI Frontend (`mano-tui`)
(This frontend is still WIP)

Terminal User Interface with real-time visualization.

**Usage:**
```bash
cargo run --bin mano-tui -- example_program.txt
```

**Features:**
- Live register display
- Memory visualization
- Message logging
- Interactive controls:
  - `S` - Step through execution
  - `R` - Toggle auto-run mode
  - `X` - Reset machine
  - `Q` - Quit

#### Web Frontend (`mano-web`)
HTTP REST API with web interface.

**Usage:**
```bash
cargo run --bin mano-web -- --port 3000
# Open http://localhost:3000 in your browser
```

**Features:**
- WASM-based client web application
- Real-time state updates
- Program editing and execution
- Memory and register visualization
- Debugging and stepping capabilities

## Example Program

The repository includes an example program (`example_program.txt`) that demonstrates basic Mano assembly:

```assembly
   ORG 0    /Origin of program is location 0
   LDA A    /Load operand from location A
   ADD B    /Add operand from location B
   STA C    /Store sum in location C
   HLT      /Halt computer
A, DEC 83   /Decimal operand
B, DEC -23  /Decimal operand
C, DEC 0    /Sum stored in location C
   END      /End of symbolic program
```
This program adds the contents of memory locations A and B, and moves the result to C.

In this example, the value at memory location 6 should be equal to `60` or `0x3C` after execution.  

## Building and Testing

```bash
# Build all frontends
cargo build

# Run tests
cargo test

# Build specific frontend
cargo build --bin mano-cli
cargo build --bin mano-tui
cargo build --bin mano-web

# Check all packages compile
cargo check
```
