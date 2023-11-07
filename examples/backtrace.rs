// ***************************************************************************
// About
// ***************************************************************************

//! Backtraces
//
// ***************************************************************************
// Dependencies
// ***************************************************************************

use backtrace::Backtrace;
use log::info;

// ***************************************************************************
// Definitions
// ***************************************************************************

fn crate_backtrace() -> Backtrace {
    info!("********************************************************");
    info!("Crate Version of Backtrace");
    info!("********************************************************");
    Backtrace::new()
}

fn std_backtrace() -> std::backtrace::Backtrace {
    info!("********************************************************");
    info!("* Std Backtrace");
    info!("********************************************************");
    std::backtrace::Backtrace::capture()
}

// ***************************************************************************
// Main
// ***************************************************************************

fn main() { // miette::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let crate_bt = crate_backtrace();
    info!("\n{:#?}", crate_bt);  // always shows

    let std_bt = std_backtrace();

    if let std::backtrace::BacktraceStatus::Captured = std_bt.status() {
        // only with RUST_BACKTRACE=1
        // make sure to pretty print it with that hash identifier! {:#?}
        info!("\n{:#?}", std_bt);
    } else {
        info!("Backtrace is disabled.");
        info!("Run with RUST_BACKTRACE=1");
    }

    println!("\nMay you be blessed by a tickle from his noodly appendages...\n");
}
