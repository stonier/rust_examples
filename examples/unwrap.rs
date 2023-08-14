// ***************************************************************************
// About
// ***************************************************************************

// Avoid unwrap

// ***************************************************************************
// Dependencies
// ***************************************************************************

// use std::time::Duration;
// use bevy_ecs::prelude::Res;

// ***************************************************************************
// Data Structures
// ***************************************************************************

#[derive(Copy, Clone, Default, Debug)]
pub struct Foo {
    pub bar: u32,
}

#[derive(Debug, Clone)]
struct CustomError;

fn substitute_for_none(foo: Option<Foo>) {
    let with_substitute = match foo {
        Some(f) => f,
        None => Foo { bar: 0 }
    };
    println!("With Substitute: {:?}", with_substitute);
}

fn if_let(foo: Option<Foo>) {
    if let Some(if_let) = foo {
        println!("If Let: foo is {:?}", if_let);
    } else {
        println!("If Let: foo was none");
    };
}

fn error_on_none(foo: Option<Foo>) -> Result<(), CustomError> {
    let without_error = match foo {
        Some(f) => f,
        None => {
            return Err(CustomError {});
        },
    };
    // can safely use now
    println!("Error On None: {:?}", without_error);
    Ok(())
}

// ***************************************************************************
// Main
// ***************************************************************************

fn main() -> std::result::Result<(), CustomError> {

    let five = Some(Foo { bar: 5 });
    let nothing = None;

    let unwrapped = five.unwrap();
    println!("Unwrapped: {:?}", unwrapped); // bad practice

    substitute_for_none(five);
    substitute_for_none(nothing);

    if_let(five);
    if_let(nothing);

    let _ = error_on_none(five)?;
//    let _ = error_on_none(nothing)?;  // prints "Error: CustomError" on stdout and then exits

    let without_error = match five {
        Some(f) => f,
        None => {
            return Err(CustomError {});
        },
    };
    println!("Without Error: {:?}", without_error);

    let _with_error = match nothing {
        Some(f) => f,
        None => {
            return Err(CustomError {}); // prints "Error: CustomError" on stdout and then exits
        },
    };

    println!("\n\nMay you be blessed by a tickle from his noodly appendages...");
    Ok(())
}
