// ***************************************************************************
// About
// ***************************************************************************

//! Enums and Variants
//
// ***************************************************************************
// Dependencies
// ***************************************************************************

use log::error;
use thiserror::Error;

// ***************************************************************************
// Definitions
// ***************************************************************************

#[derive(Clone, Debug, Error)]
pub enum Foo {
    #[error("A Error")]
    A(A),
    #[error("B Error")]
    B
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

// ***************************************************************************
// Main
// ***************************************************************************

fn main() { // miette::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    println!("matchin foo");
    let foo = Foo::A(
        A {
            name: "foo".to_string(),
            ..A::default()
        }
    );
    match foo {
        Foo::A(ref a) => println!("Name: {}\nNick: {}", a.name, a.nick),
        _ => ()
    }
    if let Foo::A(ref a) = foo {
        println!("Name: {}\nNick: {}", a.name, a.nick)
    };

    println!("\nMay you be blessed by a tickle from his noodly appendages...\n");
}

