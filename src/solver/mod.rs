//! solver module
//!
//! contains various electromagnetic solvers

pub mod gauss_seidel_sor;

use gauss_seidel_sor::*;

enum Solver {
    GaussSeidelSOR(GaussSeidelSOR),
}

trait Engine {
    fn update(d_time: f64);
}