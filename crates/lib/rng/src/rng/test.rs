/*
 * Dark Forge is a library and extension for Godot engine that implements the Blades in the Dark SRD by One Seven Design.
 * Copyright (C) 2025 Pierre Fouilloux, Hibiscus Collective
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along with this program.
 * If not, see https://www.gnu.org/licenses/.
 */
#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss, reason = "Allowing wrapping and sign loss in tests")]

use core::iter::Cycle;

use crate::rng::Random;

/// Macro for asserting that a value is approximately equal to an expected value.
///
/// This macro is useful for testing random number generators, where exact equality
/// cannot be guaranteed but approximate equality within a tolerance is sufficient.
///
/// # Arguments
///
/// * `$expect` - The expected value
/// * `$actual` - The actual value
/// * `$tolerance` - The maximum allowed difference between the expected and actual values
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate dfrng;
/// # fn main() {
/// // Assert that 100 is approximately equal to 102 with a tolerance of 5
/// assert_approx!(100, 102, 5);
///
/// // This would fail because 100 is not approximately equal to 110 with a tolerance of 5
/// // assert_approx!(100, 110, 5);
/// # }
/// ```
#[macro_export]
macro_rules! assert_approx {
    ($expect:expr, $actual:expr, $tolerance:expr) => {
        assert!(
            ($expect.checked_sub($actual).unwrap().unsigned_abs() < $tolerance),
            "expected: {} +/- {}, but got: {}",
            $expect,
            $tolerance,
            $actual
        );
    };
}

/// Implementation of the `Random` trait for `Cycle<I>` iterators.
///
/// This implementation is useful for testing, as it allows for predictable
/// "random" values by cycling through a fixed sequence.
///
/// # Type Parameters
///
/// * `T` - The type of values generated
/// * `I` - The type of iterator being cycled
///
/// # Examples
///
/// ```
/// use dfrng::rng::Random;
/// use std::iter::Cycle;
///
/// #[cfg(test)]
/// fn test_with_cycle() {
///     // Create a cycle of values 1, 2, 3
///     let mut cycle = [1, 2, 3].iter().cycle();
///
///     // Use the cycle as a random number generator
///     let value = Random::next(&mut cycle);
///     assert_eq!(*value, 1);
///
///     let values = Random::take(&mut cycle, 5);
///     assert_eq!(values, vec![&2, &3, &1, &2, &3]);
/// }
/// ```
impl<T: Clone, I: Iterator<Item = T> + Clone> Random<T> for Cycle<I> {
    #[expect(clippy::expect_used, reason = "Exception as a Cycle should never return None")]
    fn next(&mut self) -> T {
        Iterator::next(&mut self.clone()).expect("Cycle should not be empty")
    }

    fn take(&mut self, n: usize) -> Vec<T> {
        Iterator::take(&mut self.clone(), n).collect()
    }
}

/// A random number generator that always returns the same value.
///
/// This struct is useful for testing, as it allows for completely predictable
/// "random" values by always returning the same value.
///
/// # Type Parameters
///
/// * `T` - The type of values generated
///
/// # Examples
///
/// ```
/// #![cfg(test)]
/// use dfrng::rng::{Random, test::Repeat};
///
/// fn test_with_repeat() {
///     // Create a random number generator that always returns 42
///     let mut repeat = Repeat(42);
///
///     // Generate a "random" number
///     let value = repeat.next();
///     assert_eq!(value, 42);
///
///     // Generate multiple "random" numbers
///     let values = repeat.take(3);
///     assert_eq!(values, vec![42, 42, 42]);
/// }
/// ```
#[non_exhaustive]
pub struct Repeat<T: Clone>(pub T);

impl<T: Clone> Random<T> for Repeat<T> {
    /// Returns the stored value.
    fn next(&mut self) -> T {
        self.0.clone()
    }

    /// Returns a vector containing `n` copies of the stored value.
    fn take(&mut self, n: usize) -> Vec<T> {
        vec![self.0.clone(); n]
    }
}
