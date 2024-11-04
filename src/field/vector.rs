use crate::field::scalar::ScalarField;
use crate::helpers::coordinate_triplet::CoordinateTriplet;
use num::Num;
use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, DivAssign, Index, IndexMut, MulAssign, SubAssign};

/// `VectorField<T>` struct
///
/// describes a vector field
#[derive(Debug)]
pub struct VectorField<T> {
    /// number of cells in vector field
    cells: CoordinateTriplet<usize>,

    /// x component of vector field
    pub x: ScalarField<T>,

    /// y component of vector field
    pub y: ScalarField<T>,

    /// z component of vector field
    pub z: ScalarField<T>,
}

impl<T: Num + Copy> VectorField<T> {
    /// `VectorField<T>` constructor
    ///
    /// # Arguments
    /// - `cells: &CoordinateTriplet<usize>` number of cells in bounding box
    ///
    /// # Returns
    /// `Result<VectorField<T>, anyhow::Error>`
    ///
    /// # Errors
    /// - any call to `ScalarField::new()` errors
    pub fn new(cells: &CoordinateTriplet<usize>) -> Result<VectorField<T>, anyhow::Error> {
        // clone cells
        let cells = cells.clone();

        // create subfields
        let x = ScalarField::new(&cells)?;
        let y = ScalarField::new(&cells)?;
        let z = ScalarField::new(&cells)?;

        Ok(VectorField { cells, x, y, z })
    }
}

/// allows `VectorField<T>` to be written in a text format
impl<T: Display> Display for VectorField<T> {
    /// writes `VectorField<T>` in a text format
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
                        "VectorField({}, {}, {}) = [{}, {}, {}]\n",
                        i,
                        j,
                        k,
                        self.x[(i, j, k)],
                        self.y[(i, j, k)],
                        self.z[(i, k, j)],
                    )?;
                }
            }
        }
        Ok(())
    }
}

/// implements VectorField<T> += VectorField<T>
impl<T: Copy + AddAssign + Num> AddAssign<VectorField<T>> for VectorField<T> {
    /// implements VectorField<T> += VectorField<T>
    ///
    /// # Arguments
    /// - `&mut self` mutable reference to self
    /// - `rhs: VectorField<T>` rhs of operation
    ///
    /// # Returns
    ///
    /// # Errors
    ///
    fn add_assign(&mut self, rhs: VectorField<T>) {
        // x component
        for (elem, num) in self.x.iter_mut().zip(rhs.x.iter()) {
            *elem += *num;
        }
        
        // y component
        for (elem, num) in self.y.iter_mut().zip(rhs.y.iter()) {
            *elem += *num;
        }
        
        // z component
        for (elem, num) in self.z.iter_mut().zip(rhs.z.iter()) {
            *elem += *num;
        }
    }
}

/// implements VectorField<T> -= VectorField<T>
impl<T: Copy + SubAssign + Num> SubAssign<VectorField<T>> for VectorField<T> {
    /// implements VectorField<T> -= VectorField<T>
    ///
    /// # Arguments
    /// - `&mut self` mutable reference to self
    /// - `rhs: VectorField<T>` rhs of operation
    ///
    /// # Returns
    ///
    /// # Errors
    ///
    fn sub_assign(&mut self, rhs: VectorField<T>) { 
        // x component
        for (elem, num) in self.x.iter_mut().zip(rhs.x.iter()) {
            *elem -= *num;
        }
        // y component
        for (elem, num) in self.y.iter_mut().zip(rhs.y.iter()) {
            *elem -= *num;
        }
        
        // z component
        for (elem, num) in self.z.iter_mut().zip(rhs.z.iter()) {
            *elem -= *num;
        }
    }
}

/// implements VectorField<T> *= VectorField<T>
impl<T: Copy + MulAssign + Num> MulAssign<VectorField<T>> for VectorField<T> {
    /// implements VectorField<T> *= VectorField<T>
    ///
    /// # Arguments
    /// - `&mut self` mutable reference to self
    /// - `rhs: VectorField<T>` rhs of operation
    ///
    /// # Returns
    ///
    /// # Errors
    ///
    fn mul_assign(&mut self, rhs: VectorField<T>) {
        // x component
        for (elem, num) in self.x.iter_mut().zip(rhs.x.iter()) {
            *elem *= *num;
        }
        
        // y component
        for (elem, num) in self.y.iter_mut().zip(rhs.y.iter()) {
            *elem *= *num;
        }
        
        // z component
        for (elem, num) in self.z.iter_mut().zip(rhs.z.iter()) {
            *elem *= *num;
        }
    }
}

/// implements VectorField<T> /= VectorField<T>
impl<T: Copy + DivAssign + Num> DivAssign<VectorField<T>> for VectorField<T> {
    /// implements VectorField<T> /= VectorField<T>
    ///
    /// # Arguments
    /// - `&mut self` mutable reference to self
    /// - `rhs: VectorField<T>` rhs of operation
    ///
    /// # Returns
    ///
    /// # Errors
    ///
    fn div_assign(&mut self, rhs: VectorField<T>) {
        // x component
        for (elem, num) in self.x.iter_mut().zip(rhs.x.iter()) {
            *elem /= *num;
        }
        
        // y component
        for (elem, num) in self.y.iter_mut().zip(rhs.y.iter()) {
            *elem /= *num;
        }
        
        // z component
        for (elem, num) in self.z.iter_mut().zip(rhs.z.iter()) {
            *elem /= *num;
        }
    }
}

impl<T: Copy + AddAssign + Num> AddAssign<T> for VectorField<T> {
    /// implements VectorField<T> += T
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
        // x component
        for elem in self.x.iter_mut() {
            *elem += rhs;
        }
        
        // y component
        for elem in self.y.iter_mut() {
            *elem += rhs;
        }
        
        // z component
        for elem in self.z.iter_mut() {
            *elem += rhs;
        }
    }
}

impl<T: Copy + SubAssign + Num> SubAssign<T> for VectorField<T> {
    /// implements VectorField<T> -= T
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
        // x component
        for elem in self.x.iter_mut() {
            *elem -= rhs;
        }
        
        // y component
        for elem in self.y.iter_mut() {
            *elem -= rhs;
        }
        
        // z component
        for elem in self.z.iter_mut() {
            *elem -= rhs;
        }
    }
}

impl<T: Copy + MulAssign + Num> MulAssign<T> for VectorField<T> {
    /// implements VectorField<T> *= T
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
        // x component
        for elem in self.x.iter_mut() {
            *elem *= rhs;
        }
        
        // y component
        for elem in self.y.iter_mut() {
            *elem *= rhs;
        }
        
        // z component
        for elem in self.z.iter_mut() {
            *elem *= rhs;
        }
    }
}

impl<T: Copy + DivAssign + Num> DivAssign<T> for VectorField<T> {
    /// implements VectorField<T> /= T
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
        // x component
        for elem in self.x.iter_mut() {
            *elem /= rhs;
        }
        
        // y component
        for elem in self.y.iter_mut() {
            *elem /= rhs;
        }
        
        // z component
        for elem in self.z.iter_mut() {
            *elem /= rhs;
        }
    }
}

#[cfg(test)]
mod tests{
    
}