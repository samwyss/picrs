//! driver binary crate
//!
//! picrs build target

use anyhow::Result;
use picrs::model::Model;

/// main driver function
///
/// # Arguments
///
/// # Returns
/// `Result<(), anyhow::Error>`
///
/// # Errors
fn main() -> Result<(), anyhow::Error> {
    // todo create configuration from disk somehow

    // construct model
    let mut model = Model::new()?;

    // run model
    model.run();

    println!("Hello, world!");

    Ok(())
}
