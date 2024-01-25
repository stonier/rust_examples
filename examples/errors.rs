// ***************************************************************************
// About
// ***************************************************************************

// How to propagate an arbitrary error through a method.

// This is a useful pattern for use with trait methods that don't know all
// the possible error types that will pass through it.
//
// See also: https://www.sheshbabu.com/posts/rust-error-handling/

// ***************************************************************************
// Uses
// ***************************************************************************

use thiserror::Error;

// ***************************************************************************
// Error Types
// ***************************************************************************

#[derive(Debug, Error)]
pub enum MixError {
    #[error("There is no spoon")]
    NoSpoon,
    #[error("These noodles are simulated")]
    ThirteenthFloor,
}

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("No Noodles")]
    NoNoodles,
    #[error("The colander is leaky, umm...")]
    LeakyColander,
}


#[derive(Debug, Error)]
pub enum TopLevelError {
    #[error("A known error kind.")]
    BoringError,
    #[error("A rethrow of an arbitrary (possibly unknown) error kind.")]
    ArbitraryError {
        source: Box<dyn std::error::Error>
    },
    #[error("Remap from FetchError")]
    FakeError {
        #[from]
        source: FetchError,
    }
}

#[derive(Debug)]
pub enum Ingredient {
    Noodles,
    Pasta,
    Water
}
// ***************************************************************************
// Methods
// ***************************************************************************

pub fn mix() -> Result<(), MixError> {
    Err(MixError::NoSpoon)
}

pub fn fetch_ingredient(ingredient: Ingredient) -> Result<(), FetchError> {
    match ingredient {
        Ingredient::Noodles => Err(FetchError::NoNoodles),
        Ingredient::Pasta => Ok(()),
        Ingredient::Water => Err(FetchError::LeakyColander)
    }
}

pub fn cook(ingredient: Ingredient) -> Result<(), TopLevelError> {
    fetch_ingredient(ingredient)?;
    Ok(())
}

fn main() {
    for ingredient in [Ingredient::Pasta, Ingredient::Water, Ingredient::Noodles] {
        println!("Ingredient: {:?}", ingredient);
        match cook(ingredient) {
            Ok(_) => println!(" -> Alles is ok"),
            Err(e) => println!(" -> {:?}", e)
        }
        println!("Mix:");
        match mix() {
            Ok(_) => println!(" -> Alles is ok"),
            Err(e) => println!(" -> {:?}", e)
        }
    }
    println!("\nMay you be blessed by a tickle from his noodly appendages...\n");
}
