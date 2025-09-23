use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::{Duration, Instant};

use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Table, Row, Cell},
    Frame, Terminal,
};

use mano_lib::machine::{Machine, MachineState};
use mano_lib::message::Level;

#[derive(Parser)]
#[command(about = "Mano Machine TUI")]
struct Args {
    /// Assembly file to load
    file: String,
}

struct App {
    machine: Machine,
    messages: Vec<(Level, String)>,
    running: bool,
    auto_run: bool,
    last_tick: Instant,
}

impl App {
    fn new(machine: Machine) -> Self {
        Self {
            machine,
            messages: Vec::new(),
            running: false,
            auto_run: false,
            last_tick: Instant::now(),
        }
    }

    fn step(&mut self) {
        if !self.machine.is_halted() && self.machine.is_primed() {
            let result = self.machine.tick();
            for entry in result.entries {
                self.messages.push(entry);
            }

            // Keep only last 100 messages
            if self.messages.len() > 100 {
                self.messages.drain(0..self.messages.len() - 100);
            }

            if self.machine.is_halted() {
                self.auto_run = false;
            }
        }
    }

    fn toggle_auto_run(&mut self) {
        self.auto_run = !self.auto_run;
        if self.auto_run {
            self.last_tick = Instant::now();
        }
    }

    fn reset(&mut self) {
        let _ = self.machine.reset();
        self.messages.clear();
        self.auto_run = false;
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Load and prime the machine
    let mut machine = Machine::new();
    let program = read_file(&args.file)?;
    let result = machine.prime(program);

    if result.has_errors() {
        println!("Failed to load program:");
        for (level, msg) in &result.entries {
            if matches!(level, Level::Error) {
                println!("  ERROR: {}", msg);
            }
        }
        return Ok(());
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new(machine);
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        // Auto-run logic
        if app.auto_run && app.last_tick.elapsed() >= Duration::from_millis(100) {
            app.step();
            app.last_tick = Instant::now();
        }

        // Handle input
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('s') => app.step(),
                        KeyCode::Char('r') => app.toggle_auto_run(),
                        KeyCode::Char('x') => app.reset(),
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(0)].as_ref())
        .split(chunks[0]);

    // Registers panel
    draw_registers(f, left_chunks[0], &app.machine.get_state());

    // Memory panel
    draw_memory(f, left_chunks[1], &app.machine);

    // Messages panel
    draw_messages(f, chunks[1], &app.messages, app.auto_run, app.machine.is_halted());
}

fn draw_registers(f: &mut Frame, area: Rect, state: &MachineState) {
    let rows = vec![
        Row::new(vec![Cell::from("PC"), Cell::from(format!("0x{:04X}", state.program_counter))]),
        Row::new(vec![Cell::from("AC"), Cell::from(format!("0x{:04X}", state.accumulator))]),
        Row::new(vec![Cell::from("IR"), Cell::from(format!("0x{:04X}", state.instruction_register))]),
        Row::new(vec![Cell::from("AR"), Cell::from(format!("0x{:04X}", state.address_register))]),
        Row::new(vec![Cell::from("DR"), Cell::from(format!("0x{:04X}", state.data_register))]),
        Row::new(vec![Cell::from("E"), Cell::from(format!("0x{:04X}", state.extend_register))]),
        Row::new(vec![Cell::from("SC"), Cell::from(format!("{}", state.sequence_counter))]),
    ];

    let status_style = if state.is_halted {
        Style::default().fg(Color::Red)
    } else if state.is_running {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Yellow)
    };

    let status = if state.is_halted {
        "HALTED"
    } else if state.is_running {
        "RUNNING"
    } else {
        "READY"
    };

    let table = Table::new(rows)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!("Registers [{}]", status))
            .title_style(status_style))
        .widths(&[Constraint::Length(4), Constraint::Length(8)]);

    f.render_widget(table, area);
}

fn draw_memory(f: &mut Frame, area: Rect, machine: &Machine) {
    let memory = machine.load_memory_range(0, 16);
    let mut items = Vec::new();

    for (addr, value) in memory.iter().enumerate() {
        if *value != 0 {
            let line = Line::from(vec![
                Span::raw(format!("[{:04X}]: ", addr)),
                Span::styled(format!("{:04X}", value), Style::default().fg(Color::Cyan)),
            ]);
            items.push(ListItem::new(line));
        }
    }

    if items.is_empty() {
        items.push(ListItem::new("No data in memory"));
    }

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Memory"));

    f.render_widget(list, area);
}

fn draw_messages(f: &mut Frame, area: Rect, messages: &[(Level, String)], auto_run: bool, is_halted: bool) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(area);

    // Messages
    let items: Vec<ListItem> = messages
        .iter()
        .rev()
        .take(chunks[0].height as usize - 2) // Account for borders
        .map(|(level, msg)| {
            let style = match level {
                Level::Error => Style::default().fg(Color::Red),
                Level::Info => Style::default().fg(Color::Green),
                Level::Debug => Style::default().fg(Color::Gray),
            };
            ListItem::new(Line::from(Span::styled(msg.clone(), style)))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Messages"));

    f.render_widget(list, chunks[0]);

    // Controls
    let auto_status = if auto_run { "ON" } else { "OFF" };
    let help_text = if is_halted {
        format!("HALTED | [Q]uit [X]reset | Auto: {}", auto_status)
    } else {
        format!("[S]tep [R]un [Q]uit [X]reset | Auto: {}", auto_status)
    };

    let controls = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Controls"));

    f.render_widget(controls, chunks[1]);
}

fn read_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    let lines: Result<Vec<String>, _> = buf.lines().collect();
    Ok(lines?)
}