use crate::field::scalar::ScalarField;
use crate::world::CoordinateTriplet;
use num::Num;
use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, DivAssign, Index, IndexMut, MulAssign, SubAssign};

/// `VectorField` struct
///
/// describes a vector field
#[derive(Debug)]
pub struct VectorField<T> {
    /// number of cells in vector field
    cells: CoordinateTriplet<usize>,

    /// x component of vector field
    x: ScalarField<T>,

    /// y component of vector field
    y: ScalarField<T>,

    /// z component of vector field
    z: ScalarField<T>,
}

impl<T: Num + Copy> VectorField<T> {
    /// `VectorField` constructor
    ///
    /// # Arguments
    /// - `cells`: &CoordinateTriplet<usize> number of cells in bounding box
    ///
    /// # Returns
    /// `Result<VectorField<T>, anyhow::Error>`
    ///
    /// # Errors
    /// - any call to `ScalarField::new()` errors
    pub fn new(cells: &CoordinateTriplet<usize>) -> Result<VectorField<T>, anyhow::Error> {
        let cells = cells.clone();

        // create subfields
        let x = ScalarField::new(&cells)?;
        let y = ScalarField::new(&cells)?;
        let z = ScalarField::new(&cells)?;

        Ok(VectorField { cells, x, y, z })
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
                        "VectorField({}, {}, {}) = {}\n",
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
