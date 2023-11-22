// ***************************************************************************
// About
// ***************************************************************************

//! Dereferencing
//
// ***************************************************************************
// Dependencies
// ***************************************************************************

use std::ops::{Deref, DerefMut};

// ***************************************************************************
// Errors
// ***************************************************************************

#[derive(Copy, Clone, Debug, Default)]
struct FooBar {
    foo: f32,
    bar: f32,
    foobar: f32,
}

impl FooBar {
    pub fn doit(&mut self) {
        self.foobar = self.foo + self.bar;
    }
}

#[derive(Debug)]
struct SuperFooBar {
    inner: FooBar
}

impl Deref for SuperFooBar {
    type Target = FooBar;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for SuperFooBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// NewType Pattern
//
// https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html
//
// This is ok if you aren't accessing the underlying type directly (as in
// the example linked above). If however, you need to access that type
// directly, you end up with '.0's everywhere or passthrough methods
// defined for everything. Meh.
//
#[derive(Debug)]
struct NewFooBar(FooBar);

// ***************************************************************************
// Main
// ***************************************************************************

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut foobar = FooBar {
        foo: 3.0,
        bar: 5.0,
        foobar: 1.0
    };
    println!("Foobar: {foobar:?}");
    foobar.doit();
    println!("Foobar: {foobar:?}");

    let mut s1 = SuperFooBar {
        inner: FooBar {
            foo: 2.1,
            bar: 3.1,
            foobar: 3.0
        }
    };
    println!("SuperFoobar: {:?}", *s1);
    s1.doit();
    println!("SuperFoobar: {:?}", *s1);

    let mut newfoobar = NewFooBar(FooBar {
        foo: 3.1,
        bar: 3.1,
        foobar: 3.0
    });
    println!("NewFoobar: {:?}", newfoobar);
    newfoobar.0.doit();
    println!("NewFoobar: {:?}", newfoobar);
}

