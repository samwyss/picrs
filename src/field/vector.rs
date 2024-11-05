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

/// implements `VectorField<T> += VectorField<T>`
impl<T: Copy + AddAssign + Num> AddAssign<VectorField<T>> for VectorField<T> {
    /// implements `VectorField<T> += VectorField<T>`
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

/// implements `VectorField<T> -= VectorField<T>`
impl<T: Copy + SubAssign + Num> SubAssign<VectorField<T>> for VectorField<T> {
    /// implements `VectorField<T> -= VectorField<T>`
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

/// implements `VectorField<T> *= VectorField<T>`
impl<T: Copy + MulAssign + Num> MulAssign<VectorField<T>> for VectorField<T> {
    /// implements `VectorField<T> *= VectorField<T>`
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

/// implements `VectorField<T> /= VectorField<T>`
impl<T: Copy + DivAssign + Num> DivAssign<VectorField<T>> for VectorField<T> {
    /// implements `VectorField<T> /= VectorField<T>`
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

/// implements `VectorField<T> += T`
impl<T: Copy + AddAssign + Num> AddAssign<T> for VectorField<T> {
    /// implements `VectorField<T> += T`
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

/// implements `VectorField<T> -= T`
impl<T: Copy + SubAssign + Num> SubAssign<T> for VectorField<T> {
    /// implements `VectorField<T> -= T`
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

/// implements `VectorField<T> *= T`
impl<T: Copy + MulAssign + Num> MulAssign<T> for VectorField<T> {
    /// implements `VectorField<T> *= T`
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

/// implements `VectorField<T> /= T`
impl<T: Copy + DivAssign + Num> DivAssign<T> for VectorField<T> {
    /// implements `VectorField<T> /= T`
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

/// implements `VectorField<T> += ScalarField<T>`
impl<T: Copy + AddAssign + Num> AddAssign<ScalarField<T>> for VectorField<T> {
    /// implements `VectorField<T> += ScalarField<T>`
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
        // x component
        for (elem, num) in self.x.iter_mut().zip(rhs.iter()) {
            *elem += *num;
        }

        // y component
        for (elem, num) in self.y.iter_mut().zip(rhs.iter()) {
            *elem += *num;
        }

        // z component
        for (elem, num) in self.z.iter_mut().zip(rhs.iter()) {
            *elem += *num;
        }
    }
}

/// implements `VectorField<T> -= ScalarField<T>`
impl<T: Copy + SubAssign + Num> SubAssign<ScalarField<T>> for VectorField<T> {
    /// implements `VectorField<T> -= ScalarField<T>`
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
        // x component
        for (elem, num) in self.x.iter_mut().zip(rhs.iter()) {
            *elem -= *num;
        }

        // y component
        for (elem, num) in self.y.iter_mut().zip(rhs.iter()) {
            *elem -= *num;
        }

        // z component
        for (elem, num) in self.z.iter_mut().zip(rhs.iter()) {
            *elem -= *num;
        }
    }
}

/// implements `VectorField<T> *= ScalarField<T>`
impl<T: Copy + MulAssign + Num> MulAssign<ScalarField<T>> for VectorField<T> {
    /// implements `VectorField<T> *= ScalarField<T>`
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
        // x component
        for (elem, num) in self.x.iter_mut().zip(rhs.iter()) {
            *elem *= *num;
        }

        // y component
        for (elem, num) in self.y.iter_mut().zip(rhs.iter()) {
            *elem *= *num;
        }

        // z component
        for (elem, num) in self.z.iter_mut().zip(rhs.iter()) {
            *elem *= *num;
        }
    }
}

/// implements `VectorField<T> /= ScalarField<T>`
impl<T: Copy + DivAssign + Num> DivAssign<ScalarField<T>> for VectorField<T> {
    /// implements `VectorField<T> /= ScalarField<T>`
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
        // x component
        for (elem, num) in self.x.iter_mut().zip(rhs.iter()) {
            *elem /= *num;
        }

        // y component
        for (elem, num) in self.y.iter_mut().zip(rhs.iter()) {
            *elem /= *num;
        }

        // z component
        for (elem, num) in self.z.iter_mut().zip(rhs.iter()) {
            *elem /= *num;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::field::scalar::ScalarField;
    use crate::field::vector::VectorField;
    use crate::helpers::coordinate_triplet::CoordinateTriplet;

    /// helper function that sets up a `VectorField<f64>` for testing
    ///
    /// # Arguments
    ///
    /// # Returns
    /// `Result<VectorField<f64>, anyhow::Error>`
    ///
    /// # Errors
    /// - `CoordinateTriplet::new()` fails
    fn setup() -> Result<VectorField<f64>, anyhow::Error> {
        // size of data for testing
        let cells = CoordinateTriplet::new(2, 4, 6)?;

        // scalar field for testing
        let vector_field: Result<VectorField<f64>, anyhow::Error> = VectorField::new(&cells);

        vector_field
    }

    /// tests `VectorField::new()` for success
    ///
    /// # Errors
    /// - `VectorField::new()` fails for f64
    /// - `VectorField::new()` fails for u64
    ///
    #[test]
    fn new_success() {
        // setup
        let cells = CoordinateTriplet::new(2, 4, 6).unwrap();
        let vector_field_f64: Result<VectorField<f64>, anyhow::Error> = VectorField::new(&cells);
        let vector_field_u64: Result<VectorField<u64>, anyhow::Error> = VectorField::new(&cells);

        // assertions
        assert!(vector_field_f64.is_ok());
        assert!(vector_field_u64.is_ok())
    }

    /// tests `VectorField::new()` for correct setting of `cells` member
    ///
    /// # Errors
    /// - `VectorField::new()` sets incorrect `VectorField.cells`
    ///
    #[test]
    fn new_correct_cells() {
        // setup
        let vector_field: VectorField<f64> = setup().unwrap();

        // assertions
        assert_eq!(
            vector_field.cells,
            CoordinateTriplet::new(2, 4, 6,).unwrap()
        );
    }

    /// tests `VectorField::new()` for correct setting of `x` member
    ///
    /// # Errors
    /// - `VectorField::new()` sets incorrect `VectorField.x`
    /// - `ScalarField::new()` fails
    #[test]
    fn new_correct_x() {
        // setup
        let vector_field: VectorField<f64> = setup().unwrap();
        let cells = CoordinateTriplet::new(2, 4, 6).unwrap();

        // assertions
        assert_eq!(vector_field.x, ScalarField::new(&cells).unwrap());
    }

    /// tests `VectorField::new()` for correct setting of `y` member
    ///
    /// # Errors
    /// - `VectorField::new()` sets incorrect `VectorField.y`
    /// - `ScalarField::new()` fails
    ///
    #[test]
    fn new_correct_y() {
        // setup
        let vector_field: VectorField<f64> = setup().unwrap();
        let cells = CoordinateTriplet::new(2, 4, 6).unwrap();

        // assertions
        assert_eq!(vector_field.y, ScalarField::new(&cells).unwrap());
    }

    /// tests `VectorField::new()` for correct setting of `z` member
    ///
    /// # Errors
    /// - `VectorField::new()` sets incorrect `VectorField.z`
    /// - `ScalarField::new()` fails
    ///
    #[test]
    fn new_correct_z() {
        // setup
        let vector_field: VectorField<f64> = setup().unwrap();
        let cells = CoordinateTriplet::new(2, 4, 6).unwrap();

        // assertions
        assert_eq!(vector_field.z, ScalarField::new(&cells).unwrap());
    }

    /// tests `VectorField` for implementation of `Display`
    ///
    /// # Errors
    /// - `VectorField` does not implement `Display`
    ///
    #[test]
    fn impl_display() {
        // setup
        let vector_field: VectorField<f64> = setup().unwrap();

        // assertions
        // this will fail if Display is not implemented, it is not currently worth checking the output
        println!("{}", vector_field);
    }

    /// tests `VectorField` for correct implementation of `AddAssign<VectorField<T>>`
    ///
    /// # Errors
    /// - `VectorField` does not implement `AddAssign<VectorField<T>>` correctly
    /// - `VectorField` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_add_assign_vector_field() {
        // setup
        let mut vector_field1: VectorField<f64> = setup().unwrap();
        vector_field1 += 1.0;

        let mut vector_field2: VectorField<f64> = setup().unwrap();
        vector_field2 += 2.0;

        vector_field1 += vector_field2;

        // assertions
        vector_field1.x.iter().for_each(|num| assert_eq!(*num, 3.0));
        vector_field1.y.iter().for_each(|num| assert_eq!(*num, 3.0));
        vector_field1.z.iter().for_each(|num| assert_eq!(*num, 3.0));
    }

    /// tests `VectorField` for correct implementation of `SubAssign<VectorField<T>>`
    ///
    /// # Errors
    /// - `VectorField` does not implement `SubAssign<ScalarField<T>>` correctly
    /// - `VectorField` does not implement `SubAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_sub_assign_vector_field() {
        // setup
        let mut vector_field1: VectorField<f64> = setup().unwrap();
        vector_field1 -= 1.0;

        let mut vector_field2: VectorField<f64> = setup().unwrap();
        vector_field2 -= 2.0;

        vector_field1 -= vector_field2;

        // assertions
        vector_field1.x.iter().for_each(|num| assert_eq!(*num, 1.0));
        vector_field1.y.iter().for_each(|num| assert_eq!(*num, 1.0));
        vector_field1.z.iter().for_each(|num| assert_eq!(*num, 1.0));
    }

    /// tests `VectorField` for correct implementation of `MulAssign<VectorField<T>>`
    ///
    /// # Errors
    /// - `VectorField` does not implement `MulAssign<VectorField<T>>` correctly
    /// - `VectorField` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_mul_assign_vector_field() {
        // setup
        let mut vector_field1: VectorField<f64> = setup().unwrap();
        vector_field1 += 1.0;

        let mut vector_field2: VectorField<f64> = setup().unwrap();
        vector_field2 += 2.0;

        vector_field1 *= vector_field2;

        // assertions
        vector_field1.x.iter().for_each(|num| assert_eq!(*num, 2.0));
        vector_field1.y.iter().for_each(|num| assert_eq!(*num, 2.0));
        vector_field1.z.iter().for_each(|num| assert_eq!(*num, 2.0));
    }

    /// tests `VectorField` for correct implementation of `DivAssign<VectorField<T>>`
    ///
    /// # Errors
    /// - `VectorField` does not implement `DivAssign<VectorField<T>>` correctly
    /// - `VectorField` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_div_assign_vector_field() {
        // setup
        let mut vector_field1: VectorField<f64> = setup().unwrap();
        vector_field1 += 1.0;

        let mut vector_field2: VectorField<f64> = setup().unwrap();
        vector_field2 += 2.0;

        vector_field1 /= vector_field2;

        // assertions
        vector_field1.x.iter().for_each(|num| assert_eq!(*num, 0.5));
        vector_field1.y.iter().for_each(|num| assert_eq!(*num, 0.5));
        vector_field1.z.iter().for_each(|num| assert_eq!(*num, 0.5));
    }

    /// tests `VectorField` for correct implementation of `AddAssign<T>`
    ///
    /// # Errors
    /// - `VectorField` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_add_assign_t() {
        // setup
        let mut vector_field: VectorField<f64> = setup().unwrap();
        vector_field += 1.0;

        // assertions
        vector_field.x.iter().for_each(|num| assert_eq!(*num, 1.0));
        vector_field.y.iter().for_each(|num| assert_eq!(*num, 1.0));
        vector_field.z.iter().for_each(|num| assert_eq!(*num, 1.0));
    }

    /// tests `VectorField` for correct implementation of `SubAssign<T>`
    ///
    /// # Errors
    /// - `VectorField` does not implement `SubAssign<T>` correctly
    /// - `VectorField` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_sub_assign_t() {
        // setup
        let mut vector_field: VectorField<f64> = setup().unwrap();
        vector_field += 10.0;
        vector_field -= 5.0;

        // assertions
        vector_field.x.iter().for_each(|num| assert_eq!(*num, 5.0));
        vector_field.y.iter().for_each(|num| assert_eq!(*num, 5.0));
        vector_field.z.iter().for_each(|num| assert_eq!(*num, 5.0));
    }

    /// tests `VectorField` for correct implementation of `MulAssign<T>`
    ///
    /// # Errors
    /// - `VectorField` does not implement `MulAssign<T>` correctly
    /// - `VectorField` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_mul_assign_t() {
        // setup
        let mut vector_field: VectorField<f64> = setup().unwrap();
        vector_field += 10.0;
        vector_field *= 5.0;

        // assertions
        vector_field.x.iter().for_each(|num| assert_eq!(*num, 50.0));
        vector_field.y.iter().for_each(|num| assert_eq!(*num, 50.0));
        vector_field.z.iter().for_each(|num| assert_eq!(*num, 50.0));
    }

    /// tests `VectorField` for correct implementation of `DivAssign<T>`
    ///
    /// # Errors
    /// - `VectorField` does not implement `DivAssign<T>` correctly
    /// - `VectorField` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_div_assign_t() {
        // setup
        let mut vector_field: VectorField<f64> = setup().unwrap();
        vector_field += 10.0;
        vector_field /= 5.0;

        // assertions
        vector_field.x.iter().for_each(|num| assert_eq!(*num, 2.0));
        vector_field.y.iter().for_each(|num| assert_eq!(*num, 2.0));
        vector_field.z.iter().for_each(|num| assert_eq!(*num, 2.0));
    }
    
    /// tests `VectorField<T>` for correct implementation of `AddAssign<ScalarField<T>>`
    ///
    /// # Errors
    /// - `VectorField<T>` does not implement `AddAssign<ScalarField<T>>` correctly
    /// - `VectorField<T>` does not implement `AddAssign<T>` correctly
    /// - `ScalarField<T>` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_add_assign_scalar_field() {
        // setup
        let mut vector_field: VectorField<f64> = setup().unwrap();
        let cells: CoordinateTriplet<usize> = CoordinateTriplet::new(2, 4, 6).unwrap();
        let mut scalar_field: ScalarField<f64> = ScalarField::new(&cells).unwrap();
        vector_field += 1.0;
        scalar_field += 2.0;
        vector_field += scalar_field;
        
        // assertions
        vector_field.x.iter().for_each(|num| assert_eq!(*num, 3.0));
        vector_field.y.iter().for_each(|num| assert_eq!(*num, 3.0));
        vector_field.z.iter().for_each(|num| assert_eq!(*num, 3.0));
    }
    
    /// tests `VectorField<T>` for correct implementation of `SubAssign<ScalarField<T>>`
    ///
    /// # Errors
    /// - `VectorField<T>` does not implement `SubAssign<ScalarField<T>>` correctly
    /// - `VectorField<T>` does not implement `SubAssign<T>` correctly
    /// - `ScalarField<T>` does not implement `SubAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_sub_assign_scalar_field() {
        // setup
        let mut vector_field: VectorField<f64> = setup().unwrap();
        let cells: CoordinateTriplet<usize> = CoordinateTriplet::new(2, 4, 6).unwrap();
        let mut scalar_field: ScalarField<f64> = ScalarField::new(&cells).unwrap();
        vector_field -= 10.0;
        scalar_field -= 2.0;
        vector_field -= scalar_field;
        
        // assertions
        vector_field.x.iter().for_each(|num| assert_eq!(*num, -8.0));
        vector_field.y.iter().for_each(|num| assert_eq!(*num, -8.0));
        vector_field.z.iter().for_each(|num| assert_eq!(*num, -8.0));
    }
    
    /// tests `VectorField<T>` for correct implementation of `MulAssign<ScalarField<T>>`
    ///
    /// # Errors
    /// - `VectorField<T>` does not implement `MulAssign<ScalarField<T>>` correctly
    /// - `VectorField<T>` does not implement `AddAssign<T>` correctly
    /// - `ScalarField<T>` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_mul_assign_scalar_field() {
        // setup
        let mut vector_field: VectorField<f64> = setup().unwrap();
        let cells: CoordinateTriplet<usize> = CoordinateTriplet::new(2, 4, 6).unwrap();
        let mut scalar_field: ScalarField<f64> = ScalarField::new(&cells).unwrap();
        vector_field += 2.0;
        scalar_field += 10.0;
        vector_field *= scalar_field;
        
        // assertions
        vector_field.x.iter().for_each(|num| assert_eq!(*num, 20.0));
        vector_field.y.iter().for_each(|num| assert_eq!(*num, 20.0));
        vector_field.z.iter().for_each(|num| assert_eq!(*num, 20.0));
    }
    
    /// tests `VectorField<T>` for correct implementation of `DivAssign<ScalarField<T>>`
    ///
    /// # Errors
    /// - `VectorField<T>` does not implement `DivAssign<ScalarField<T>>` correctly
    /// - `VectorField<T>` does not implement `AddAssign<T>` correctly
    /// - `ScalarField<T>` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_div_assign_scalar_field() {
        // setup
        let mut vector_field: VectorField<f64> = setup().unwrap();
        let cells: CoordinateTriplet<usize> = CoordinateTriplet::new(2, 4, 6).unwrap();
        let mut scalar_field: ScalarField<f64> = ScalarField::new(&cells).unwrap();
        vector_field += 10.0;
        scalar_field += 2.0;
        vector_field /= scalar_field;
        
        // assertions
        vector_field.x.iter().for_each(|num| assert_eq!(*num, 5.0));
        vector_field.y.iter().for_each(|num| assert_eq!(*num, 5.0));
        vector_field.z.iter().for_each(|num| assert_eq!(*num, 5.0));
    }
}
