extern crate guessing_game;

use guessing_game::compare;
use std::cmp::Ordering;

#[test]
fn it_works_test_case_1() {
    assert_eq!(Ordering::Greater, compare::cmp(3, 2));
    assert_eq!(Ordering::Less, compare::cmp(100, 200));
}

#[test]
#[should_fail(expected = "assertion failed")]
fn it_still_works_test_case_2() {
    assert_eq!(Ordering::Equal, compare::cmp(1234, 4321));
}