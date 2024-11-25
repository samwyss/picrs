//! model module
//!
//! describes a model facade struct for using picrs

use crate::engine::Electrostatic;

/// `Model` struct
///
/// provides a facade for using picrs
pub struct Model {
    // engine simulation object
    engine: Electrostatic,
}

impl Model {
    /// `Model` constructor
    ///
    /// # Arguments
    ///
    /// # Returns
    /// `Result<Model, anyhow::Error>`
    ///
    /// # Errors
    ///
    pub fn new() -> Result<Model, anyhow::Error> {
        // todo read in from input deck
        let size: [f64; 3] = [1.0, 1.0, 1.0];
        let cells: [usize; 3] = [10, 10, 10];

        // construct engine
        let engine = Electrostatic::new(&size, &cells)?;

        Ok(Model { engine })
    }

    /// runs configured `Model`
    ///
    /// # Arguments
    /// - `&mut self` mutable reference to self
    ///
    /// # Returns
    /// `Result<(), anyhow::Error>`
    ///
    /// # Errors
    ///
    pub fn run(&mut self) -> Result<(), anyhow::Error> {
        for step in 0..10 {
            self.engine.update()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
