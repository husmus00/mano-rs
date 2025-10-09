use leptos::*;
use mano_lib::{machine::MachineState, message::Messages};

use super::{assembly::Assembly, messages::MessagesPane, state::State};

#[component]
pub fn Output(
    debug_mode: ReadSignal<bool>,
    messages: ReadSignal<Messages>,
    assembled_program: ReadSignal<Vec<String>>,
    machine_state: ReadSignal<Option<MachineState>>,
) -> impl IntoView {
    view! {
        <div class="output-container">
            <div class="output-debug">
                <div class="output-top">
                    <div class="output-pane output-messages">
                        <MessagesPane messages=messages debug_mode=debug_mode />
                    </div>
                    <div class="output-pane output-assembly">
                        <Assembly assembled_program=assembled_program />
                    </div>
                </div>
                <div class="output-bottom">
                    <div class="output-pane output-state">
                        <State machine_state=machine_state debug_mode=debug_mode />
                    </div>
                </div>
            </div>
        </div>
    }
}
