use crate::field::{ScalarField, VectorField};

pub struct Model {}

impl Model {
    pub fn new() -> Result<Model, anyhow::Error> {
        Ok(Model {})
    }

    pub fn run(&mut self) -> Result<(), anyhow::Error> {
        let mut field: ScalarField<f64> = ScalarField::new(2, 2, 2)?;
        field[(1, 1, 1)] = 1000.0;
        field[(0, 0, 0)] = 1000.0;

        println!("{}", field);

        let mut vec_field: VectorField<f64> = VectorField::new(2, 2, 2)?;
        vec_field[(1, 1, 1, 0)] = 1000.0;
        vec_field[(0, 0, 0, 2)] = 1000.0;

        println!("{}", vec_field);

        Ok(())
    }
}
