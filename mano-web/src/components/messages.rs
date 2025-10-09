use leptos::*;
use mano_lib::message::{Level, Messages};
use wasm_bindgen::JsCast;

#[component]
pub fn MessagesPane(
    messages: ReadSignal<Messages>,
    debug_mode: ReadSignal<bool>,
) -> impl IntoView {
    let scroll_container = create_node_ref::<html::Div>();

    // Auto-scroll when messages change
    create_effect(move |_| {
        messages.track();
        if let Some(container) = scroll_container.get() {
            let element = container.unchecked_ref::<web_sys::HtmlElement>();
            element.set_scroll_top(element.scroll_height());
        }
    });

    view! {
        <div class="messages-pane">
            <div class="pane-title-bar">
                <h2 class="pane-title">"Messages"</h2>
                <div class="message-legend">
                    <div class="legend-item">
                        <span class="legend-dot legend-info"></span>
                        <span class="legend-label">"Info"</span>
                    </div>
                    <div class="legend-item">
                        <span class="legend-dot legend-error"></span>
                        <span class="legend-label">"Error"</span>
                    </div>
                    {move || if debug_mode.get() {
                        view! {
                            <div class="legend-item">
                                <span class="legend-dot legend-debug"></span>
                                <span class="legend-label">"Debug"</span>
                            </div>
                        }.into_view()
                    } else {
                        view! { <></> }.into_view()
                    }}
                </div>
            </div>
            <div class="messages-content scrollable" node_ref=scroll_container>
                {move || {
                    let msgs = messages.get();
                    if msgs.entries.is_empty() {
                        view! {
                            <div class="message-empty">"No messages"</div>
                        }.into_view()
                    } else {
                        msgs.entries.iter()
                            .filter(|(level, _)| {
                                // Show debug messages only in debug mode
                                !matches!(level, Level::Debug) || debug_mode.get()
                            })
                            .map(|(level, msg)| {
                                let class = match level {
                                    Level::Info => "message-info",
                                    Level::Error => "message-error",
                                    Level::Debug => "message-debug",
                                };

                                view! {
                                    <div class=class>
                                        <span class="message-dot"></span>
                                        <span class="message-text">{msg}</span>
                                    </div>
                                }
                            })
                            .collect_view()
                    }
                }}
            </div>
        </div>
    }
}
