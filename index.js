import init, { greet } from "./pkg/hello_wasm.js";

async function run() {
  // Inizializza il modulo WebAssembly
  await init();

  // Chiama la funzione 'greet' dal tuo modulo Rust Wasm
  const message = greet("World");

  // Mostra il messaggio sulla pagina HTML
  document.getElementById("output").innerText = message;

  // Vedrai "Rust module loaded!" nella console del browser grazie a run()
  console.log(
    "JavaScript side: Wasm module initialized and greet function called.",
  );
}

run();
