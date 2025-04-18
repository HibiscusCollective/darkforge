/*
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
#![expect(clippy::min_ident_chars, reason = "Exception as dN is a tabletop convention")]

//! # Dice
//!
//! This module provides functionality for simulating dice rolls in tabletop games.
//!
//! The module offers:
//! - A [`Dice`] trait that defines the interface for all dice types
//! - A generic [`D`] struct that implements dice with any number of sides
//! - Type aliases for common dice types (D4, D6, D8, D10, D12, D20, D100)
//!
//! ## Examples
//!
//! ```
//! use darkforge_rng::dice::{Dice, D6};
//!
//! // Create a standard six-sided die
//! let d6 = D6::default();
//!
//! // Roll the die once
//! let result = d6.roll();
//! assert!(result >= 1 && result <= 6);
//!
//! // Roll multiple dice at once (a pool of 3 dice)
//! let results = d6.roll_pool(3);
//! assert_eq!(results.len(), 3);
//! ```

use std::sync::Mutex;

use crate::rng::{Random, UniformThreadRandom};

/// Trait defining the interface for dice objects.
///
/// This trait provides methods for querying the number of sides a die has
/// and for rolling the die to generate random values.
pub trait Dice {
    /// Rolls the die once and returns the result.
    ///
    /// The returned value will be between 1 and the number of sides (inclusive).
    ///
    /// # Examples
    ///
    /// ```
    /// use darkforge_rng::dice::{Dice, D6};
    ///
    /// let d6 = D6::default();
    /// let result = d6.roll();
    /// assert!(result >= 1 && result <= 6);
    /// ```
    fn roll(&self) -> u8;

    /// Rolls the die multiple times and returns all results as a vector.
    ///
    /// # Arguments
    ///
    /// * `pool` - The number of times to roll the die
    ///
    /// # Examples
    ///
    /// ```
    /// use darkforge_rng::dice::{Dice, D10};
    ///
    /// let d10 = D10::default();
    /// let results = d10.roll_pool(5);
    /// assert_eq!(results.len(), 5);
    /// for &result in &results {
    ///     assert!(result >= 1 && result <= 10);
    /// }
    /// ```
    fn roll_pool(&self, pool: usize) -> Vec<u8>;

    /// Returns the number of sides on this die.
    ///
    /// # Examples
    ///
    /// ```
    /// use darkforge_rng::dice::{Dice, D20};
    ///
    /// let d20 = D20::default();
    /// assert_eq!(d20.sides(), 20);
    /// ```
    fn sides(&self) -> u8;
}

/// A ten-sided die (pentagonal trapezohedron).
///
/// # Examples
///
/// ```
/// use darkforge_rng::dice::{Dice, D10};
///
/// let d10 = D10::default();
/// let result = d10.roll();
/// assert!(result >= 1 && result <= 10);
/// ```
pub type D10<R> = D<10, R>;

/// A hundred-sided die (often represented by two ten-sided dice).
///
/// # Examples
///
/// ```
/// use darkforge_rng::dice::{Dice, D100};
///
/// let d100 = D100::default();
/// let result = d100.roll();
/// assert!(result >= 1 && result <= 100);
/// ```
pub type D100<R> = D<100, R>;

/// A twelve-sided die (dodecahedron).
///
/// # Examples
///
/// ```
/// use darkforge_rng::dice::{Dice, D12};
///
/// let d12 = D12::default();
/// let result = d12.roll();
/// assert!(result >= 1 && result <= 12);
/// ```
pub type D12<R> = D<12, R>;

/// A twenty-sided die (icosahedron).
///
/// # Examples
///
/// ```
/// use darkforge_rng::dice::{Dice, D20};
///
/// let d20 = D20::default();
/// let result = d20.roll();
/// assert!(result >= 1 && result <= 20);
/// ```
pub type D20<R> = D<20, R>;

/// A four-sided die (tetrahedron).
///
/// # Examples
///
/// ```
/// use darkforge_rng::dice::{Dice, D4};
///
/// let d4 = D4::default();
/// let result = d4.roll();
/// assert!(result >= 1 && result <= 4);
/// ```
pub type D4<R> = D<4, R>;

/// A six-sided die (cube).
///
/// # Examples
///
/// ```
/// use darkforge_rng::dice::{Dice, D6};
///
/// let d6 = D6::default();
/// let result = d6.roll();
/// assert!(result >= 1 && result <= 6);
/// ```
pub type D6<R> = D<6, R>;

/// An eight-sided die (octahedron).
///
/// # Examples
///
/// ```
/// use darkforge_rng::dice::{Dice, D8};
///
/// let d8 = D8::default();
/// let result = d8.roll();
/// assert!(result >= 1 && result <= 8);
/// ```
pub type D8<R> = D<8, R>;

/// A generic die with a configurable number of sides.
///
/// This struct represents a die with `SIDES` number of sides, using a random
/// number generator of type `R` that implements the [`Random<u8>`](Random) trait.
///
/// The die is thread-safe due to the internal mutex protecting the random number generator.
/// Nevertheless, consider creating new dice for each thread to avoid waiting for the lock.
///
/// # Type Parameters
///
/// * `SIDES` - The number of sides on the die (must be a positive integer)
/// * `R` - The type of random number generator to use
///
/// # Examples
///
/// Creating a custom 30-sided die:
///
/// ```
/// use darkforge_rng::dice::{D, Dice};
/// use darkforge_rng::rng::UniformThreadRandom;
///
/// // Create a 30-sided die with the default random number generator
/// let d30 = D::<30, UniformThreadRandom<u8>>::default();
/// let result = d30.roll();
/// assert!(result >= 1 && result <= 30);
/// ```
///
/// Using a custom random number generator:
///
/// ```
/// use darkforge_rng::dice::{D, Dice};
/// use darkforge_rng::rng::UniformThreadRandom;
///
/// // Create a custom RNG for a 12-sided die
/// let rng = UniformThreadRandom::new(1, 12).unwrap();
/// let d12 = D::<12, _>::new(rng);
///
/// let result = d12.roll();
/// assert!(result >= 1 && result <= 12);
/// ```
pub struct D<const SIDES: u8, R: Random<u8>> {
    /// The random number generator used to generate the random values.
    ///
    /// # Locking
    ///
    /// This field is wrapped in a `Mutex` to ensure thread safety.
    /// This means that access to the random number generator is synchronized across threads.
    ///
    /// # Type
    ///
    /// The type of the field is `Mutex<R>`, where `R` is the type of the random number generator.
    /// This type is specified by the `R` type parameter of the `D` struct.
    ///
    rng: Mutex<R>,
}

impl<const SIDES: u8, R: Random<u8>> D<SIDES, R> {
    /// Creates a new die with the specified random number generator.
    ///
    /// # Arguments
    ///
    /// * `rng` - A random number generator that implements the [`Random<u8>`](Random) trait
    ///
    /// # Examples
    ///
    /// ```
    /// use darkforge_rng::dice::{D, Dice};
    /// use darkforge_rng::rng::UniformThreadRandom;
    ///
    /// let rng = UniformThreadRandom::new(1, 20).unwrap();
    /// let d20 = D::<20, _>::new(rng);
    ///
    /// let result = d20.roll();
    /// assert!(result >= 1 && result <= 20);
    /// ```
    #[inline]
    pub fn new(rng: R) -> Self {
        Self { rng: Mutex::new(rng) }
    }
}

impl<const SIDES: u8> Default for D<SIDES, UniformThreadRandom<u8>> {
    /// Creates a new die with the default random number generator.
    ///
    /// This uses a [`UniformThreadRandom<u8>`](UniformThreadRandom) configured to generate
    /// values between 1 and `SIDES` (inclusive).
    ///
    /// # Examples
    ///
    /// ```
    /// use darkforge_rng::dice::{D, Dice};
    ///
    /// let d6 = D::<6, _>::default();
    /// let result = d6.roll();
    /// assert!(result >= 1 && result <= 6);
    /// ```
    #[inline]
    fn default() -> Self {
        Self {
            #[expect(clippy::unwrap_used, reason = "Exception as this call failing would be a programming error")]
            rng: Mutex::new(UniformThreadRandom::new(1, SIDES).unwrap()),
        }
    }
}

impl<const SIDES: u8, R: Random<u8>> Dice for D<SIDES, R> {
    #[inline]
    fn roll(&self) -> u8 {
        *Option::unwrap(self.roll_pool(1).first())
    }

    #[inline]
    fn roll_pool(&self, pool: usize) -> Vec<u8> {
        #[expect(clippy::unwrap_used, reason = "Mutex being poisoned is a programming error")]
        self.rng.lock().unwrap().take(pool)
    }

    #[inline]
    fn sides(&self) -> u8 {
        SIDES
    }
}

#[cfg(test)]
#[expect(clippy::expect_used, reason = "Expect is allowed is tests")]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::assert_approx;

    const SAMPLES: u32 = 1_000_000;
    const ERROR_TOLERANCE_PCT: f64 = 5.0 / 100.0;

    type D42 = D<42, UniformThreadRandom<u8>>;

    #[rstest]
    #[case::d4(D4::default())]
    #[case::d6(D6::default())]
    #[case::d8(D8::default())]
    #[case::d10(D10::default())]
    #[case::d12(D12::default())]
    #[case::d20(D20::default())]
    #[case::d100(D100::default())]
    #[case::d42(D42::default())]
    fn should_distribute_values_evenly_when_sampling_single_values(#[case] d: impl Dice) {
        let mut buckets = vec![0u32; d.sides().into()];

        for _ in 0..SAMPLES {
            buckets[(d.roll() - 1) as usize] += 1;
        }

        let approx_expect: f64 = SAMPLES.checked_div(d.sides().into()).expect("should not overflow").into();

        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation, reason = "Exception as this is a test")]
        let error_tolerance = (approx_expect * ERROR_TOLERANCE_PCT) as u64;

        assert_eq!(buckets.iter().sum::<u32>(), SAMPLES, "should have recorded {SAMPLES} samples");

        #[expect(clippy::cast_possible_truncation, reason = "Exception as this is a test")]
        for bucket in buckets {
            assert_approx!(approx_expect as i64, i64::from(bucket), error_tolerance);
        }
    }

    #[rstest]
    #[case::d4(D4::default())]
    #[case::d6(D6::default())]
    #[case::d8(D8::default())]
    #[case::d10(D10::default())]
    #[case::d12(D12::default())]
    #[case::d20(D20::default())]
    #[case::d100(D100::default())]
    #[case::d42(D42::default())]
    fn should_distribute_values_evenly_when_sampling_many_values(#[case] d: impl Dice) {
        let mut buckets = vec![0u32; d.sides() as usize];

        for n in d.roll_pool(SAMPLES as usize) {
            buckets[(n - 1) as usize] += 1;
        }

        let approx_expect = f64::from(SAMPLES) / f64::from(d.sides());

        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation, reason = "Exception as this is a test")]
        let error_tolerance = (approx_expect * ERROR_TOLERANCE_PCT) as u64;

        assert_eq!(buckets.iter().sum::<u32>(), SAMPLES, "should have recorded {SAMPLES} samples");

        #[expect(clippy::cast_possible_truncation, reason = "Exception as this is a test")]
        for bucket in buckets {
            assert_approx!(approx_expect as i64, i64::from(bucket), error_tolerance);
        }
    }
}
