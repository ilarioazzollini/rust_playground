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
/// use utilities::math;
///
/// let result: u64 = math::add(2, 3);
/// println!("result: {result}");  // result: 5
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
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
