# CLI Application

> Command-line interface specification for Mano Machine simulator

## Overview

The CLI provides a simple, non-interactive way to assemble and execute Mano assembly programs from the command line, with formatted output suitable for terminal viewing.

## Features

- Single-command execution of assembly programs
- Verbose mode for detailed debug output
- Formatted source and assembled program display
- CPU register state visualisation
- Memory hex-dump display

## Implementation Details

- Takes an assembly program as a direct argument and an optional "-v/--verbose" flag.
- Supports the "-h / --help" flag for usage
- The application takes in the assembly file and prints it, then passes it to prime(). Then it prints the assembled program.
- The pre- and post-assembly programs should be nicely formatted when printed.
- It then runs tick() in a loop until the machine halts or there's an error in messages.
- It outputs the info and error messages as they're received from Machine. If the -v flag exists, print debug messages.
- Debug and error output should be prepended with "DBG:" and "ERR:", respectively.
- After exiting the loop, the machine state should be requested from Machine and nicely formatted and printed
- This state consists of the CPU state and a hex-dump-style display of memory contents.
- Then the program exits.