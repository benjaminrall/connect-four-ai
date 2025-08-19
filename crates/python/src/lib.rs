//! Python bindings for a high-performance implementation of a perfect Connect Four solver.

mod position;
mod solver;
mod ai_player;

use pyo3::prelude::*;

/// A high performance implementation of a perfect Connect Four solver, written in Rust.
///
/// This library provides functionality to compute the score and optimal move for any
/// given Connect Four position.
#[pymodule]
mod connect_four_ai {
    #[pymodule_export]
    use crate::position::PyPosition;

    #[pymodule_export]
    use crate::solver::PySolver;

    #[pymodule_export]
    use crate::ai_player::PyAIPlayer;

    #[pymodule_export]
    use crate::ai_player::PyDifficulty;
}