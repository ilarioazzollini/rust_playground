//! An example of library crate
//!
//! The `///` syntax is used to document the item present after it.
//! That's why it is called an outer documentation. There is another syntax: `//!`,
//! which is used to document the item it is present inside. It is called an inner documentation.
//! It is often used when documenting the entire crate, because nothing comes before it:
//! it is the root of the crate. So in order to document an entire crate, you need to use //! syntax.

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
/// let result: u64 = my_lib::add(2, 3);
/// assert_eq!(result, 5);
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
