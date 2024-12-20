use std::fmt::{Display, Formatter};

/// `CoordinateTriplet` struct
///
/// represents generic data that by nature has (x, y, z) components
#[derive(Debug, Clone, PartialEq)]
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

impl<T: Display> Display for CoordinateTriplet<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::coordinate_triplet::CoordinateTriplet;

    /// tests `CoordinateTriplet::new()` for success
    ///
    /// # Errors
    /// - `CoordinateTriplet::new()` fails for f64
    /// - `CoordinateTriplet::new()` fails for usize
    ///
    #[test]
    fn new_success() {
        // test f64
        let a: CoordinateTriplet<f64> = CoordinateTriplet::new(1.0, 2.0, 3.0).unwrap();
        assert_eq!(CoordinateTriplet {x: 1.0, y: 2.0, z: 3.0}, a);
        
        // test usize
        let b: CoordinateTriplet<usize> = CoordinateTriplet::new(1, 2, 3).unwrap();
        assert_eq!(CoordinateTriplet {x: 1, y: 2, z: 3}, b);
    }
}