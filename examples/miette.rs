// ***************************************************************************
// About
// ***************************************************************************

//! Error reports with Miette
//
// Q: Can you customise an error message?
// A: Yes, usual thiserror fashion
//
// Q: How to get file, line #?
//
// Q: How to get backtraces?
//
// Q: Span traces?
//
// Q: How to print the error?
// A: println!("{}", e);
//
// Q: How to print the report?
// A: println!("{:?}", e);
//
// Q: Can you convert a foreign error into a miette error?
// A: Yes, use `into_diagnostic()` on it
//
// Q: Can you read in a source file for labelling?
// A: Not yet, only strings. See https://github.com/zkat/miette/issues/297

// ***************************************************************************
// Dependencies
// ***************************************************************************

use log::{error, info};
use thiserror::Error;
use miette::{Diagnostic, diagnostic, IntoDiagnostic, NamedSource, SourceSpan, WrapErr};

// ***************************************************************************
// Errors
// ***************************************************************************

#[derive(Error, Diagnostic, Debug)]
pub enum ExampleError {
    #[error("phooey the startup systems sucks! [{location}]")]
    #[diagnostic(code(StartupSystemSucks), help("Blame Daniel Stonier"), url("https://cricinfo.com"))]
    StartupSystemSucks { location: String },
    #[error("customised help, url [{location}]")]
    #[diagnostic(code(ErrorWithCustomHelp))]
    ErrorWithCustomHelp {
        #[help]
        help: Option<String>,
        location: String
    },
    #[error("error without diagnostic")]
    NoDiagnosticError,
    #[error("damn who wrote the first system!")]
    #[diagnostic(code(ErrorWithSourceCode))]
    ErrorWithSourceCode {
        #[source_code]
        src: NamedSource,
        #[label("Here")]
        span: SourceSpan
    },
}

// ***************************************************************************
// Systems
// ***************************************************************************

fn error_with_diagnostic() -> miette::Result<()> {
    info!("error_with_diagnostic()");
    Err(ExampleError::StartupSystemSucks { location: format!("{}:{}", file!(), line!()) })?
}

fn wrap_error_with_diagnostic() -> miette::Result<()> {
    info!("wrap_error_with_diagnostic()");
    Err(ExampleError::StartupSystemSucks { location: format!("{}:{}", file!(), line!()) })
    .wrap_err("Wrapped")
}

fn error_with_custom_help() -> miette::Result<()> {
    info!("error_with_custom_help()");
    Err(ExampleError::ErrorWithCustomHelp { 
        help: Some("Blame Nikita".to_string()),
        location: format!("{}:{}", file!(), line!())
    })?
}

fn error_into_diagnostic() -> miette::Result<()> {
    info!("error_into_diagnostic()");
    Err(ExampleError::NoDiagnosticError).into_diagnostic()
}

fn error_with_source_code_strings() -> miette::Result<()> {
    info!("error_with_source_code()");
    // can only pipe in strings for now, see Q&A above
    Err(ExampleError::ErrorWithSourceCode {
        src: NamedSource::new(format!("{}",file!()), "foo or bar\n"),
        span: (7, 3).into()
     })?
}

// ***************************************************************************
// Main
// ***************************************************************************

fn main() -> Result<(), Box<dyn std::error::Error>> { // miette::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    if let Err(report) = error_with_diagnostic() {
        info!("{}", report);  // just thiserror::Error output
        error!("\n\n{:?}", report);  // a miette full report
    }

    if let Err(report) = wrap_error_with_diagnostic() {
        error!("\n\n{:?}", report);
    }

    if let Err(report) = error_into_diagnostic() {
        error!("\n\n{:?}", report);
    }

    if let Err(report) = error_with_custom_help() {
        error!("\n\n{:?}", report);
    }

    if let Err(report) = error_with_source_code_strings() {
        if report.is::<ExampleError>() {
            error!("ExampleError detected:\n\n{:?}", report);
        } else {
            panic!("Unknown Error")
        }
    }

    // This shows <disabled> if RUST_BACKTRACE is not set
    println!("{:#?}", std::backtrace::Backtrace::capture() );

    println!("\nMay you be blessed by a tickle from his noodly appendages...\n");
    Ok(())
}
