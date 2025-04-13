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
use rand::{
    Rng,
    distr::{Uniform, uniform, uniform::SampleUniform},
    prelude::Distribution,
    rngs::ThreadRng,
};
use std::result;
use thiserror::Error;

mod dice;

pub use dice::*;

type Result<T> = result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    UniformDistribution(#[from] uniform::Error),
}

pub trait Random<T, D: Distribution<T> = Uniform<T>, R: Rng = ThreadRng> {
    fn next(&mut self) -> T;
    fn take(&mut self, n: usize) -> Vec<T>;
}

pub struct UniformThreadRandom<T: SampleUniform> {
    distribution: Uniform<T>,
    rng: ThreadRng,
}

impl<T: SampleUniform> UniformThreadRandom<T> {
    fn new(low: T, high: T) -> Result<Self> {
        Ok(Self {
            distribution: Uniform::new_inclusive(low, high)?,
            rng: ThreadRng::default(),
        })
    }
}

impl<T: SampleUniform> Random<T> for UniformThreadRandom<T> {
    fn next(&mut self) -> T {
        self.distribution.sample(&mut self.rng)
    }

    fn take(&mut self, n: usize) -> Vec<T> {
        (&self.distribution).sample_iter(&mut self.rng).take(n).collect()
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! assert_approx {
    ($expect:expr, $actual:expr, $tolerance:expr) => {
        assert!(
            (($expect as isize - $actual as isize).unsigned_abs() < $tolerance),
            "expected: {} +/- {}, but got: {}",
            $expect,
            $tolerance,
            $actual
        );
    };
}
