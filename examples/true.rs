#[macro_use]
extern crate rust_contracts;

#[requires(true)]
#[ensures(true)]
fn id(x: i32) -> i32 { x }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check that the behaviour of the function is unchanged
    for i in -50..50 {
        assert_eq!(id(i), i);
    }
    Ok(())
}
