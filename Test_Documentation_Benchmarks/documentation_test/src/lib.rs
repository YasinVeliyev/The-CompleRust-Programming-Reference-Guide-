//doctest
//!This crate provides functionality for adding things
//!
//! #Examples
//! ```
//! use documentation_test::sum;
//!
//! let work_a=4;
//! let work_b=34;
//! let total_work=sum(work_a,work_b);
//! ```
//!

/// Sum two arguments
///
/// #Examples
///
/// ```
/// use documentation_test::sum;
/// assert_eq!(sum(1,1),2);
/// ```

pub fn sum(a: i8, b: i8) -> i8 {
    a + b
}
