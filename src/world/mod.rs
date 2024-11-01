//! world module
//!
//! contents describe world of the simulation domain

use num::Zero;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

/// `Mesh` struct
///
/// mesh of the simulation domain
#[derive(Debug)]
struct Mesh {
    /// (m) size of bounding box
    size: CoordinateTriplet<f64>,

    /// number of cells
    cells: CoordinateTriplet<u64>,

    /// (m) spatial increment
    delta: CoordinateTriplet<f64>,
}

impl Mesh {
    /// `Mesh` constructor
    ///
    /// # Arguments
    /// - `size`: &[f64; 3] (m) size of bounding box
    /// - `cells`: &[u64; 3] number of cells
    ///
    /// # Returns
    /// `Result<Mesh, anyhow::Error>`
    ///
    /// # Errors
    /// - any call to `CoordinateTriplet::new()` fails
    pub fn new(size: &[f64; 3], cells: &[u64; 3]) -> Result<Mesh, anyhow::Error> {
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

        Ok(Mesh { size, cells, delta })
    }
}

/// `CoordinateTriplet` struct
///
/// represents generic data that by nature has (x, y, z) components
#[derive(Debug)]
struct CoordinateTriplet<T> {
    /// x component
    x: T,

    /// y component
    y: T,

    /// z component
    z: T,
}

impl<T> CoordinateTriplet<T> {
    /// `CoordinateTriplet` constructor
    ///
    /// # Arguments
    /// - `x`: T x component
    /// - `y`: T y component
    /// - `z`: T z component
    ///
    /// # Returns
    /// `Result<CoordinateTriplet<T>, anyhow::Error>`
    ///
    /// # Errors
    ///
    pub fn new(x: T, y: T, z: T) -> Result<CoordinateTriplet<T>, anyhow::Error> {
        Ok(CoordinateTriplet { x, y, z })
    }
}

/// `ScalarField` struct
///
/// describes a scalar field
#[derive(Debug)]
pub struct ScalarField<T: Zero + Copy> {
    data: Vec<T>,
    size: [usize; 3],
    r_offset: usize,
    p_offset: usize,
}

impl<T: Zero + Copy> ScalarField<T> {
    pub fn new(
        x_cells: usize,
        y_cells: usize,
        z_cells: usize,
    ) -> Result<ScalarField<T>, anyhow::Error> {
        let size = [x_cells, y_cells, z_cells];
        let r_offset = x_cells;
        let p_offset = x_cells * y_cells;

        let data: Vec<T> = vec![T::zero(); size[0] * size[1] * size[2]];
        Ok(ScalarField {
            data,
            size,
            r_offset,
            p_offset,
        })
    }
}

impl<T: Zero + Copy + Display> Index<(usize, usize, usize)> for ScalarField<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize, usize)) -> &Self::Output {
        let (i, j, k) = idx;
        &self.data[i + self.r_offset * j + self.p_offset * k]
    }
}

impl<T: Zero + Copy + Display> IndexMut<(usize, usize, usize)> for ScalarField<T> {
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        let (i, j, k) = index;
        &mut self.data[i + self.r_offset * j + self.p_offset * k]
    }
}

impl<T: Zero + Copy + Display> Display for ScalarField<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.size[0] {
            for j in 0..self.size[1] {
                for k in 0..self.size[2] {
                    write!(
                        f,
                        "ScalarField({}, {}, {}) = {}\n",
                        i,
                        j,
                        k,
                        self[(i, j, k)]
                    )?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct VectorField<T: Zero + Copy + Display> {
    data: Vec<[T; 3]>,
    size: [usize; 3],
    r_offset: usize,
    p_offset: usize,
}

impl<T: Zero + Copy + Display> VectorField<T> {
    pub fn new(
        x_cells: usize,
        y_cells: usize,
        z_cells: usize,
    ) -> Result<VectorField<T>, anyhow::Error> {
        let size = [x_cells, y_cells, z_cells];
        let r_offset = x_cells;
        let p_offset = x_cells * y_cells;

        let data: Vec<[T; 3]> = vec![[T::zero(); 3]; size[0] * size[1] * size[2]];
        Ok(VectorField {
            data,
            size,
            r_offset,
            p_offset,
        })
    }
}

impl<T: Zero + Copy + Display> Index<(usize, usize, usize, usize)> for VectorField<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize, usize, usize)) -> &Self::Output {
        let (i, j, k, dir) = idx;
        &self.data[i + self.r_offset * j + self.p_offset * k][dir]
    }
}

impl<T: Zero + Copy + Display> IndexMut<(usize, usize, usize, usize)> for VectorField<T> {
    fn index_mut(&mut self, index: (usize, usize, usize, usize)) -> &mut Self::Output {
        let (i, j, k, dir) = index;
        &mut self.data[i + self.r_offset * j + self.p_offset * k][dir]
    }
}

impl<T: Zero + Copy + Display> Display for VectorField<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.size[0] {
            for j in 0..self.size[1] {
                for k in 0..self.size[2] {
                    write!(
                        f,
                        "VectorField({}, {}, {}) = [{}, {}, {}]\n",
                        i,
                        j,
                        k,
                        self[(i, j, k, 0)],
                        self[(i, j, k, 1)],
                        self[(i, j, k, 2)]
                    )?;
                }
            }
        }
        Ok(())
    }
}