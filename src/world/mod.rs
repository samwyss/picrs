//! world module
//!
//! contents describe world of the simulation domain

use crate::field::scalar::ScalarField;
use crate::field::vector::VectorField;
use crate::utils::coordinate_triplet::CoordinateTriplet;
use crate::constants::INV_VAC_PERM;

/// sor acceleration constant
const SOR_ACC: f64 = 1.4;

/// gauss-seidel iterations between convergence check
const CONV_CHECK_ITER: u64 = 25;

/// gauss-seidel max iterations
const GS_MAX_ITER: u64 = 10000;

/// gauss-seidel tolerance
const GS_TOL: f64 = 1e-5;

/// `World` struct
///
/// describes the world of the simulation domain
#[derive(Debug)]
pub struct World {
    /// (m) size of bounding box
    size: CoordinateTriplet<f64>,

    /// number of cells
    cells: CoordinateTriplet<usize>,

    /// (m) spatial increment
    delta: CoordinateTriplet<f64>,

    /// (V) electric field potential
    potential: ScalarField<f64>,

    /// (c/m^3) electric charge density
    charge_density: ScalarField<f64>,

    /// (V/m) electric field
    electric_field: VectorField<f64>,

    /// (m^3) cell volumes
    cell_vol: ScalarField<f64>,

    /// (m^-2) inverse spatial increments squared for use in gauss-seidel sor scheme
    delta_inv_sq: CoordinateTriplet<f64>,
}

impl World {
    /// `World` constructor
    ///
    /// # Arguments
    /// - `size`: &[f64; 3] (m) size of bounding box
    /// - `cells`: &[usize; 3] number of cells
    ///
    /// # Returns
    /// `Result<World, anyhow::Error>`
    ///
    /// # Errors
    /// - any call to `CoordinateTriplet::new()` fails
    /// - any call to `ScalarField::new()` fails
    /// - any call to `VectorField::new()` fails
    pub fn new(size: &[f64; 3], cells: &[usize; 3]) -> Result<World, anyhow::Error> {
        // unpack dimensions
        let size: CoordinateTriplet<f64> = CoordinateTriplet::new(size[0], size[1], size[2])?;

        // unpack cells
        let cells: CoordinateTriplet<usize> = CoordinateTriplet::new(cells[0], cells[1], cells[2])?;

        // initialize spatial increments
        let dx = size.x / (cells.x - 1) as f64;
        let dy = size.y / (cells.y - 1) as f64;
        let dz = size.z / (cells.z - 1) as f64;
        let delta: CoordinateTriplet<f64> = CoordinateTriplet::new(dx, dy, dz)?;

        // todo add assertion that all spacing is less than that of the Debeye length

        // precompute inverse of delta squared for use in gauss-seidel sor scheme
        let dx_inv_sq = 1.0 / (dx * dx);
        let dy_inv_sq = 1.0 / (dy * dy);
        let dz_inv_sq = 1.0 / (dz * dz);
        let delta_inv_sq = CoordinateTriplet::new(dx_inv_sq, dy_inv_sq, dz_inv_sq)?;

        // initialize electric potential
        let potential: ScalarField<f64> = ScalarField::new(&cells)?;

        // initialize charge density
        let charge_density: ScalarField<f64> = ScalarField::new(&cells)?;

        // initialize electric field
        let electric_field: VectorField<f64> = VectorField::new(&cells)?;

        // initialize cell volumes
        // todo fill in properly
        let cell_vol: ScalarField<f64> = ScalarField::new(&cells)?;

        Ok(World {
            size,
            cells,
            delta,
            potential,
            charge_density,
            electric_field,
            cell_vol,
            delta_inv_sq,
        })
    }

    fn update_electrostatic_sys(&mut self) -> Result<(), anyhow::Error> {
        Self::solve_potential(self)?;
        Self::solve_electric_field()?;
        Ok(())
    }

    fn solve_potential(&mut self) -> Result<(), anyhow::Error> {
        // loop counter
        let mut loop_ctr: u64 = 0;

        // l2 error norm
        let mut l2_err_norm: f64 = f64::MAX;

        // gauss-seidel sor scheme loop
        while (loop_ctr <= GS_MAX_ITER) || (l2_err_norm > GS_TOL) {
            // update potential on interior nodes
            for i in 1..(self.cells.x - 1) {
                for j in 1..(self.cells.y - 1) {
                    for k in 1..(self.cells.z - 1) {
                        // solve potential using gauss-seidel
                        let potential_new: f64 = (self.charge_density[(i, j, k)] * INV_VAC_PERM
                        + self.delta_inv_sq.x * (self.potential[(i + 1, j, k)] + self.potential[(i - 1, j, k)])
                        + self.delta_inv_sq.y * (self.potential[(i, j + 1, k)] - self.potential[(i, j - 1, k)])
                        + self.delta_inv_sq.z * (self.potential[(i, j, k + 1)] - self.potential[(i, j, k - 1)]))
                        / (2.0 * (self.delta_inv_sq.x + self.delta_inv_sq.y + self.delta_inv_sq.z));

                        // apply sor
                        self.potential[(i, j, k)] += SOR_ACC * (potential_new - self.potential[(i, j, k)]);
                    }
                }
            }

            // conditionally check for convergence
            if (loop_ctr % CONV_CHECK_ITER) == 0 {
                // residue accumulator
                let mut res_acc: f64 = 0.0;

                // accumulate l2 error norm
                for i in 1..(self.cells.x - 1) {
                    for j in 1..(self.cells.y - 1) {
                        for k in 1..(self.cells.z - 1) {
                            // residue vector value
                            let res = - self.potential[(i, j, k)] * 2.0 * (self.delta_inv_sq.x + self.delta_inv_sq.y + self.delta_inv_sq.z)
                                + self.charge_density[(i, j, k)] * INV_VAC_PERM
                                + self.delta_inv_sq.x * (self.potential[(i + 1, j, k)] + self.potential[(i - 1, j, k)])
                                + self.delta_inv_sq.y * (self.potential[(i, j + 1, k)] - self.potential[(i, j - 1, k)])
                                + self.delta_inv_sq.z * (self.potential[(i, j, k + 1)] - self.potential[(i, j, k - 1)]);

                            res_acc += res * res;
                        }
                    }
                }

                // todo check for convergence
            }

            // increment loop counter
            loop_ctr += 1;
        }

        Ok(())
    }

    fn solve_electric_field() -> Result<(), anyhow::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::field::scalar::ScalarField;
    use crate::field::vector::VectorField;
    use crate::utils::coordinate_triplet::CoordinateTriplet;
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
        let cells: [usize; 3] = [3, 11, 31];
        World::new(&size, &cells)
    }

    /// tests `World::new()` for success
    ///
    /// # Errors
    /// - `World::new()` fails for valid input
    ///
    #[test]
    fn new_success() {
        assert!(setup().is_ok());
    }

    /// tests `World::new()` for correct setting of `World.size` member
    ///
    /// # Errors
    /// - `World::new()` sets incorrect `world.size.x`
    /// - `World::new()` sets incorrect `world.size.y`
    /// - `World::new()` sets incorrect `world.size.z`
    ///
    #[test]
    fn new_correct_size() {
        // setup
        let world = setup().unwrap();

        // assertions
        assert_eq!(world.size.x, 1.0);
        assert_eq!(world.size.y, 2.0);
        assert_eq!(world.size.z, 3.0);
    }

    /// tests `World::new()` for correct setting of `World.cells' member
    ///
    /// # Errors
    /// - `World::new()` sets incorrect `world.cells.x`
    /// - `World::new()` sets incorrect `world.cells.y`
    /// - `World::new()` sets incorrect `world.cells.z`
    ///
    #[test]
    fn new_correct_cells() {
        // setup
        let world = setup().unwrap();

        // assertions
        assert_eq!(world.cells.x, 3);
        assert_eq!(world.cells.y, 11);
        assert_eq!(world.cells.z, 31);
    }

    /// tests `World::new()` for correct setting of `World.delta` member
    ///
    /// # Errors
    /// - `World::new()` sets incorrect `world.delta.x`
    /// - `World::new()` sets incorrect `world.delta.y`
    /// - `World::new()` sets incorrect `world.delta.z`
    ///
    #[test]
    fn new_correct_delta() {
        // setup
        let world = setup().unwrap();

        // assertions
        assert_eq!(world.delta.x, 0.5);
        assert_eq!(world.delta.y, 0.2);
        assert_eq!(world.delta.z, 0.1);
    }

    /// tests `World::new()` for correct setting of `World.potential` member
    ///
    /// # Errors
    /// - `World::new()` sets incorrect `World.potential`
    /// - call to `CoordinateTriplet::new()` fails
    /// - call to `ScalarField::new()` fails
    ///
    #[test]
    fn new_correct_potential() {
        // setup
        let world = setup().unwrap();
        let cells = CoordinateTriplet::new(3, 11, 31).unwrap();

        // assertions
        assert_eq!(world.potential, ScalarField::new(&cells).unwrap())
    }

    /// tests `World::new()` for correct setting of `World.charge_density` member
    ///
    /// # Errors
    /// - `World::new()` sets incorrect `World.charge_density`
    /// - call to `CoordinateTriplet::new()` fails
    /// - call to `ScalarField::new()` fails
    ///
    #[test]
    fn new_correct_charge_density() {
        // setup
        let world = setup().unwrap();
        let cells = CoordinateTriplet::new(3, 11, 31).unwrap();

        // assertions
        assert_eq!(world.charge_density, ScalarField::new(&cells).unwrap())
    }

    /// tests `World::new()` for correct setting of `World.electric_field` member
    ///
    /// # Errors
    /// - `World::new()` sets incorrect `World.electric_field`
    /// - call to `CoordinateTriplet::new()` fails
    /// - call to `VectorField::new()` fails
    ///
    #[test]
    fn new_correct_electric_field() {
        // setup
        let world = setup().unwrap();
        let cells = CoordinateTriplet::new(3, 11, 31).unwrap();

        // assertions
        assert_eq!(world.electric_field, VectorField::new(&cells).unwrap())
    }

    /// tests `World::new()` for correct setting of `World.cell_vol` member
    ///
    /// # Errors
    /// - `World::new()` sets incorrect `World.cell_vol`
    /// - call to `CoordinateTriplet::new()` fails
    /// - call to `ScalarField::new()` fails
    ///
    #[test]
    fn new_correct_cell_vol() {
        // setup
        let world = setup().unwrap();
        let cells = CoordinateTriplet::new(3, 11, 31).unwrap();

        // assertions
        assert_eq!(world.cell_vol, ScalarField::new(&cells).unwrap())
    }

    /// tests `World::new()` for correct setting of `World.delta_inv_sq` member
    ///
    /// # Errors
    /// - `World::new()` sets incorrect `World.delta_inv_sq`
    /// - call to `CoordinateTriplet::new()` fails
    ///
    #[test]
    fn new_correct_delta_inv_sq() {
        // setup
        let world = setup().unwrap();

        // assertions
        assert_eq!(world.delta_inv_sq.x, 1.0 / (world.delta.x * world.delta.x));
        assert_eq!(world.delta_inv_sq.y, 1.0 / (world.delta.y * world.delta.y));
        assert_eq!(world.delta_inv_sq.z, 1.0 / (world.delta.z * world.delta.z));
    }
}
