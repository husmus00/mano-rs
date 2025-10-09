mod app;
mod components;

use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn main() {
    // Set up better panic messages in the browser console
    console_error_panic_hook::set_once();

    // Initialise logging
    _ = console_log::init_with_level(log::Level::Debug);

    log::info!("Mounting Mano Web App...");

    mount_to_body(|| {
        log::info!("App component rendering...");
        view! { <app::App/> }
    });

    log::info!("Mano Web App mounted successfully");
}
