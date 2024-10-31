//! Geometry module
//!
//! describes the geometry of the simulation domain
mod geometry {

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
            let cells: CoordinateTriplet<u64> =
                CoordinateTriplet::new(cells[0], cells[1], cells[2])?;

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
}
