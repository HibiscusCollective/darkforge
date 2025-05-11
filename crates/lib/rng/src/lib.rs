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

//! # DFRNG - Darkforge Random Number Generation
//!
//! A library for random number generation focused on tabletop gaming applications.
//! Built as part of the Darkforge project at Hibiscus Games. It should still be applicable to
//! different game systems.
//!
//! This crate provides utilities for:
//! - Dice simulation with various numbers of sides
//! - Random number generation with different distributions
//!
//! ## Modules
//!
//! - [`dice`]: Dice simulation for tabletop gaming
//! - [`rng`]: Random number generation
//!
//! ## Examples
//!
//! ```
//! use darkforge_rng::dice::{Dice, D20};
//! use darkforge_rng::rng::Random;
//!
//! // Create a 20-sided die
//! let d20 = D20::default();
//!
//! // Roll the die
//! let roll = d20.roll();
//! assert!(roll >= 1 && roll <= 20);
//!
//! // Roll multiple times
//! let rolls = d20.roll_pool(3);
//! assert_eq!(rolls.len(), 3);
//! ```

use core::result;

use rng::RngError;
use thiserror::Error;

pub mod cards;
pub mod dice;
pub mod rng;

/// Errors that can occur in DFRNG operations.
///
/// This enum represents the various errors that can occur when using the DFRNG library.
/// It wraps errors from the random number generation module.
///
/// # Examples
///
/// ```
/// use darkforge_rng::rng::{UniformThreadRandom, RngError};
/// use darkforge_rng::DFRngError;
///
/// // This will fail because high < low
/// let err = UniformThreadRandom::<u8>::new(100, 1).expect_err("should have failed");
/// match err {
///     DFRngError::RngError(e) => {
///         // Handle the RNG error
///         match e {
///             RngError::InvalidDistribution(_) => {
///                 println!("Invalid distribution parameters");
///             }
///         }
///     }
///     _ => panic!("unexpected error type")
/// }
/// ```
#[derive(Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum DFRngError {
    /// An error occurred in the random number generation.
    ///
    /// This wraps errors from the `rng` module, such as invalid distribution parameters.
    /// See `rng::RngError` for more details on specific error types.
    #[error(transparent)]
    RngError(#[from] RngError),
}

/// A specialised Result type for DFRNG operations.
///
/// This type is used throughout the crate to return either a successful value
/// or an error that occurred during the operation.
///
/// # Examples
///
/// ```
/// use darkforge_rng::rng::UniformThreadRandom;
/// use darkforge_rng::Result;
///
/// fn create_random_generator() -> Result<UniformThreadRandom<u8>> {
///     UniformThreadRandom::new(1, 100)
/// }
///
/// let generator = create_random_generator().expect("Failed to create random generator");
/// ```
pub type Result<T> = result::Result<T, DFRngError>;
