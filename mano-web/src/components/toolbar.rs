use leptos::*;

#[component]
pub fn Toolbar(
    on_assemble: impl Fn() + 'static + Clone,
    on_run: impl Fn() + 'static + Clone,
    on_step: impl Fn() + 'static + Clone,
    on_reset: impl Fn() + 'static + Clone,
    debug_mode: ReadSignal<bool>,
    on_toggle_debug: impl Fn() + 'static + Clone,
    is_running: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="toolbar">
            <div class="toolbar-buttons">
                <button
                    class="toolbar-button"
                    on:click=move |_| on_assemble()
                    disabled=move || is_running.get()
                >
                    "Assemble"
                </button>
                {
                    let on_step_clone = on_step.clone();
                    let on_run_clone = on_run.clone();
                    move || if debug_mode.get() {
                        let on_step_inner = on_step_clone.clone();
                        view! {
                            <button
                                class="toolbar-button step-button"
                                on:click=move |_| on_step_inner()
                            >
                                "Step"
                            </button>
                        }.into_view()
                    } else {
                        let on_run_inner = on_run_clone.clone();
                        view! {
                            <button
                                class="toolbar-button"
                                on:click=move |_| on_run_inner()
                                disabled=move || is_running.get()
                            >
                                {move || if is_running.get() { "Running..." } else { "Run" }}
                            </button>
                        }.into_view()
                    }
                }
                <button
                    class="toolbar-button"
                    on:click=move |_| on_reset()
                    disabled=move || is_running.get()
                >
                    "Reset"
                </button>
            </div>
            <div class="toolbar-controls">
                <button
                    class="debug-toggle-button"
                    class:pressed=move || debug_mode.get()
                    on:click=move |_| on_toggle_debug()
                >
                    "Debug Mode"
                </button>
            </div>
        </div>
    }
}
