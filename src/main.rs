use rust_playground::utilities::math;

/// The main function of the rust_playground crate.
fn main() {
    println!("Hello, Rust!");

    // The following line triggers a warning by the clippy linter, but not by the rustc linters
    let foo = math::fooFct();
    println!("foo: {foo}");

    let result: i64 = math::add(2, 3);
    println!("result: {result}");
}
