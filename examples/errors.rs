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
    #[error("There is no spoon, but {forks} forks.")]
    NoSpoon { forks: u64 },
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
        #[from]
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

// Option 1: Conversion is done on the fly in the Result.

// pub fn fetch_spoon() -> Result<(), Box<dyn std::error::Error>> {
//     Err( MixError::NoSpoon { forks: 4 })?;
//     Ok(())
// }
// Automatically converts the box to TopLevelError::Arbitrary
// pub fn mix() -> Result<(), TopLevelError> {
//     fetch_spoon()?;
//     Ok(())
// }

// Option 2: Convert manually

pub fn fetch_spoon() -> Result<(), MixError> {
    Err( MixError::NoSpoon { forks: 4 })?;
    Ok(())
}

pub fn mix() -> Result<(), TopLevelError> {
    match fetch_spoon() {
        Ok(_) => Ok(()),
        Err(e) => Err(
            TopLevelError::ArbitraryError {
                source: Box::new(e),
            }
        ),
    }
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
            Err(e) => {
                println!(" ---> {}", e);
                println!(" ---> {:?}", e);
                match e {
                    TopLevelError::FakeError { source } => {
                        println!(" -----> {}", source);
                        println!(" -----> {:?}", source);
                    },
                    _ => {}
                }
            },
        }
    }
    println!("Mix:");
    match mix() {
        Ok(_) => println!(" -> Alles is ok"),
        Err(TopLevelError::ArbitraryError { source }) => {
            println!(" -> ArbitraryError");
            // No downcasting needed if you just want to Debug or Display it
            println!(" ---> {}", source);
            println!(" ---> {:?}", source);
            if let Some(mix_err) = source.downcast_ref::<MixError>() {
                match mix_err {
                    MixError::NoSpoon { forks } => { println!(" ---> # Forks: {forks}"); },
                    _ => { println!(" ---> Nothing to see here"); }
                }
            }
        },
        Err(e) => println!(" -> {:?}", e)
    }
    println!("\nMay you be blessed by a tickle from his noodly appendages...\n");
}
