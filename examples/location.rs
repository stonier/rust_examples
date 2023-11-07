// ***************************************************************************
// About
// ***************************************************************************

//! Enums and Variants
//
// ***************************************************************************
// Dependencies
// ***************************************************************************

use log::info;
use std::panic::Location;

// ***************************************************************************
// Definitions
// ***************************************************************************

/// Returns the [`Location`] at which it is called.
#[track_caller]
fn get_caller_location() -> &'static Location<'static> {
    Location::caller()
}

// From https://twitter.com/mitsuhiko/status/1486126762346045442
#[doc(hidden)]
#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of_val<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        // This has the form module_name::function_name
        let mut name = type_name_of_val(f).strip_suffix("::f").unwrap_or("");
        // Closure handling?
        while let Some(rest) = name.strip_suffix("::{{closure}}") {
            name = rest
        }
        // Strip the module_name
        if let Some((_, function_name)) = name.rsplit_once("::") {
            name = function_name
        }
        name
    }};
}

fn do_rae_me() {
    info!("Inside do_rae_me())");
    info!("{:?}", get_caller_location());
    info!("{}", get_caller_location());
    info!("function name: {}", function_name!());
}

// ***************************************************************************
// Main
// ***************************************************************************

fn main() { // miette::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    do_rae_me();

    println!("\nMay you be blessed by a tickle from his noodly appendages...\n");
}

