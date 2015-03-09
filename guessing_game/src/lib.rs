//! The compare module has 1 method, cmp, used to compare two numbers.
//!
//! # Examples
//!
//! ```
//! use guessing_game::compare;
//! use std::cmp::Ordering;
//! 
//! assert_eq!(Ordering::Less, compare::cmp(12, 122));
//! ```


pub mod compare;