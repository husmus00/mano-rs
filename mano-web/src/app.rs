use leptos::*;
use mano_lib::{machine::Machine, message::Messages};

use crate::components::{
    editor::Editor,
    toolbar::Toolbar,
    output::Output,
};

#[component]
pub fn App() -> impl IntoView {
    // State
    let (source_code, set_source_code) = create_signal(String::from(
        "ORG 0              /Origin of program is location 0\n\
LDA A              /Load operand from location A\n\
ADD B              /Add operand from location B\n\
STA C              /Store sum in location C\n\
HLT                /Halt computer\n\
A,  DEC 83         /Decimal operand\n\
B,  DEC -23        /Decimal operand\n\
C,  DEC 0          /Sum stored in location C\n\
END                /End of symbolic program"
    ));

    let (debug_mode, set_debug_mode) = create_signal(false);
    let (messages, set_messages) = create_signal(Messages::new());
    let (assembled_program, set_assembled_program) = create_signal(Vec::<String>::new());

    // Initialise machine state with zeroed values (permanent display)
    let initial_state = {
        use mano_lib::machine::MachineState;
        Some(MachineState {
            program_counter: 0,
            accumulator: 0,
            instruction_register: 0,
            address_register: 0,
            data_register: 0,
            extend_register: 0,
            sequence_counter: 0,
            is_halted: false,
            is_running: false,
            memory_snapshot: vec![0; 32],
        })
    };
    let (machine_state, set_machine_state) = create_signal(initial_state);
    let (is_running, set_is_running) = create_signal(false);

    // Machine instance (stored without cloning)
    let machine = store_value(Machine::new());

    // Actions
    let assemble = move || {
        let code = source_code.get();
        let lines: Vec<String> = code.lines().map(|s| s.to_string()).collect();

        // Prime the machine (uses interior mutability via update_value)
        // Note: We need to collect results outside the closure
        let msgs = {
            // Create temp storage for results
            use std::cell::RefCell;
            let msgs_cell = RefCell::new(Messages::new());

            machine.update_value(|m| {
                *m = Machine::new();
                *msgs_cell.borrow_mut() = m.prime(lines);
            });

            msgs_cell.into_inner()
        };

        let assembled = machine.with_value(|m| m.get_assembled_program().to_vec());
        let state = machine.with_value(|m| m.get_state());

        set_messages.set(msgs);
        set_assembled_program.set(assembled);
        set_machine_state.set(Some(state));
    };

    let run = move || {
        if is_running.get() {
            return;
        }

        set_is_running.set(true);

        // Run the machine and collect results
        let all_messages = {
            use std::cell::RefCell;
            let msgs_cell = RefCell::new(messages.get());

            machine.update_value(|m| {
                let mut step_count = 0;
                let max_steps = 10000;

                while !m.is_halted() && step_count < max_steps {
                    let mut step_messages = Messages::new();
                    m.tick(&mut step_messages);
                    msgs_cell.borrow_mut().combine(step_messages);

                    if msgs_cell.borrow().has_errors() {
                        break;
                    }

                    step_count += 1;
                }
            });

            msgs_cell.into_inner()
        };

        let final_state = machine.with_value(|m| m.get_state());

        set_messages.set(all_messages);
        set_machine_state.set(Some(final_state));
        set_is_running.set(false);
    };

    let step = move || {
        let step_messages = {
            use std::cell::RefCell;
            let msgs_cell = RefCell::new(messages.get());

            machine.update_value(|m| {
                let mut step_msg = Messages::new();
                m.tick(&mut step_msg);
                msgs_cell.borrow_mut().combine(step_msg);
            });

            msgs_cell.into_inner()
        };

        let state = machine.with_value(|m| m.get_state());

        set_messages.set(step_messages);
        set_machine_state.set(Some(state));
    };

    let reset = move || {
        machine.update_value(|m| {
            *m = Machine::new();
        });

        set_messages.set(Messages::new());
        set_assembled_program.set(Vec::new());

        // Create a zeroed machine state instead of None to keep display "permanent"
        let zeroed_state = machine.with_value(|_m| {
            use mano_lib::machine::MachineState;
            MachineState {
                program_counter: 0,
                accumulator: 0,
                instruction_register: 0,
                address_register: 0,
                data_register: 0,
                extend_register: 0,
                sequence_counter: 0,
                is_halted: false,
                is_running: false,
                memory_snapshot: vec![0; 32],
            }
        });
        set_machine_state.set(Some(zeroed_state));
    };

    let toggle_debug = move || {
        set_debug_mode.update(|d| *d = !*d);
    };

    view! {
        <div class="app">
            <div class="title-bar">
                <h1 class="app-title">"MANO MACHINE SIM"</h1>
                <a href="https://github.com/husmus00/mano-rs" target="_blank" rel="noopener noreferrer" class="github-link">
                    <i class="fab fa-github"></i>
                </a>
            </div>
            <div class="main-container">
                <div class="input-group">
                    <Editor
                        source_code=source_code
                        set_source_code=set_source_code
                    />
                </div>
                <div class="output-group">
                    <Output
                        debug_mode=debug_mode
                        messages=messages
                        assembled_program=assembled_program
                        machine_state=machine_state
                    />
                </div>
            </div>
            <div class="bottom-container">
                <Toolbar
                    on_assemble=assemble
                    on_run=run
                    on_step=step
                    on_reset=reset
                    debug_mode=debug_mode
                    on_toggle_debug=toggle_debug
                    is_running=is_running
                />
                <div class="copyright">"COPYRIGHT HSM SYSTEMS 1978"</div>
            </div>
        </div>
    }
}
