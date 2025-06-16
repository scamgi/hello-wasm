use wasm_bindgen::prelude::*;

// Questa macro `#[wasm_bindgen]` rende la funzione accessibile da JavaScript.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {} from Rust WebAssembly!", name)
}

#[wasm_bindgen(start)]
pub fn run() {
    // Questa funzione verrà eseguita automaticamente quando il modulo Wasm viene caricato.
    // console_log è una macro che mappa a console.log di JavaScript.
    web_sys::console::log_1(&"Rust module loaded!".into());
}

// Per usare console_log, dobbiamo aggiungere una dipendenza 'web-sys'
// Non dimenticare di aggiungere "web_sys" come dipendenza nel tuo Cargo.toml
// e attivare la feature "console"
