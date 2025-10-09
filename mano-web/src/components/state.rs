use leptos::*;
use mano_lib::machine::MachineState;

#[component]
pub fn State(
    machine_state: ReadSignal<Option<MachineState>>,
    debug_mode: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="state-pane">
            <div class="pane-title-bar">
                <h2 class="pane-title">"Machine State"</h2>
                {move || if debug_mode.get() {
                    view! {
                        <div class="debug-indicator">
                            <span class="debug-light"></span>
                            <span class="debug-label">"Debug Mode"</span>
                        </div>
                    }.into_view()
                } else {
                    view! { <></> }.into_view()
                }}
            </div>
            <div class="state-content scrollable">
                {move || {
                    match machine_state.get() {
                        None => view! {
                            <div class="state-empty">"No state available"</div>
                        }.into_view(),
                        Some(state) => view! {
                            <div class="state-display-horizontal">
                                <div class="cpu-state">
                                    <h3 class="state-section-title">"CPU"</h3>
                                    <div class="registers-list">
                                        <div class="register-row">
                                            <span class="register-name">"PC"</span>
                                            <span class="register-value">{format!("{:04X}", state.program_counter)}</span>
                                        </div>
                                        <div class="register-row">
                                            <span class="register-name">"AC"</span>
                                            <span class="register-value">{format!("{:04X}", state.accumulator)}</span>
                                        </div>
                                        <div class="register-row">
                                            <span class="register-name">"IR"</span>
                                            <span class="register-value">{format!("{:04X}", state.instruction_register)}</span>
                                        </div>
                                        <div class="register-row">
                                            <span class="register-name">"AR"</span>
                                            <span class="register-value">{format!("{:04X}", state.address_register)}</span>
                                        </div>
                                        <div class="register-row">
                                            <span class="register-name">"DR"</span>
                                            <span class="register-value">{format!("{:04X}", state.data_register)}</span>
                                        </div>
                                        <div class="register-row">
                                            <span class="register-name">"E"</span>
                                            <span class="register-value">{format!("{:04X}", state.extend_register)}</span>
                                        </div>
                                        <div class="register-row">
                                            <span class="register-name">"SC"</span>
                                            <span class="register-value">{format!("{}", state.sequence_counter)}</span>
                                        </div>
                                        <div class="register-row">
                                            <span class="register-name">"Status"</span>
                                            <span class="register-value register-status" class:halted=state.is_halted>
                                                {if state.is_halted { "HALT" } else { "RUN" }}
                                            </span>
                                        </div>
                                    </div>
                                </div>

                                <div class="memory-state">
                                    <h3 class="state-section-title">"Memory"</h3>
                                    <div class="memory-dump-lengthwise">
                                        {
                                            let memory = &state.memory_snapshot;
                                            (0..memory.len()).map(|addr| {
                                                view! {
                                                    <div class="memory-line">
                                                        <span class="mem-addr-inline">{format!("{:02X}", addr)}</span>
                                                        <span class="mem-value-inline">{format!("{:04X}", memory[addr])}</span>
                                                    </div>
                                                }
                                            }).collect_view()
                                        }
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}
