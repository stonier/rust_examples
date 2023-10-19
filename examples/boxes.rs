// ***************************************************************************
// About
// ***************************************************************************

//! More fun in a box than you can deal with.
//
// ***************************************************************************
// Dependencies
// ***************************************************************************

use log::{error, info};
use thiserror::Error;

// ***************************************************************************
// Errors
// ***************************************************************************

#[derive(Error, Debug)]
pub enum ExampleError {
    #[error("Could not play music")]
    MusicalJane,
}

// ***************************************************************************
// Systems
// ***************************************************************************

fn dereference_a_boxed_trait_object<E>(result: Result<(), E>)
where
    E: std::error::Error + 'static
{
    if let Err(error) = result {
        let boxed_error = Box::new(error);

        // let reffed = &(*boxed_error);      // how to get a ref to the value
        let reffed = boxed_error.as_ref(); // same as previous ine
        info!("referenced: {}", reffed);

        // Move the value out of the box
        let moved = *boxed_error;
        info!("moved: {}", moved);
    };

}
// ***************************************************************************
// Main
// ***************************************************************************

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let result = Err(ExampleError::MusicalJane);
    dereference_a_boxed_trait_object(result);
}

