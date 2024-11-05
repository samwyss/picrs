//! model module
//!
//! describes a model facade struct for using picrs

use crate::world::World;

/// `Model` struct
///
/// provides a facade for using picrs
pub struct Model {
    // world simulation object
    world: World,
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

        // construct world
        let world = World::new(&size, &cells)?;

        Ok(Model { world })
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
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
