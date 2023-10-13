// ***************************************************************************
// About
// ***************************************************************************

//! Error reports with Eyre
//
// Eyre uses span tracing, which is a cheaper Backtrace
//
// Q: Do I need to tracing?
// A: No, you still get the base report + backtrace if you want it. Tracing
//    gets you the backtrace.

// ***************************************************************************
// Dependencies
// ***************************************************************************

use color_eyre::{
    Section,
    eyre::{Result, WrapErr}
};
use log::{error, info};
// use tracing::{info, error, instrument};
use thiserror::Error;

// ***************************************************************************
// Errors
// ***************************************************************************

#[derive(Error, Debug)] // , Diagnostic)]
pub enum ExampleError {
    #[error("Jane eyre is not plain!")]
    PlainJane,

    #[error("Could not play music")]
    MusicalJane,

    // #[error("show me where the action is! [{location}]")]
    // ErrorwithLocation { location: String },
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

// #[instrument]
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
    // install_tracing();

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


// ***************************************************************************
// Helpers
// ***************************************************************************

// fn install_tracing() {
//     use tracing_error::ErrorLayer;
//     use tracing_subscriber::prelude::*;
//     use tracing_subscriber::{fmt, EnvFilter};

//     let fmt_layer = fmt::layer().with_target(false);
//     let filter_layer = EnvFilter::try_from_default_env()
//         .or_else(|_| EnvFilter::try_new("info"))
//         .unwrap();

//     tracing_subscriber::registry()
//         .with(filter_layer)
//         .with(fmt_layer)
//         .with(ErrorLayer::default())
//         .init();
// }