fn main() {
    let number_1: u64 = 2;
    let number_2: u64 = 1;

    let result: u64 = my_lib::add(number_1, number_2);
    println!(
        "Added numbers '{}' and '{}', and got the result: '{}'",
        number_1, number_2, result
    );
}
