use rust_playground::utilities::math;

fn main() {
    println!("Hello, Rust!");

    // The following line triggers a warning by the clippy linter, but not by the rustc linters
    let foo = math::fooFct();

    println!("foo: {foo}");
}
