use crate::world::CoordinateTriplet;
use num::Num;
use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, DivAssign, Index, IndexMut, MulAssign, SubAssign};

/// `ScalarField` struct
///
/// describes a scalar field
#[derive(Debug)]
pub struct ScalarField<T> {
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

impl<T> Index<(usize, usize, usize)> for ScalarField<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize, usize)) -> &Self::Output {
        let (i, j, k) = idx;
        &self.data[i + self.r_offset * j + self.p_offset * k]
    }
}

impl<T> IndexMut<(usize, usize, usize)> for ScalarField<T> {
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        let (i, j, k) = index;
        &mut self.data[i + self.r_offset * j + self.p_offset * k]
    }
}

impl<T: Display> Display for ScalarField<T> {
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

impl<T: Copy + AddAssign> AddAssign<ScalarField<T>> for ScalarField<T> {
    fn add_assign(&mut self, rhs: ScalarField<T>) {
        for (elem, num) in self.data.iter_mut().zip(&rhs.data) {
            *elem += *num;
        }
    }
}

impl<T: Copy + SubAssign> SubAssign<ScalarField<T>> for ScalarField<T> {
    fn sub_assign(&mut self, rhs: ScalarField<T>) {
        for (elem, num) in self.data.iter_mut().zip(&rhs.data) {
            *elem -= *num;
        }
    }
}

impl<T: Copy + MulAssign> MulAssign<ScalarField<T>> for ScalarField<T> {
    fn mul_assign(&mut self, rhs: ScalarField<T>) {
        for (elem, num) in self.data.iter_mut().zip(&rhs.data) {
            *elem *= *num;
        }
    }
}

impl<T: Copy + DivAssign> DivAssign<ScalarField<T>> for ScalarField<T> {
    fn div_assign(&mut self, rhs: Self) {
        for (elem, num) in self.data.iter_mut().zip(&rhs.data) {
            *elem /= *num;
        }
    }
}

impl<T: Copy + AddAssign> AddAssign<T> for ScalarField<T> {
    fn add_assign(&mut self, rhs: T) {
        for elem in self.data.iter_mut() {
            *elem += rhs;
        }
    }
}

impl<T: Copy + SubAssign> SubAssign<T> for ScalarField<T> {
    fn sub_assign(&mut self, rhs: T) {
        for elem in self.data.iter_mut() {
            *elem -= rhs;
        }
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for ScalarField<T> {
    fn mul_assign(&mut self, rhs: T) {
        for elem in self.data.iter_mut() {
            *elem *= rhs;
        }
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for ScalarField<T> {
    fn div_assign(&mut self, rhs: T) {
        for elem in self.data.iter_mut() {
            *elem /= rhs;
        }
    }
}

/// `VectorField` struct
///
/// describes a vector field
#[derive(Debug)]
pub struct VectorField<T> {
    /// vector field data
    data: Vec<CoordinateTriplet<T>>,

    /// number of cells in vector field
    cells: CoordinateTriplet<usize>,

    /// vector field row offset
    r_offset: usize,

    /// vector field plane offset
    p_offset: usize,
}

impl<T: Num + Copy> VectorField<T> {
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

impl<T> Index<(usize, usize, usize)> for VectorField<T> {
    type Output = CoordinateTriplet<T>;

    fn index(&self, idx: (usize, usize, usize)) -> &Self::Output {
        let (i, j, k) = idx;
        &self.data[i + self.r_offset * j + self.p_offset * k]
    }
}

impl<T> IndexMut<(usize, usize, usize)> for VectorField<T> {
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        let (i, j, k) = index;
        &mut self.data[i + self.r_offset * j + self.p_offset * k]
    }
}

impl<T: Display> Display for VectorField<T> {
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

impl<T: Copy + AddAssign> AddAssign<VectorField<T>> for VectorField<T> {
    fn add_assign(&mut self, rhs: VectorField<T>) {
        for (elem, num) in self.data.iter_mut().zip(&rhs.data) {
            elem.x += num.x;
            elem.y += num.y;
            elem.z += num.z;
        }
    }
}

impl<T: Copy + SubAssign> SubAssign<VectorField<T>> for VectorField<T> {
    fn sub_assign(&mut self, rhs: VectorField<T>) {
        for (elem, num) in self.data.iter_mut().zip(&rhs.data) {
            elem.x -= num.x;
            elem.y -= num.y;
            elem.z -= num.z;
        }
    }
}

impl<T: Copy + MulAssign> MulAssign<VectorField<T>> for VectorField<T> {
    fn mul_assign(&mut self, rhs: VectorField<T>) {
        for (elem, num) in self.data.iter_mut().zip(&rhs.data) {
            elem.x *= num.x;
            elem.y *= num.y;
            elem.z *= num.z;
        }
    }
}

impl<T: Copy + DivAssign> DivAssign<VectorField<T>> for VectorField<T> {
    fn div_assign(&mut self, rhs: VectorField<T>) {
        for (elem, num) in self.data.iter_mut().zip(&rhs.data) {
            elem.x /= num.x;
            elem.y /= num.y;
            elem.z /= num.z;
        }
    }
}

impl<T: Copy + AddAssign> AddAssign<T> for VectorField<T> {
    fn add_assign(&mut self, rhs: T) {
        for elem in self.data.iter_mut() {
            elem.x += rhs;
            elem.y += rhs;
            elem.z += rhs;
        }
    }
}

impl<T: Copy + SubAssign> SubAssign<T> for VectorField<T> {
    fn sub_assign(&mut self, rhs: T) {
        for elem in self.data.iter_mut() {
            elem.x -= rhs;
            elem.y -= rhs;
            elem.z -= rhs;
        }
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for VectorField<T> {
    fn mul_assign(&mut self, rhs: T) {
        for elem in self.data.iter_mut() {
            elem.x *= rhs;
            elem.y *= rhs;
            elem.z *= rhs;
        }
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for VectorField<T> {
    fn div_assign(&mut self, rhs: T) {
        for elem in self.data.iter_mut() {
            elem.x /= rhs;
            elem.y /= rhs;
            elem.z /= rhs;
        }
    }
}
