# Mano Web Interface

A WASM-based web interface for the Mano Machine simulator, built with Leptos and featuring a retro-inspired solarised dark theme.

## Prerequisites

- [Trunk](https://trunkrs.dev/) - WASM web application bundler
  ```bash
  cargo install trunk
  ```
- [wasm-bindgen-cli](https://rustwasm.github.io/wasm-bindgen/) (installed automatically by Trunk)

## Building

### Development Mode

Build and serve with hot-reload:

```bash
trunk serve
```

Then open http://127.0.0.1:8080 in your browser.

### Production Build

Build optimised WASM bundle:

```bash
trunk build --release
```

Output will be in the `dist/` directory.

## Running

### Option 1: Static Hosting

After building, the `dist/` directory contains all necessary files. You can:

- Copy to any static web server
- Host on GitHub Pages, Netlify, Vercel, etc.
- Open `dist/index.html` directly in a browser (may have CORS issues)

### Option 2: CLI Server

Run the included static file server:

```bash
# First build the WASM app
trunk build --release

# Then run the server
cargo run --features ssr --release
```

The server will start at http://127.0.0.1:8080

## Features

- **4-Pane Layout**: Input editor, messages, assembly output, and machine state always visible
- **Interactive Editor**:
  - Line numbers
  - Syntax highlighting (instructions, pseudo-ops, comments, numbers)
  - Transparent text overlay for highlighting
- **Assembly Output Pane**:
  - Three-column display: LIN (address), HEX (machine code), DEC (decimal value)
  - Auto-scroll as content is added
- **Machine State Pane**:
  - CPU registers display (PC, AC, IR, AR, DR, E, SC, Status)
  - 4x8 memory grid (32 words) in hex format
  - Permanent display (Reset zeroes values rather than clearing)
  - Debug mode indicator with animated warning light
- **Control Buttons**:
  - Assemble: Compile program
  - Run/Step: Execute program or single instruction (switches based on debug mode)
  - Reset: Clear all state and restart machine
  - Debug Mode: Retro-industrial toggle button with pressed state
- **Debug Mode Features**:
  - Step button (red text) for single instruction execution
  - Debug-level message output
  - Animated orange warning light in state pane
- **Auto-scrolling**: Messages and assembly panes scroll automatically as content is added
- **Retro Theme**: Solarised dark colour scheme with retro-inspired UI
- **Fully Client-Side**: No backend required, all processing in WASM

## Architecture

- **Frontend**: Leptos (CSR mode) compiled to WebAssembly
- **Core Logic**: `mano-lib` compiled to WASM
- **Styling**: CSS Grid layout with solarised theme
- **Optional Server**: Simple Axum static file server

## Development

The application is structured as:

```
mano-web/
├── src/
│   ├── lib.rs              # WASM entry point
│   ├── app.rs              # Main app component
│   ├── main.rs             # Optional CLI server
│   └── components/         # UI components
├── styles/
│   ├── main.css            # Layout and component styles
│   └── solarised.css       # Colour theme
├── index.html              # HTML template
└── Trunk.toml              # Build configuration
```

## Browser Support

Requires a modern browser with WebAssembly support:
- Chrome/Edge 57+
- Firefox 52+
- Safari 11+
