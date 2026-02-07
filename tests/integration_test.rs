use rust_playground::utilities::math;

#[test]
fn it_adds_two() {
    let result = math::add(2, 2);
    assert_eq!(result, 4);
}
