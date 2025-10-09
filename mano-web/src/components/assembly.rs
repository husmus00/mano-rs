use leptos::*;
use wasm_bindgen::JsCast;

#[component]
pub fn Assembly(
    assembled_program: ReadSignal<Vec<String>>,
) -> impl IntoView {
    let scroll_container = create_node_ref::<html::Div>();

    // Auto-scroll when assembled program changes
    create_effect(move |_| {
        assembled_program.track();
        if let Some(container) = scroll_container.get() {
            let element = container.unchecked_ref::<web_sys::HtmlElement>();
            element.set_scroll_top(element.scroll_height());
        }
    });

    view! {
        <div class="assembly-pane">
            <h2 class="pane-title">"Assembled Program"</h2>
            <div class="assembly-content scrollable" node_ref=scroll_container>
                {move || {
                    let program = assembled_program.get();
                    if program.is_empty() {
                        view! {
                            <div class="assembly-empty">"No assembled program"</div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="assembly-table">
                                <div class="assembly-header">
                                    <span class="assembly-col-lin">"LIN"</span>
                                    <span class="assembly-col-hex">"HEX"</span>
                                    <span class="assembly-col-dec">"DEC"</span>
                                </div>
                                {program.iter()
                                    .enumerate()
                                    .map(|(addr, hex_str)| {
                                        let hex_value = u16::from_str_radix(hex_str, 16).unwrap_or(0);
                                        let dec_value = hex_value as i16;

                                        view! {
                                            <div class="assembly-row">
                                                <span class="assembly-col-lin">{format!("[{:04X}]", addr)}</span>
                                                <span class="assembly-col-hex">{format!("{:04X}", hex_value)}</span>
                                                <span class="assembly-col-dec">{format!("{}", dec_value)}</span>
                                            </div>
                                        }
                                    })
                                    .collect_view()
                                }
                            </div>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}
