use crate::world::{CoordinateTriplet, ScalarField};

pub struct Model {}

impl Model {
    pub fn new() -> Result<Model, anyhow::Error> {
        Ok(Model {})
    }

    pub fn run(&mut self) -> Result<(), anyhow::Error> {
        let coordinates: CoordinateTriplet<usize> = CoordinateTriplet::new(1, 1, 1)?;

        let mut field: ScalarField<f64> = ScalarField::new(coordinates)?;

        field[(0, 0, 0)] = 1.0;

        field *= 2.0;

        println!("{}", field);

        Ok(())
    }
}
