// ***************************************************************************
// About
// ***************************************************************************

//! Variants
//
// ***************************************************************************
// Dependencies
// ***************************************************************************

// ***************************************************************************
// Definitions
// ***************************************************************************

#[derive(Clone, Debug)]
pub enum Snafu {
    Foo,
    Bar,
    FooBar { value: i32 },
}

pub fn what_kind_of_foo(num: i32) -> Snafu {
    if num < -10 {
        Snafu::Foo
    } else if num > 10 {
        Snafu::Bar
    } else {
        Snafu::FooBar { value: num }
    }
}

// ***************************************************************************
// Main
// ***************************************************************************

fn main() { // miette::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    for num in [-11, 3, 21].iter() {
        match what_kind_of_foo(*num) {
            Snafu::Foo => println!("Foo"),
            Snafu::Bar => println!("Bar"),
            Snafu::FooBar { value } => println!("FooBar {}", value)
        };
    }
    println!("\nMay you be blessed by a tickle from his noodly appendages...\n");
}

