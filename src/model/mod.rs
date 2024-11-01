use crate::world::{CoordinateTriplet, ScalarField};

pub struct Model {}

impl Model {
    pub fn new() -> Result<Model, anyhow::Error> {
        Ok(Model {})
    }

    pub fn run(&mut self) -> Result<(), anyhow::Error> {
        let coordinates: CoordinateTriplet<usize> = CoordinateTriplet::new(2, 2, 2)?;

        let mut field: ScalarField<f64> = ScalarField::new(coordinates)?;

        field += 1.0;
        field = field + 1.0;
        field = 1.0 + field;

        println!("{}", field);

        Ok(())
    }
}
