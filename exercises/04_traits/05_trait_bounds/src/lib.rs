// TODO: Add the necessary trait bounds to `min` so that it compiles successfully.
//   Refer to the documentation of the `std::cmp` module for more information on the traits you might need.
//
// Note: there are different trait bounds that'll make the compiler happy, but they come with
// different _semantics_. We'll cover those differences later in the course when we talk about ordered
// collections (e.g. BTreeMap).

use std::fmt::Debug;

/// Return the minimum of two values.

trait Min<T> {
    fn min(left:T, right:T) -> T;
}

pub fn min<T: Debug + PartialOrd >(left: T, right: T) -> T {
// pub fn min<T>(left: T, right: T) -> T where T: Debug + PartialEq + PartialOrd {
    if left <= right {
        left
    } else {
        right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
        assert_eq!(2, min(2,3));
    }
}
