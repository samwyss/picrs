use crate::solver::Engine;

const SOR: f64 = 1.4;
const ITER_BETWEEN_CONV_CHECK: u64 = 25;

pub struct GaussSeidelSOR{
    max_iter: u64,
    tolerance: f64,
}

impl GaussSeidelSOR {
    pub fn new(max_iter: &u64, tolerance: &f64) -> Result<GaussSeidelSOR, anyhow::Error> {
        let max_iter = *max_iter;

        let tolerance = *tolerance;

        Ok(GaussSeidelSOR { max_iter, tolerance })
    }
}