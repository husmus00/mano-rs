# Mano Machine Specifications

This directory contains specifications for the various Mano Machine simulator interfaces.

## Available Interfaces

- **[CLI](cli.md)** - Command-line interface for running Mano assembly programs
- **[TUI](tui.md)** - Terminal user interface with interactive debugging
- **[Web](web.md)** - Web-based API and frontend interface

## Core Architecture

All interfaces implement the same Mano Machine architecture:
- 16-bit word size
- 4K memory (4096 words)
- Basic Computer instruction set
- Two-pass assembler

For implementation details, see the respective interface specifications.
