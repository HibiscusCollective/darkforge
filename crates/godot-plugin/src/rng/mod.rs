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
mod tests {
    use super::*;

    macro_rules! assert_approx {
        ($expect:expr, $actual:expr, $tolerance:expr) => {
            assert!(
                (($expect as isize - $actual as isize).abs() < $tolerance),
                "expected: {} +/- {}, but got: {}",
                $expect,
                $tolerance,
                $actual
            );
        };
    }

    const NUM: u32 = 1_000_000;

    #[test]
    fn should_distribute_values_evenly_when_sampling_single_values() {
        let mut rng = UniformThreadRandom::new(0, 9).unwrap();

        let mut buckets = [0u32; 10];
        for _ in 0..NUM {
            buckets[rng.next()] += 1;
        }

        assert_eq!(buckets.iter().sum::<u32>(), NUM, "should have recorded {} samples", NUM);
        for bucket in buckets {
            assert_approx!(NUM / 10, bucket, NUM as isize / 15);
        }
    }

    #[test]
    fn should_distribute_values_evenly_when_sampling_many_values() {
        let mut rng = UniformThreadRandom::new(0, 9).unwrap();

        let mut buckets = [0u32; 10];
        for n in rng.take(NUM as usize) {
            buckets[n] += 1;
        }

        assert_eq!(buckets.iter().sum::<u32>(), NUM, "should have recorded {} samples", NUM);
        for bucket in buckets {
            assert_approx!(NUM / 10, bucket, NUM as isize / 15);
        }
    }
}
