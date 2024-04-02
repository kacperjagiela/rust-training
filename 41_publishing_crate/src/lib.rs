//! # Publishing crate
//!
//! `publishing_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to number given.
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = publishing_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(3);
        assert_eq!(result, 4);
    }
}
