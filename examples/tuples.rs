// ***************************************************************************
// About
// ***************************************************************************

//! Enums and Variants
//
// ***************************************************************************
// Dependencies
// ***************************************************************************

// ***************************************************************************
// Definitions
// ***************************************************************************
trait And<B> {
    type Result;
    fn and(self, rhs: B) -> Self::Result;
}

impl<A, B> And<B> for (A,) {
    type Result = (A, B);
    fn and(self, rhs: B) -> (A, B) {
        (self.0, rhs)
    }
}

impl<A, B, C> And<C> for (A, B) {
    type Result = (A, B, C);
    fn and(self, rhs: C) -> (A, B, C) {
        (self.0, self.1, rhs)
    }
}

// ***************************************************************************
// Main
// ***************************************************************************

fn main() { // miette::Result<()> {
    let a = (1, 'a');
    let b = (2, 'b');

    // Not what we want
    let c = (a, b); // A two element tuple of tuples
    println!("c: {:?}", c);

    println!("{:?}", (a,).and(b).and(1));

    let d = tupleops::concat_tuples(a, b);
    println!("{:?}", d);
    println!("\nMay you be blessed by a tickle from his noodly appendages...\n");
}

