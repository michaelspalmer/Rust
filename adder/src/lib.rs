//! The 'adder' crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, adder::add_two(2));
//! ```

/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use adder::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```

extern crate test;


pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    
    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
    
    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));

    }
}
