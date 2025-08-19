//! WebAssembly bindings for a high-performance implementation of a perfect Connect Four solver.

mod position;
mod solver;
mod ai_player;

use wasm_bindgen::prelude::*;
pub use position::WASMPosition;
pub use solver::WASMSolver;
pub use ai_player::{WASMDifficulty, WASMAIPlayer};

/// Sets up a hook to log Rust panics to the browser's console when the
/// WASM module is first loaded.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}