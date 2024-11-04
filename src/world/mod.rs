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
    use crate::world::World;

    /// helper function that sets up a `World` for testing
    ///
    /// # Arguments
    ///
    /// # Returns
    /// `Result<World, anyhow::Error>`
    ///
    /// # Errors
    ///
    fn setup() -> Result<World, anyhow::Error> {
        let size: [f64; 3] = [1.0, 2.0, 3.0];
        let cells: [u64; 3] = [3, 11, 31];
        World::new(&size, &cells)
    }

    /// tests `World::new()` for success
    ///
    /// # Errors
    /// - `world::new()` fails for valid input
    ///
    #[test]
    fn new_success() {
        assert!(setup().is_ok());
    }

    /// tests `World::new()` for correct setting of size member
    ///
    /// # Errors
    /// - `world::new()` sets incorrect `world.size.x`
    /// - `world::new()` sets incorrect `world.size.y`
    /// - `world::new()` sets incorrect `world.size.z`
    ///
    #[test]
    fn new_correct_size() {
        // create world for testing
        let world = setup().unwrap();

        // assertions
        assert_eq!(world.size.x, 1.0);
        assert_eq!(world.size.y, 2.0);
        assert_eq!(world.size.z, 3.0);
    }

    /// tests `World::new()` for correct setting of cells member
    ///
    /// # Errors
    /// - `world::new()` sets incorrect `world.cells.x`
    /// - `world::new()` sets incorrect `world.cells.y`
    /// - `world::new()` sets incorrect `world.cells.z`
    ///
    #[test]
    fn new_correct_cells() {
        // create world for testing
        let world = setup().unwrap();

        // assertions
        assert_eq!(world.cells.x, 3);
        assert_eq!(world.cells.y, 11);
        assert_eq!(world.cells.z, 31);
    }

    /// tests `World::new()` for correct setting of delta member
    ///
    /// # Errors
    /// - `world::new()` sets incorrect `world.delta.x`
    /// - `world::new()` sets incorrect `world.delta.y`
    /// - `world::new()` sets incorrect `world.delta.z`
    ///
    #[test]
    fn new_correct_delta() {
        // create world for testing
        let world = setup().unwrap();

        // assertions
        assert_eq!(world.delta.x, 0.5);
        assert_eq!(world.delta.y, 0.2);
        assert_eq!(world.delta.z, 0.1);
    }
}
