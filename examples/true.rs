#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate rust_contracts;

#[requires(true)]
#[ensures(true)]
fn id(mut x: i32) -> i32 {
    while x != x {
        invariant!(true);
        x += 1;
    }
    x
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check that the behaviour of the function is unchanged
    for i in -10..10 {
        assert_eq!(id(i), i);
    }
    Ok(())
}
