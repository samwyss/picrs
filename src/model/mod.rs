use crate::world::{CoordinateTriplet, ScalarField, VectorField};

pub struct Model {}

impl Model {
    pub fn new() -> Result<Model, anyhow::Error> {
        Ok(Model {})
    }

    pub fn run(&mut self) -> Result<(), anyhow::Error> {
        let coordinates: CoordinateTriplet<usize> = CoordinateTriplet::new(2, 2, 2)?;

        let mut field: ScalarField<f64> = ScalarField::new(coordinates)?;
        field[(1, 1, 1)] = 50.0;
        field[(0, 0, 0)] = 1000.0;

        println!("{}", field);

        let coordinates: CoordinateTriplet<usize> = CoordinateTriplet::new(2, 2, 2)?;
        let mut vec_field: VectorField<f64> = VectorField::new(coordinates)?;
        vec_field[(1, 1, 1)].x = 1000.0;
        vec_field[(0, 0, 0)].z = 50.0;

        println!("{}", vec_field);

        Ok(())
    }
}
