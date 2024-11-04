//! world module
//!
//! contents describe world of the simulation domain

use crate::helpers::coordinate_triplet::CoordinateTriplet;

/// `World` struct
///
/// describes the world of the simulation domain
#[derive(Debug)]
struct World {
    /// (m) size of bounding box
    size: CoordinateTriplet<f64>,

    /// number of cells
    cells: CoordinateTriplet<u64>,

    /// (m) spatial increment
    delta: CoordinateTriplet<f64>,
}

impl World {
    /// `World` constructor
    ///
    /// # Arguments
    /// - `size`: &[f64; 3] (m) size of bounding box
    /// - `cells`: &[u64; 3] number of cells
    ///
    /// # Returns
    /// `Result<World, anyhow::Error>`
    ///
    /// # Errors
    /// - any call to `CoordinateTriplet::new()` fails
    pub fn new(size: &[f64; 3], cells: &[u64; 3]) -> Result<World, anyhow::Error> {
        // unpack dimensions
        let size: CoordinateTriplet<f64> = CoordinateTriplet::new(size[0], size[1], size[2])?;

        // unpack cells
        let cells: CoordinateTriplet<u64> = CoordinateTriplet::new(cells[0], cells[1], cells[2])?;

        // initialize spatial increments
        let dx = size.x / (cells.x - 1) as f64;
        let dy = size.y / (cells.y - 1) as f64;
        let dz = size.z / (cells.z - 1) as f64;
        let delta: CoordinateTriplet<f64> = CoordinateTriplet::new(dx, dy, dz)?;

        // todo add assertion that all spacing is less than that of the Debeye length

        Ok(World { size, cells, delta })
    }
}

#[cfg(test)]
mod tests {
    
}
