use crate::helpers::coordinate_triplet::CoordinateTriplet;
use num::Num;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, DivAssign, Index, IndexMut, MulAssign, SubAssign};

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
    pub fn new(cells: &CoordinateTriplet<usize>) -> Result<ScalarField<T>, anyhow::Error> {
        // clone cells
        let cells = cells.clone();

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

    /// returns an iterator over `ScalarField`
    ///
    /// # Arguments
    /// - `&'a self` reference to self
    ///
    /// # Returns
    /// `impl Iterator<Item = &'a T> + 'a`
    ///
    /// # Errors
    ///
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        self.data.iter()
    }
    
    /// returns a mutable iterator over `ScalarField`
    ///
    /// # Arguments
    /// - `&'a mut self` mutable reference to self
    ///
    /// # Returns
    /// `impl Iterator<Item = &'a mut T> + 'a`
    ///
    /// # Errors
    ///
    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> + 'a {
        self.data.iter_mut()
    }
}

/// implements [] operator on `ScalarField`
impl<T> Index<(usize, usize, usize)> for ScalarField<T> {
    type Output = T;
    
    /// returns a reference to scalar field data stored at desired index
    ///
    /// # Arguments
    /// - `&self` reference to self
    /// - `idx: (usize, usize, usize)` i, j, and k indices on cartesian grid
    ///
    /// # Returns
    /// `&T`
    ///
    /// # Errors
    ///
    fn index(&self, idx: (usize, usize, usize)) -> &Self::Output {
        // destructure idx into i, j, and k components
        let (i, j, k) = idx;
        
        // linearly index into `ScalarField` using column major ordering
        &self.data[i + self.r_offset * j + self.p_offset * k]
    }
}

/// implements mutable [] operator on `ScalarField`
impl<T> IndexMut<(usize, usize, usize)> for ScalarField<T> {
    /// returns a mutable reference to scalar field data stored at desired index
    ///
    /// # Arguments
    /// - `&mut self` mutable reference to self
    /// - `idx: (usize, usize, usize)` i, j, and k indices on cartesian grid
    ///
    /// # Returns
    /// `&mut T`
    ///
    /// # Errors
    ///
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        // destructure idx into i, j, and k components
        let (i, j, k) = index;
        
        // linearly index into `ScalarField` using column major ordering
        &mut self.data[i + self.r_offset * j + self.p_offset * k]
    }
}

/// allows `ScalarField` to be written in a text format
impl<T: Display> Display for ScalarField<T> {
    /// writes `ScalarField` in a text format
    ///
    /// # Arguments
    /// - `&self` reference to self
    /// - `f: &mut Formatter<'_>` formatter for writing
    ///
    /// # Returns
    /// `std::fmt::Result`
    ///
    /// # Errors
    /// - call to `write!()` errors
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

/// implements ScalarField<T> += ScalarField<T>
impl<T: Copy + AddAssign> AddAssign<ScalarField<T>> for ScalarField<T> {
    /// implements ScalarField<T> += ScalarField<T>
    ///
    /// # Arguments
    /// - `&mut self` mutable reference to self
    /// - `rhs: ScalarField<T>` rhs of operation
    ///
    /// # Returns
    ///
    /// # Errors
    ///
    fn add_assign(&mut self, rhs: ScalarField<T>) {
        for (elem, num) in self.data.iter_mut().zip(&rhs.data) {
            *elem += *num;
        }
    }
}

/// implements ScalarField<T> -= ScalarField<T>
impl<T: Copy + SubAssign> SubAssign<ScalarField<T>> for ScalarField<T> {
    /// implements ScalarField<T> -= ScalarField<T>
    ///
    /// # Arguments
    /// - `&mut self` mutable reference to self
    /// - `rhs: ScalarField<T>` rhs of operation
    ///
    /// # Returns
    ///
    /// # Errors
    ///
    fn sub_assign(&mut self, rhs: ScalarField<T>) {
        for (elem, num) in self.data.iter_mut().zip(&rhs.data) {
            *elem -= *num;
        }
    }
}

/// implements ScalarField<T> *= ScalarField<T>
impl<T: Copy + MulAssign> MulAssign<ScalarField<T>> for ScalarField<T> {
    /// implements ScalarField<T> *= ScalarField<T>
    ///
    /// # Arguments
    /// - `&mut self` mutable reference to self
    /// - `rhs: ScalarField<T>` rhs of operation
    ///
    /// # Returns
    ///
    /// # Errors
    ///
    fn mul_assign(&mut self, rhs: ScalarField<T>) {
        for (elem, num) in self.data.iter_mut().zip(&rhs.data) {
            *elem *= *num;
        }
    }
}

/// implements ScalarField<T> /= ScalarField<T>
impl<T: Copy + DivAssign> DivAssign<ScalarField<T>> for ScalarField<T> {
    /// implements ScalarField<T> /= ScalarField<T>
    ///
    /// # Arguments
    /// - `&mut self` mutable reference to self
    /// - `rhs: ScalarField<T>` rhs of operation
    ///
    /// # Returns
    ///
    /// # Errors
    ///
    fn div_assign(&mut self, rhs: ScalarField<T>) {
        for (elem, num) in self.data.iter_mut().zip(&rhs.data) {
            *elem /= *num;
        }
    }
}

/// implements ScalarField<T> += T
impl<T: Copy + AddAssign> AddAssign<T> for ScalarField<T> {
    /// implements ScalarField<T> += T
    ///
    /// # Arguments
    /// - `&mut self` mutable reference to self
    /// - `rhs: T` rhs of operation
    ///
    /// # Returns
    ///
    /// # Errors
    ///
    fn add_assign(&mut self, rhs: T) {
        for elem in self.data.iter_mut() {
            *elem += rhs;
        }
    }
}

/// implements ScalarField<T> -= T
impl<T: Copy + SubAssign> SubAssign<T> for ScalarField<T> {
    /// implements ScalarField<T> -= T
    ///
    /// # Arguments
    /// - `&mut self` mutable reference to self
    /// - `rhs: T` rhs of operation
    ///
    /// # Returns
    ///
    /// # Errors
    ///
    fn sub_assign(&mut self, rhs: T) {
        for elem in self.data.iter_mut() {
            *elem -= rhs;
        }
    }
}

/// implements ScalarField<T> *= T
impl<T: Copy + MulAssign> MulAssign<T> for ScalarField<T> {
    /// implements ScalarField<T> *= T
    ///
    /// # Arguments
    /// - `&mut self` mutable reference to self
    /// - `rhs: T` rhs of operation
    ///
    /// # Returns
    ///
    /// # Errors
    ///
    fn mul_assign(&mut self, rhs: T) {
        for elem in self.data.iter_mut() {
            *elem *= rhs;
        }
    }
}

/// implements ScalarField<T> /= T
impl<T: Copy + DivAssign> DivAssign<T> for ScalarField<T> {
    /// implements ScalarField<T> /= T
    ///
    /// # Arguments
    /// - `&mut self` mutable reference to self
    /// - `rhs: T` rhs of operation
    ///
    /// # Returns
    ///
    /// # Errors
    ///
    fn div_assign(&mut self, rhs: T) {
        for elem in self.data.iter_mut() {
            *elem /= rhs;
        }
    }
}

#[cfg(test)]
mod tests {
    
}