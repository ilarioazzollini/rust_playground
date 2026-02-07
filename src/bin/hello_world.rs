//! This is a binary example. It is treated as a different crate.

fn main() {
    // This unused variable triggers a warning by the rustc linter
    let i = 0;

    println!("Hello, world!");
}
