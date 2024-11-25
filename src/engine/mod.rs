//! engine module
//!
//! contents describe several computational engines for pic models

use crate::constants::INV_VAC_PERM;
use crate::field::scalar::ScalarField;
use crate::field::vector::VectorField;
use crate::utils::coordinate_triplet::CoordinateTriplet;
use anyhow::anyhow;

/// sor acceleration constant
const SOR_ACC: f64 = 1.4;

/// gauss-seidel iterations between convergence check
const CONV_CHECK_ITER: u64 = 25;

/// gauss-seidel max iterations
const GS_MAX_ITER: u64 = 10000;

/// gauss-seidel tolerance
const GS_TOL: f64 = 1e-5;

/// `Electrostatic` struct
///
/// an electrostatic pic engine
#[derive(Debug)]
pub struct Electrostatic {
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

impl Electrostatic {
    /// `Electrostatic` constructor
    ///
    /// # Arguments
    /// - `size`: &[f64; 3] (m) size of bounding box
    /// - `cells`: &[usize; 3] number of cells
    ///
    /// # Returns
    /// `Result<Electrostatic, anyhow::Error>`
    ///
    /// # Errors
    /// - any call to `CoordinateTriplet::new()` fails
    /// - any call to `ScalarField::new()` fails
    /// - any call to `VectorField::new()` fails
    pub fn new(size: &[f64; 3], cells: &[usize; 3]) -> Result<Electrostatic, anyhow::Error> {
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

        Ok(Electrostatic {
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

    pub fn update(&mut self) -> Result<(), anyhow::Error> {
        Self::solve_potential(self)?;
        Self::solve_electric_field(self)?;
        Ok(())
    }

    fn solve_potential(&mut self) -> Result<(), anyhow::Error> {
        // loop counter
        let mut loop_ctr: u64 = 0;

        // l2 error norm
        let mut l2_err_norm: f64 = f64::MAX;

        // gauss-seidel sor scheme loop
        while l2_err_norm > GS_TOL {
            // update potential on interior nodes
            for i in 1..(self.cells.x - 1) {
                for j in 1..(self.cells.y - 1) {
                    for k in 1..(self.cells.z - 1) {
                        // solve potential using gauss-seidel
                        let potential_new: f64 = (self.charge_density[(i, j, k)] * INV_VAC_PERM
                            + self.delta_inv_sq.x
                                * (self.potential[(i + 1, j, k)] + self.potential[(i - 1, j, k)])
                            + self.delta_inv_sq.y
                                * (self.potential[(i, j + 1, k)] - self.potential[(i, j - 1, k)])
                            + self.delta_inv_sq.z
                                * (self.potential[(i, j, k + 1)] - self.potential[(i, j, k - 1)]))
                            / (2.0
                                * (self.delta_inv_sq.x
                                    + self.delta_inv_sq.y
                                    + self.delta_inv_sq.z));

                        // apply sor
                        self.potential[(i, j, k)] +=
                            SOR_ACC * (potential_new - self.potential[(i, j, k)]);
                    }
                }
            }

            // conditionally check for convergence
            if (loop_ctr % CONV_CHECK_ITER) == 0 {
                // residue accumulator
                let mut res_acc: f64 = 0.0;

                // accumulate residue = Ax - b
                for i in 1..(self.cells.x - 1) {
                    for j in 1..(self.cells.y - 1) {
                        for k in 1..(self.cells.z - 1) {
                            // residue vector value
                            let res = -self.potential[(i, j, k)]
                                * 2.0
                                * (self.delta_inv_sq.x + self.delta_inv_sq.y + self.delta_inv_sq.z)
                                + self.charge_density[(i, j, k)] * INV_VAC_PERM
                                + self.delta_inv_sq.x
                                    * (self.potential[(i + 1, j, k)]
                                        + self.potential[(i - 1, j, k)])
                                + self.delta_inv_sq.y
                                    * (self.potential[(i, j + 1, k)]
                                        - self.potential[(i, j - 1, k)])
                                + self.delta_inv_sq.z
                                    * (self.potential[(i, j, k + 1)]
                                        - self.potential[(i, j, k - 1)]);

                            res_acc += res * res;
                        }
                    }
                }
                // update l2 error norm
                l2_err_norm =
                    (res_acc / (self.cells.x * self.cells.y * self.cells.z) as f64).sqrt();
            }

            // error if convergence is not met
            if loop_ctr == GS_MAX_ITER {
                return Err(anyhow!("solution to potential did not converge to tolerance of {GS_TOL} in {GS_MAX_ITER} iterations"));
            }

            // increment loop counter
            loop_ctr += 1;
        }

        Ok(())
    }

    fn solve_electric_field(&mut self) -> Result<(), anyhow::Error> {
        // precompute negative inverses
        let n_two_dx_inv = -1.0 / (2.0 * self.delta.x);
        let n_two_dy_inv = -1.0 / (2.0 * self.delta.y);
        let n_two_dz_inv = -1.0 / (2.0 * self.delta.z);

        for i in 0..self.cells.x {
            for j in 0..self.cells.y {
                for k in 0..self.cells.z {
                    // x-component
                    if i != 0 && i != self.cells.x - 1 {
                        // central difference interior nodes
                        self.electric_field.x[(i, j, k)] = n_two_dx_inv
                            * (self.potential[(i + 1, j, k)] - self.potential[(i - 1, j, k)]);
                    } else if i == 0 {
                        // forward difference low edge
                        self.electric_field.x[(i, j, k)] = n_two_dx_inv
                            * (-3.0 * self.potential[(i, j, k)]
                                + 4.0 * self.potential[(i + 1, j, k)]
                                - self.potential[(i + 2, j, k)]);
                    } else {
                        // backward difference high edge
                        self.electric_field.x[(i, j, k)] = n_two_dx_inv
                            * (self.potential[(i - 2, j, k)] - 4.0 * self.potential[(i - 1, j, k)]
                                + 3.0 * self.potential[(i, j, k)]);
                    }

                    // y-component
                    if j != 0 && j != self.cells.y - 1 {
                        // central difference interior nodes
                        self.electric_field.y[(i, j, k)] = n_two_dy_inv
                            * (self.potential[(i, j + 1, k)] - self.potential[(i, j - 1, k)]);
                    } else if j == 0 {
                        // forward difference low edge
                        self.electric_field.y[(i, j, k)] = n_two_dy_inv
                            * (-3.0 * self.potential[(i, j, k)]
                                + 4.0 * self.potential[(i, j + 1, k)]
                                - self.potential[(i, j + 2, k)]);
                    } else {
                        // backward difference high edge
                        self.electric_field.y[(i, j, k)] = n_two_dy_inv
                            * (self.potential[(i, j - 2, k)] - 4.0 * self.potential[(i, j - 1, k)]
                                + 3.0 * self.potential[(i, j, k)]);
                    }

                    // z-component
                    if k != 0 && k != self.cells.z - 1 {
                        // central difference interior nodes
                        self.electric_field.z[(i, j, k)] = n_two_dz_inv
                            * (self.potential[(i, j, k + 1)] - self.potential[(i, j, k - 1)]);
                    } else if k == 0 {
                        // forward difference low edge
                        self.electric_field.z[(i, j, k)] = n_two_dz_inv
                            * (-3.0 * self.potential[(i, j, k)]
                                + 4.0 * self.potential[(i, j, k + 1)]
                                - self.potential[(i, j, k + 2)]);
                    } else {
                        // backward difference high edge
                        self.electric_field.z[(i, j, k)] = n_two_dz_inv
                            * (self.potential[(i, j, k - 2)] - 4.0 * self.potential[(i, j, k - 1)]
                                + 3.0 * self.potential[(i, j, k)]);
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::Electrostatic;
    use crate::field::scalar::ScalarField;
    use crate::field::vector::VectorField;
    use crate::utils::coordinate_triplet::CoordinateTriplet;

    /// helper function that sets up a `Electrostatic` for testing
    ///
    /// # Arguments
    ///
    /// # Returns
    /// `Result<Electrostatic, anyhow::Error>`
    ///
    /// # Errors
    ///
    fn setup() -> Result<Electrostatic, anyhow::Error> {
        let size: [f64; 3] = [1.0, 2.0, 3.0];
        let cells: [usize; 3] = [3, 11, 31];
        Electrostatic::new(&size, &cells)
    }

    /// tests `Electrostatic::new()` for success
    ///
    /// # Errors
    /// - `Electrostatic::new()` fails for valid input
    ///
    #[test]
    fn new_success() {
        assert!(setup().is_ok());
    }

    /// tests `Electrostatic::new()` for correct setting of `Electrostatic.size` member
    ///
    /// # Errors
    /// - `Electrostatic::new()` sets incorrect `engine.size.x`
    /// - `Electrostatic::new()` sets incorrect `engine.size.y`
    /// - `Electrostatic::new()` sets incorrect `engine.size.z`
    ///
    #[test]
    fn new_correct_size() {
        // setup
        let electrostatic = setup().unwrap();

        // assertions
        assert_eq!(electrostatic.size.x, 1.0);
        assert_eq!(electrostatic.size.y, 2.0);
        assert_eq!(electrostatic.size.z, 3.0);
    }

    /// tests `Electrostatic::new()` for correct setting of `Electrostatic.cells' member
    ///
    /// # Errors
    /// - `Electrostatic::new()` sets incorrect `engine.cells.x`
    /// - `Electrostatic::new()` sets incorrect `engine.cells.y`
    /// - `Electrostatic::new()` sets incorrect `engine.cells.z`
    ///
    #[test]
    fn new_correct_cells() {
        // setup
        let electrostatic = setup().unwrap();

        // assertions
        assert_eq!(electrostatic.cells.x, 3);
        assert_eq!(electrostatic.cells.y, 11);
        assert_eq!(electrostatic.cells.z, 31);
    }

    /// tests `Electrostatic::new()` for correct setting of `Electrostatic.delta` member
    ///
    /// # Errors
    /// - `Electrostatic::new()` sets incorrect `engine.delta.x`
    /// - `Electrostatic::new()` sets incorrect `engine.delta.y`
    /// - `Electrostatic::new()` sets incorrect `engine.delta.z`
    ///
    #[test]
    fn new_correct_delta() {
        // setup
        let electrostatic = setup().unwrap();

        // assertions
        assert_eq!(electrostatic.delta.x, 0.5);
        assert_eq!(electrostatic.delta.y, 0.2);
        assert_eq!(electrostatic.delta.z, 0.1);
    }

    /// tests `Electrostatic::new()` for correct setting of `Electrostatic.potential` member
    ///
    /// # Errors
    /// - `Electrostatic::new()` sets incorrect `Electrostatic.potential`
    /// - call to `CoordinateTriplet::new()` fails
    /// - call to `ScalarField::new()` fails
    ///
    #[test]
    fn new_correct_potential() {
        // setup
        let electrostatic = setup().unwrap();
        let cells = CoordinateTriplet::new(3, 11, 31).unwrap();

        // assertions
        assert_eq!(electrostatic.potential, ScalarField::new(&cells).unwrap())
    }

    /// tests `Electrostatic::new()` for correct setting of `Electrostatic.charge_density` member
    ///
    /// # Errors
    /// - `Electrostatic::new()` sets incorrect `Electrostatic.charge_density`
    /// - call to `CoordinateTriplet::new()` fails
    /// - call to `ScalarField::new()` fails
    ///
    #[test]
    fn new_correct_charge_density() {
        // setup
        let electrostatic = setup().unwrap();
        let cells = CoordinateTriplet::new(3, 11, 31).unwrap();

        // assertions
        assert_eq!(
            electrostatic.charge_density,
            ScalarField::new(&cells).unwrap()
        )
    }

    /// tests `Electrostatic::new()` for correct setting of `Electrostatic.electric_field` member
    ///
    /// # Errors
    /// - `Electrostatic::new()` sets incorrect `Electrostatic.electric_field`
    /// - call to `CoordinateTriplet::new()` fails
    /// - call to `VectorField::new()` fails
    ///
    #[test]
    fn new_correct_electric_field() {
        // setup
        let electrostatic = setup().unwrap();
        let cells = CoordinateTriplet::new(3, 11, 31).unwrap();

        // assertions
        assert_eq!(
            electrostatic.electric_field,
            VectorField::new(&cells).unwrap()
        )
    }

    /// tests `Electrostatic::new()` for correct setting of `Electrostatic.cell_vol` member
    ///
    /// # Errors
    /// - `Electrostatic::new()` sets incorrect `Electrostatic.cell_vol`
    /// - call to `CoordinateTriplet::new()` fails
    /// - call to `ScalarField::new()` fails
    ///
    #[test]
    fn new_correct_cell_vol() {
        // setup
        let electrostatic = setup().unwrap();
        let cells = CoordinateTriplet::new(3, 11, 31).unwrap();

        // assertions
        assert_eq!(electrostatic.cell_vol, ScalarField::new(&cells).unwrap())
    }

    /// tests `Electrostatic::new()` for correct setting of `Electrostatic.delta_inv_sq` member
    ///
    /// # Errors
    /// - `Electrostatic::new()` sets incorrect `Electrostatic.delta_inv_sq`
    /// - call to `CoordinateTriplet::new()` fails
    ///
    #[test]
    fn new_correct_delta_inv_sq() {
        // setup
        let electrostatic = setup().unwrap();

        // assertions
        assert_eq!(
            electrostatic.delta_inv_sq.x,
            1.0 / (electrostatic.delta.x * electrostatic.delta.x)
        );
        assert_eq!(
            electrostatic.delta_inv_sq.y,
            1.0 / (electrostatic.delta.y * electrostatic.delta.y)
        );
        assert_eq!(
            electrostatic.delta_inv_sq.z,
            1.0 / (electrostatic.delta.z * electrostatic.delta.z)
        );
    }
}
