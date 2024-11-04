use std::fmt::{Display, Formatter};

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

impl<T: Display> Display for CoordinateTriplet<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod test {
    
}