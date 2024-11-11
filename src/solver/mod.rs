//! solver module
//!
//! contains various electromagnetic solvers

pub mod gauss_seidel_sor;

use gauss_seidel_sor::*;

enum Solver {
    GaussSeidelSOR(GaussSeidelSOR),
}

impl Solver {
    pub fn new() -> Result<Solver, anyhow::Error> {

    }
}