// ***************************************************************************
// About
// ***************************************************************************

//! Error reports with Eyre
//
// ***************************************************************************
// Dependencies
// ***************************************************************************

use color_eyre::{
    Section,
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
    info!("error_musical_jane()");
    Err(ExampleError::MusicalJane)
        .wrap_err("She hasn't got an instrument")
        .suggestion("Give her an instrument?")?
}

// ***************************************************************************
// Main
// ***************************************************************************

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

    println!("\nMay you be blessed by a tickle from his noodly appendages...\n");
    Ok(())
}

