/// Compare function: compares two numbers, returns Ordering.
/// 
/// # Examples
///
/// ```
/// use guessing_game::compare;
/// use std::cmp::Ordering;
///
/// assert_eq!(Ordering::Less, compare::cmp(2, 3));
/// ```

use std::cmp::Ordering;

pub fn cmp(a: u32, b: u32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;
    
    #[test]
    fn it_works_method_test_1() {
        assert_eq!(Ordering::Less, cmp(2, 3));
    }
    
    #[test]
    fn it_still_works_method_test_2() {
        assert_eq!(Ordering::Equal, cmp(1234, 1234));
    }
}