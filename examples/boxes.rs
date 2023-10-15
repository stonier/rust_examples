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
        let e = Box::new(error);
        let f = &(*e);  // <-- this is how to get the error out of the box
        println!("f: {}", f);
    };

}
// ***************************************************************************
// Main
// ***************************************************************************

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    dereference_a_boxed_trait_object(Err(ExampleError::MusicalJane));
}

