use crate::helpers::coordinate_triplet::CoordinateTriplet;
use num::Num;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, DivAssign, Index, IndexMut, MulAssign, SubAssign};

/// `ScalarField<T>` struct
///
/// describes a scalar field
#[derive(Debug, PartialEq)]
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
    /// `ScalarField<T>` constructor
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

    /// returns an iterator over `ScalarField<T>`
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

    /// returns a mutable iterator over `ScalarField<T>`
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

/// implements [] operator on `ScalarField<T>`
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

        // linearly index into `ScalarField<T>` using column major ordering
        &self.data[i + self.r_offset * j + self.p_offset * k]
    }
}

/// implements mutable [] operator on `ScalarField<T>`
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

        // linearly index into `ScalarField<T>` using column major ordering
        &mut self.data[i + self.r_offset * j + self.p_offset * k]
    }
}

/// allows `ScalarField<T>` to be written in a text format
impl<T: Display> Display for ScalarField<T> {
    /// writes `ScalarField<T>` in a text format
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

/// implements `ScalarField<T> += ScalarField<T>`
impl<T: Copy + AddAssign> AddAssign<ScalarField<T>> for ScalarField<T> {
    /// implements `ScalarField<T> += ScalarField<T>`
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

/// implements `ScalarField<T> -= ScalarField<T>`
impl<T: Copy + SubAssign> SubAssign<ScalarField<T>> for ScalarField<T> {
    /// implements `ScalarField<T> -= ScalarField<T>`
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

/// implements `ScalarField<T> *= ScalarField<T>`
impl<T: Copy + MulAssign> MulAssign<ScalarField<T>> for ScalarField<T> {
    /// implements `ScalarField<T> *= ScalarField<T>`
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

/// implements `ScalarField<T> /= ScalarField<T>`
impl<T: Copy + DivAssign> DivAssign<ScalarField<T>> for ScalarField<T> {
    /// implements `ScalarField<T> /= ScalarField<T>`
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

/// implements `ScalarField<T> += T`
impl<T: Copy + AddAssign> AddAssign<T> for ScalarField<T> {
    /// implements `ScalarField<T> += T`
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

/// implements `ScalarField<T> -= T`
impl<T: Copy + SubAssign> SubAssign<T> for ScalarField<T> {
    /// implements `ScalarField<T> -= T`
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

/// implements `ScalarField<T> *= T`
impl<T: Copy + MulAssign> MulAssign<T> for ScalarField<T> {
    /// implements `ScalarField<T> *= T`
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

/// implements `ScalarField<T> /= T`
impl<T: Copy + DivAssign> DivAssign<T> for ScalarField<T> {
    /// implements `ScalarField<T> /= T`
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
    use crate::field::scalar::ScalarField;
    use crate::helpers::coordinate_triplet::CoordinateTriplet;

    /// helper function that sets up a `ScalarField<f64>` for testing
    ///
    /// # Arguments
    ///
    /// # Returns
    /// `Result<ScalarField<f64>, anyhow::Error>`
    ///
    /// # Errors
    /// - `CoordinateTriplet::new()` fails
    fn setup() -> Result<ScalarField<f64>, anyhow::Error> {
        // size of data for testing
        let cells = CoordinateTriplet::new(2, 4, 6)?;

        // scalar field for testing
        let scalar_field: Result<ScalarField<f64>, anyhow::Error> = ScalarField::new(&cells);

        scalar_field
    }

    /// tests `ScalarField::new()` for success
    ///
    /// # Errors
    /// - `ScalarField::new()` fails for f64
    /// - `ScalarField::new()` fails for u64
    ///
    #[test]
    fn new_success() {
        // setup
        let cells = CoordinateTriplet::new(2, 4, 6).unwrap();
        let scalar_field_f64: Result<ScalarField<f64>, anyhow::Error> = ScalarField::new(&cells);
        let scalar_field_u64: Result<ScalarField<u64>, anyhow::Error> = ScalarField::new(&cells);

        // assertions
        assert!(scalar_field_f64.is_ok());
        assert!(scalar_field_u64.is_ok())
    }

    /// tests `ScalarField::new()` for correct setting of `cells` member
    ///
    /// # Errors
    /// - `ScalarField::new()` sets incorrect `ScalarField.cells`
    ///
    #[test]
    fn new_correct_cells() {
        // setup
        let scalar_field: ScalarField<f64> = setup().unwrap();

        // assertions
        assert_eq!(
            scalar_field.cells,
            CoordinateTriplet::new(2, 4, 6,).unwrap()
        );
    }

    /// tests `ScalarField::new()` for correct setting of `r_offset` member
    ///
    /// # Errors
    /// - `ScalarField::new()` sets incorrect `ScalarField.r_offset`
    ///
    #[test]
    fn new_correct_r_offset() {
        // setup
        let scalar_field: ScalarField<f64> = setup().unwrap();

        // assertions
        assert_eq!(scalar_field.r_offset, 2);
    }

    /// tests `ScalarField::new()` for correct setting of `p_offset` member
    ///
    /// # Errors
    /// - `ScalarField::new()` sets incorrect `ScalarField.p_offset`
    ///
    #[test]
    fn new_correct_p_offset() {
        // setup
        let scalar_field: ScalarField<f64> = setup().unwrap();

        // assertions
        assert_eq!(scalar_field.p_offset, 8);
    }

    /// tests `ScalarField::new()` for correct setting of `data` member
    ///
    /// # Errors
    /// - `ScalarField::new()` sets incorrect `ScalarField.data`
    ///
    #[test]
    fn new_correct_data() {
        // setup
        let scalar_field: ScalarField<f64> = setup().unwrap();

        // assertions
        assert_eq!(scalar_field.data.len(), 48);
        for elem in scalar_field.data {
            assert_eq!(elem, 0.0);
        }
    }

    /// tests `ScalarField::iter()` for correctness
    ///
    /// # Errors
    /// - `ScalarField::iter_mut()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_iter() {
        // setup
        let scalar_field: ScalarField<f64> = setup().unwrap();

        // assertions
        scalar_field.iter().for_each(|num| {
            assert_eq!(*num, 0.0);
        });
    }

    /// tests `ScalarField::iter_mut()` for correctness
    ///
    /// # Errors
    /// - `ScalarField::iter_mut()` does not implement iterator correctly
    ///
    #[test]
    fn impl_iter_mut() {
        // setup
        let mut scalar_field: ScalarField<f64> = setup().unwrap();

        // assertions
        scalar_field.iter_mut().for_each(|num| *num += 1.0);
        scalar_field
            .iter_mut()
            .for_each(|num| assert_eq!(*num, 1.0));
    }

    /// tests `ScalarField` for correct implementation of `Index`
    ///
    /// # Errors
    /// - `ScalarField<T>` does not implement `Index` correctly
    ///
    #[test]
    fn impl_index() {
        // setup
        let scalar_field: ScalarField<f64> = setup().unwrap();

        // assertions
        assert_eq!(scalar_field[(0, 0, 0)], 0.0);
        assert_eq!(scalar_field[(1, 2, 0)], 0.0);
        assert_eq!(scalar_field[(2, 2, 2)], 0.0);
    }

    /// tests `ScalarField<T>` for correct implementation of `IndexMut`
    ///
    /// # Errors
    /// - `ScalarField<T>` does not implement `IndexMut` correctly
    ///
    #[test]
    fn impl_index_mut() {
        // setup
        let mut scalar_field: ScalarField<f64> = setup().unwrap();
        scalar_field[(0, 0, 0)] = 10.0;
        scalar_field[(1, 2, 0)] = 20.0;
        scalar_field[(2, 2, 2)] = 30.0;

        // assertions
        assert_eq!(scalar_field[(0, 0, 0)], 10.0);
        assert_eq!(scalar_field[(1, 2, 0)], 20.0);
        assert_eq!(scalar_field[(2, 2, 2)], 30.0);
    }

    /// tests `ScalarField<T>` for implementation of `Display`
    ///
    /// # Errors
    /// - `ScalarField<T>` does not implement `Display`
    ///
    #[test]
    fn impl_display() {
        // setup
        let scalar_field: ScalarField<f64> = setup().unwrap();

        // assertions
        // this will fail if Display is not implemented, it is not currently worth checking the output
        println!("{}", scalar_field);
    }

    /// tests `ScalarField<T>` for correct implementation of `AddAssign<ScalarField<T>>`
    ///
    /// # Errors
    /// - `ScalarField<T>` does not implement `AddAssign<ScalarField<T>>` correctly
    /// - `ScalarField<T>` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_add_assign_scalar_field() {
        // setup
        let mut scalar_field1: ScalarField<f64> = setup().unwrap();
        scalar_field1 += 1.0;

        let mut scalar_field2: ScalarField<f64> = setup().unwrap();
        scalar_field2 += 2.0;

        scalar_field1 += scalar_field2;

        // assertions
        scalar_field1.iter().for_each(|num| assert_eq!(*num, 3.0));
    }

    /// tests `ScalarField<T>` for correct implementation of `SubAssign<ScalarField<T>>`
    ///
    /// # Errors
    /// - `ScalarField<T>` does not implement `SubAssign<ScalarField<T>>` correctly
    /// - `ScalarField<T>` does not implement `SubAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_sub_assign_scalar_field() {
        // setup
        let mut scalar_field1: ScalarField<f64> = setup().unwrap();
        scalar_field1 -= 1.0;

        let mut scalar_field2: ScalarField<f64> = setup().unwrap();
        scalar_field2 -= 2.0;

        scalar_field1 -= scalar_field2;

        // assertions
        scalar_field1.iter().for_each(|num| assert_eq!(*num, 1.0));
    }

    /// tests `ScalarField<T>` for correct implementation of `MulAssign<ScalarField<T>>`
    ///
    /// # Errors
    /// - `ScalarField<T>` does not implement `MulAssign<ScalarField<T>>` correctly
    /// - `ScalarField<T>` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_mul_assign_scalar_field() {
        // setup
        let mut scalar_field1: ScalarField<f64> = setup().unwrap();
        scalar_field1 += 1.0;

        let mut scalar_field2: ScalarField<f64> = setup().unwrap();
        scalar_field2 += 2.0;

        scalar_field1 *= scalar_field2;

        // assertions
        scalar_field1.iter().for_each(|num| assert_eq!(*num, 2.0));
    }

    /// tests `ScalarField<T>` for correct implementation of `DivAssign<ScalarField<T>>`
    ///
    /// # Errors
    /// - `ScalarField<T>` does not implement `DivAssign<ScalarField<T>>` correctly
    /// - `ScalarField<T>` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_div_assign_scalar_field() {
        // setup
        let mut scalar_field1: ScalarField<f64> = setup().unwrap();
        scalar_field1 += 1.0;

        let mut scalar_field2: ScalarField<f64> = setup().unwrap();
        scalar_field2 += 2.0;

        scalar_field1 /= scalar_field2;

        // assertions
        scalar_field1.iter().for_each(|num| assert_eq!(*num, 0.5));
    }

    /// tests `ScalarField<T>` for correct implementation of `AddAssign<T>`
    ///
    /// # Errors
    /// - `ScalarField<T>` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_add_assign_t() {
        // setup
        let mut scalar_field: ScalarField<f64> = setup().unwrap();
        scalar_field += 1.0;

        // assertions
        scalar_field.iter().for_each(|num| assert_eq!(*num, 1.0));
    }

    /// tests `ScalarField<T>` for correct implementation of `SubAssign<T>`
    ///
    /// # Errors
    /// - `ScalarField<T>` does not implement `SubAssign<T>` correctly
    /// - `ScalarField<T>` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_sub_assign_t() {
        // setup
        let mut scalar_field: ScalarField<f64> = setup().unwrap();
        scalar_field += 10.0;
        scalar_field -= 5.0;

        // assertions
        scalar_field.iter().for_each(|num| assert_eq!(*num, 5.0));
    }

    /// tests `ScalarField<T>` for correct implementation of `MulAssign<T>`
    ///
    /// # Errors
    /// - `ScalarField<T>` does not implement `MulAssign<T>` correctly
    /// - `ScalarField<T>` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_mul_assign_t() {
        // setup
        let mut scalar_field: ScalarField<f64> = setup().unwrap();
        scalar_field += 10.0;
        scalar_field *= 5.0;

        // assertions
        scalar_field.iter().for_each(|num| assert_eq!(*num, 50.0));
    }

    /// tests `ScalarField<T>` for correct implementation of `DivAssign<T>`
    ///
    /// # Errors
    /// - `ScalarField<T>` does not implement `DivAssign<T>` correctly
    /// - `ScalarField<T>` does not implement `AddAssign<T>` correctly
    /// - `ScalarField::iter()` does not implement `Iterator` correctly
    ///
    #[test]
    fn impl_div_assign_t() {
        // setup
        let mut scalar_field: ScalarField<f64> = setup().unwrap();
        scalar_field += 10.0;
        scalar_field /= 5.0;

        // assertions
        scalar_field.iter().for_each(|num| assert_eq!(*num, 2.0));
    }
}
