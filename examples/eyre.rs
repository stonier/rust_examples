// ***************************************************************************
// About
// ***************************************************************************

//! Error reports with Eyre
//
// ***************************************************************************
// Dependencies
// ***************************************************************************

use std::os::unix::prelude::OsStringExt;

// use std::marker::{Sync, Send};
use color_eyre::{
    Report, Section,
    eyre::{Result, WrapErr}
};
use log::{error, info};
use thiserror::Error;

// ***************************************************************************
// Errors
// ***************************************************************************

#[derive(Error, Debug)]
pub enum ExampleError {
    #[error("Jane eyre is not plain!")]
    PlainJane,

    #[error("Could not play music")]
    MusicalJane,
}

// ***************************************************************************
// Systems
// ***************************************************************************

fn error_plain_jane() -> Result<()> {
    info!("error_plain_jane()");
    Err(ExampleError::PlainJane)
        .wrap_err("Found Plain Jane")
        .suggestion("You can't think for yourself?")?
}

fn error_musical_jane() -> Result<()> {
    let error = ExampleError::MusicalJane;
    info!("error_musical_jane()");
    Err(error)
        .wrap_err("She hasn't got an instrument")
        .suggestion("Give her an instrument?")?
}

// type BoxedError = Box::<dyn std::error::Error + 'static>;

// fn boxed_error() -> BoxedError {
//     Box::new(ExampleError::MusicalJane)
// }

// fn generate_error_result() -> std::result::Result<(), ExampleError> {
//     Err(ExampleError::MusicalJane)
// }

// fn result_into_a_boxed_error<E>(result: std::result::Result<(), E>) -> BoxedError
// where
//     E: std::error::Error + 'static
// {
//     if let Err(err) = result {
//         let boxed_error: BoxedError = Box::new(err);
//         // let report = eyre!(boxed_error);
//         // info!("Report: {}", report);
//         // let moved_error = *boxed_error;
//         // info!("Report: {:?}", Report::new(moved_error));
//         boxed_error
//     } else {
//         let error = ExampleError::MusicalJane;
//         let boxed_error : BoxedError = Box::new(error);
//         boxed_error
//     }
// }

// ***************************************************************************
// Main
// ***************************************************************************

#[derive(Clone, Debug, Error)]
pub enum Foo {
    #[error("A Error")]
    A(A)
}

#[derive(Clone, Debug)]
pub struct A {
    name: String,
    nick: String
}

impl Default for A {
    fn default() -> Self {
        Self {
            name: "default_name".to_string(),
            nick: "default_nick".to_string()
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> { // miette::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    color_eyre::install()?;

    if let Err(report) = error_plain_jane() {
        error!("\n\n{:?}", report);
    };

    if let Err(report) = error_musical_jane() {
        error!("\n\n{:?}", report);
    };

    // let result = generate_error_result();
    // let boxed_error = result_into_a_boxed_error(result);
    // // info!("\nBoxed Error: {}", boxed_error);
    // let dereferenced_boxed_error = boxed_error.as_ref();
    // println!("{:?}", dereferenced_boxed_error);  // <-- OK
    // // info!("Report: {}", Report::new(dereferenced_boxed_error)); // <-- fails, lifetime not static
    // let moved_error = *boxed_error;

    // result_into_a_boxed_error(result);

    println!("\nMay you be blessed by a tickle from his noodly appendages...\n");
    Ok(())
}

