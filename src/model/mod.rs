use crate::field::scalar::ScalarField;
use crate::field::vector::VectorField;
use crate::helpers::coordinate_triplet::CoordinateTriplet;

pub struct Model {}

impl Model {
    pub fn new() -> Result<Model, anyhow::Error> {
        Ok(Model {})
    }

    pub fn run(&mut self) -> Result<(), anyhow::Error> {
        let coordinates: CoordinateTriplet<usize> = CoordinateTriplet::new(3, 3, 3)?;

        let mut vfield: VectorField<f64> = VectorField::new(&coordinates)?;
        let mut sfield: ScalarField<f64> = ScalarField::new(&coordinates)?;
        
        sfield += 10.0;
        sfield *= 2.0;
        sfield -= 5.0;
        sfield /= 5.0;
        
        vfield += 10.0;
        vfield *= 2.0;
        vfield -= 5.0;
        vfield /= 5.0;

        println!("{}", sfield);
        println!("{}", vfield);

        Ok(())
    }
}
