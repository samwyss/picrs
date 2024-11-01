//! world module
//!
//! contents describe world of the simulation domain

use num::Num;
use std::fmt::{Display, Formatter};
use std::ops::{DivAssign, Index, IndexMut};

/// `CoordinateTriplet` struct
///
/// represents generic data that by nature has (x, y, z) components
#[derive(Debug, Clone)]
pub struct CoordinateTriplet<T> {
    /// x component
    pub x: T,

    /// y component
    pub y: T,

    /// z component
    pub z: T,
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

/// `ScalarField` struct
///
/// describes a scalar field
#[derive(Debug)]
pub struct ScalarField<T: Num + Copy> {
    /// scalar field data
    data: Vec<T>,

    /// number of cells in scalar field
    cells: CoordinateTriplet<usize>,

    /// scalar field row offset
    r_offset: usize,

    /// scalar field plane offset
    p_offset: usize,
}

impl<T: Num + Copy> ScalarField<T> {
    /// `ScalarField` constructor
    ///
    /// # Arguments
    /// - `cells`: CoordinateTriplet<usize> number of cells in bounding box
    ///
    /// # Returns
    /// `Result<ScalarField<T>, anyhow::Error>`
    ///
    /// # Errors
    ///
    pub fn new(cells: CoordinateTriplet<usize>) -> Result<ScalarField<T>, anyhow::Error> {
        // define offsets
        let r_offset = cells.x;
        let p_offset = cells.x * cells.y;

        // define initial vector field
        let data: Vec<T> = vec![T::zero(); cells.x * cells.y * cells.z];

        Ok(ScalarField {
            data,
            cells,
            r_offset,
            p_offset,
        })
    }
}

impl<T: Num + Copy + Display> Index<(usize, usize, usize)> for ScalarField<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize, usize)) -> &Self::Output {
        let (i, j, k) = idx;
        &self.data[i + self.r_offset * j + self.p_offset * k]
    }
}

impl<T: Num + Copy + Display> IndexMut<(usize, usize, usize)> for ScalarField<T> {
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        let (i, j, k) = index;
        &mut self.data[i + self.r_offset * j + self.p_offset * k]
    }
}

impl<T: Num + Copy + Display> Display for ScalarField<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.cells.x {
            for j in 0..self.cells.y {
                for k in 0..self.cells.z {
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

impl<T: Num + Copy + Display> DivAssign<ScalarField<T>> for ScalarField<T> {
    fn div_assign(&mut self, rhs: Self) {
        for i in 0..self.cells.x {
            for j in 0..self.cells.y {
                for k in 0..self.cells.z {
                    self[(i, j, k)] /= rhs[(i, j, k)];
                }
            }
        }
    }
}

/// `VectorField` struct
///
/// describes a vector field
#[derive(Debug)]
pub struct VectorField<T: Num + Copy + Display> {
    /// vector field data
    data: Vec<CoordinateTriplet<T>>,

    /// number of cells in vector field
    cells: CoordinateTriplet<usize>,

    /// vector field row offset
    r_offset: usize,

    /// vector field plane offset
    p_offset: usize,
}

impl<T: Num + Copy + Display> VectorField<T> {
    /// `VectorField` constructor
    ///
    /// # Arguments
    /// - `cells`: CoordinateTriplet<usize> number of cells in bounding box
    ///
    /// # Returns
    /// `Result<VectorField<T>, anyhow::Error>`
    ///
    /// # Errors
    /// - any call to `CoordinateTriplet::new()` fails
    pub fn new(cells: CoordinateTriplet<usize>) -> Result<VectorField<T>, anyhow::Error> {
        // define offsets
        let r_offset = cells.x;
        let p_offset = cells.x * cells.y;

        // define initial vector field
        let data: Vec<CoordinateTriplet<T>> =
            vec![
                CoordinateTriplet::new(T::zero(), T::zero(), T::zero())?;
                cells.x * cells.y * cells.z
            ];

        Ok(VectorField {
            data,
            cells,
            r_offset,
            p_offset,
        })
    }
}

impl<T: Num + Copy + Display> Index<(usize, usize, usize)> for VectorField<T> {
    type Output = CoordinateTriplet<T>;

    fn index(&self, idx: (usize, usize, usize)) -> &Self::Output {
        let (i, j, k) = idx;
        &self.data[i + self.r_offset * j + self.p_offset * k]
    }
}

impl<T: Num + Copy + Display> IndexMut<(usize, usize, usize)> for VectorField<T> {
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        let (i, j, k) = index;
        &mut self.data[i + self.r_offset * j + self.p_offset * k]
    }
}

impl<T: Num + Copy + Display> Display for VectorField<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.cells.x {
            for j in 0..self.cells.y {
                for k in 0..self.cells.z {
                    write!(
                        f,
                        "VectorField({}, {}, {}) = [{}, {}, {}]\n",
                        i,
                        j,
                        k,
                        self[(i, j, k)].x,
                        self[(i, j, k)].y,
                        self[(i, j, k)].z
                    )?;
                }
            }
        }
        Ok(())
    }
}
