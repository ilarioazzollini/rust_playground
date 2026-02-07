/// This is a function name that triggers a warning by the rustc linter
pub fn fooFct() -> i32 {
    let result: i32 = 0;
    result
}

/// This is a short description. This function adds two integer numbers.
///
/// Then we should include a longer and more detailed description.
///
/// Then even more advanced explanations if necessary. Moreover,
/// it is useful to describe the cases in which the function can result in panic.
/// Last but not least, it is good practice to include at least one code example
/// that users can copy/paste to try it.
///
/// # Panics
/// The add function never panics. It's strong like that.
///
/// # Examples
///
/// ```rust
/// use rust_playground::utilities::math;
///
/// let result: i64 = math::add(2, 3);
/// println!("result: {result}");  // result: 5
/// ```
pub fn add(first: i64, second: i64) -> i64 {
    first + second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
