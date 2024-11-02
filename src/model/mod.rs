use crate::field::vector::VectorField;
use crate::world::CoordinateTriplet;

pub struct Model {}

impl Model {
    pub fn new() -> Result<Model, anyhow::Error> {
        Ok(Model {})
    }

    pub fn run(&mut self) -> Result<(), anyhow::Error> {
        let coordinates: CoordinateTriplet<usize> = CoordinateTriplet::new(100, 100, 100)?;

        let mut field: VectorField<f64> = VectorField::new(&coordinates)?;

        field += 1.0;
        field /= 5.0;

        println!("{}", field[(1,1,1)].x);

        Ok(())
    }
}
