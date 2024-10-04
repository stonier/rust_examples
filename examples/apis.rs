// ***************************************************************************
// Investigating `fn foo(s: T)` where T can be:
//
// * AsRef<str>
// * Borrow<str>
// * Into<Cow<'a, str>>
// * Into<String>
//
// References:
// 
// * Q: AsRef<T>: permit a wide a variety of compatible types
// * Q: Borrow<T>: permit only hash-equivalent compatible types
// * &T: if you need a laser focused contract
//
// Owning:
//
// * Into: simple, but will always clone if a ref is passed in
// * Cow: complex, but allows a lazy runtime decision to clone or not if a ref is passed in
//
// ***************************************************************************

// ***************************************************************************
// Uses
// ***************************************************************************

use std::{
    borrow::{Borrow, Cow},
    path::{Path, PathBuf}
};

// ***************************************************************************
// Helpers
// ***************************************************************************

fn foo_asexplicitref(s: &str) {
   println!("What is foo: {:?}", s);
}

fn foo_asref(s: impl AsRef<str>) {

    // You *can* allocate and own from here, e.g.
    //
    //   let a: String = s.as_ref().to_owned();
    //
    // But this is completely redundant if the user moved s in, e.g.
    //
    //   let s = String::from("notbar");
    //   foo_asref(s);
    println!("What is foo: {:?}", s.as_ref());
}

fn foo_asborrow(s: impl Borrow<str>) {
    println!("What is foo: {:?}", s.borrow());
}

fn foo_into(s: impl Into<String>) {
    // Into forces a compile time decision
    //  -> always own

    // IF a borrow is passed in, it always clones
    println!("What is foo: {:?}", s.into());
}

fn foo_cow<'a>(s: impl Into<Cow<'a, str>>) {
    // Cow gives lets you make a lazy runtime decision
    //  -> sometimes own

    // IF you want to own it, explicitly do so
    // let s2 = s.into().into_owned();
    // println!("What is foo: {:?}", s2);

    // OR just use the ref if immutable access is sufficient
    println!("What is foo: {:?}", s.into());
    
}

fn path_asref(s: impl AsRef<Path>) {
     println!("What is path: {:?}", s.as_ref());
}

fn path_asborrow(s: impl Borrow<Path>) {
    println!("What is path: {:?}", s.borrow());
}

// ***************************************************************************
// Main
// ***************************************************************************

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // ********************
    // &T
    // ********************
    // Only permit the user to allocate or borrow
    //  + clear contract
    //  - no moves
    //  - user has to handle any compatible conversions
    // Don't try to internally allocate in the function -> inefficient.
    foo_asexplicitref("notbar");                // [✓] User can allocate on the stack
    foo_asexplicitref(&String::from("notbar")); // [✓] User can allocate on the heap
                                                // [x] User handles conversion to &
    let s = String::from("notbar");
    foo_asexplicitref(&s);                      // [✓] User can keep s for reuse
    foo_asexplicitref(&s);
    // foo_asexplicitref(s);     // [x] User cannot move in

    // ********************
    // AsRef
    // ********************
    // Allow a user to allocate, move in or borrow
    //  + lets the user decide
    //  + can handle a wide variety of input types
    // Don't try to internally allocate in the function -> inefficient.
    foo_asref("notbar");               // [✓] User can allocate on the stack
    foo_asref(String::from("notbar")); // [✓] User can allocate on the heap
    let s = String::from("notbar");
    foo_asref(&s);                     // [✓] User can keep s for reuse
    foo_asref(s);                      // [✓] User can prevent s from reuse
    // foo_asref(s);                   // [x] String has already moved

    // ********************
    // AsRef vs Borrow
    // ********************
    // Borrow has tighter constraints than AsRef
    path_asref(String::from("/opt/foo"));                 // [✓] Path is representable by a String
    // path_asborrow(String::from("/opt/foo"));           // [x] Path is not hash-equivalent to String
    path_asborrow(PathBuf::new().join("/opt/foo")); // [✓] Path is hash-equivalent to PathBuf

    // ********************
    // Borrow
    // ********************
    // Allow a user to allocate, move in or borrow
    //  + lets the user decide
    //  - can only handle hash-equivalent types
    // Don't try to allocate to an owned variable in the function -> inefficient.
    foo_asborrow("notbar");               // [✓] User can allocate on the stack
    foo_asborrow(String::from("notbar")); // [✓] User can allocate on the heap
    let s = String::from("notbar");
    // foo_asborrow(&s);                  // [x] No Borrow<str> for &String
    foo_asborrow(&*s);                    // [✓] User can keep s for reuse (WEIRD SYNTAX)
    foo_asborrow(s);                      // [✓] User can prevent s from reuse
    // foo_asborrow(s);                   // [x] String has already moved

    // ********************
    // Into
    // ********************
    // Allow a user to allocate, move in or borrow
    //  + lets the user decide
    //  - the function always clones borrows
    // Don't try to allocate to an owned variable in the function -> inefficient.
    foo_into("notbar");               // [✓] User can allocate on the stack
    foo_into(String::from("notbar")); // [✓] User can allocate on the heap
    let s = String::from("notbar");
    foo_into(&s);                     // [✓] User can keep s for reuse
    foo_into(s);                      // [✓] User can prevent s from reuse
    // foo_into(s);                   // [x] String has already moved

    // ********************
    // Cow
    // ********************
    // Allow a user to allocate, move in or borrow
    //  + lets the user decide
    //  - the function always clones borrows
    // Don't try to allocate to an owned variable in the function -> inefficient.
    foo_cow("notbar");               // [✓] User can allocate on the stack
    foo_cow(String::from("notbar")); // [✓] User can allocate on the heap
    let s = String::from("notbar");
    foo_cow(&s);                     // [✓] User can keep s for reuse
    foo_cow(s);                      // [✓] User can prevent s from reuse
    // foo_cow(s);                   // [x] String has already moved

    Ok(())
}
