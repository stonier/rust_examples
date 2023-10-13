// ***************************************************************************
// About
// ***************************************************************************

//! Error reports with Eyre
//
// Eyre uses span tracing, which is a cheaper Backtrace

// ***************************************************************************
// Dependencies
// ***************************************************************************

use color_eyre::{
    Section,
    eyre::{Result, WrapErr}
};
use tracing::{info, error, instrument};
use thiserror::Error;

// ***************************************************************************
// Errors
// ***************************************************************************

#[derive(Error, Debug)] // , Diagnostic)]
pub enum ExampleError {
    #[error("jane eyre is not plain!")]
    PlainJane,

    #[error("show me where the action is! [{location}]")]
    ErrorwithLocation { location: String },
}

// ***************************************************************************
// Systems
// ***************************************************************************

#[instrument]
fn error_plain_jane() -> Result<()> {
    info!("error_plain_jane()");
    Err(ExampleError::PlainJane)
        .wrap_err("Found Plain Jane")
        .suggestion("You can't think for yourself?")?
}

// ***************************************************************************
// Main
// ***************************************************************************

fn main() -> Result<(), Box<dyn std::error::Error>> { // miette::Result<()> {
    std::env::set_var("RUST_LOG", "info");

    install_tracing();
    color_eyre::install()?;

    if let Err(report) = error_plain_jane() {
        error!("\n\n{:?}", report);
    };

    println!("\nMay you be blessed by a tickle from his noodly appendages...\n");
    Ok(())
}


// ***************************************************************************
// Helpers
// ***************************************************************************

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}