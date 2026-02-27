use utilities::math;

fn main() {
    // This unused variable triggers a warning by the rustc linter
    let i = 0;

    println!("Hello, world!");

    let result = math::add(2, 2);

    println!("result: {}", result);
}
