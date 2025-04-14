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

use crate::rng::{Random, UniformThreadRandom};
use std::sync::Mutex;

pub trait Dice {
    fn sides(&self) -> u8;
    fn roll(&self) -> u8;
    fn roll_pool(&self, pool: usize) -> Vec<u8>;
}

pub type D4<R> = D<4, R>;
pub type D6<R> = D<6, R>;
pub type D8<R> = D<8, R>;
pub type D10<R> = D<10, R>;
pub type D12<R> = D<12, R>;
pub type D20<R> = D<20, R>;
pub type D100<R> = D<100, R>;

pub struct D<const SIDES: u8, R: Random<u8>> {
    rng: Mutex<R>,
}

impl<const SIDES: u8, R: Random<u8>> D<SIDES, R> {
    pub fn new(rng: R) -> Self {
        Self { rng: Mutex::new(rng) }
    }
}

impl<const SIDES: u8> Default for D<SIDES, UniformThreadRandom<u8>> {
    fn default() -> Self {
        Self {
            rng: Mutex::new(UniformThreadRandom::new(1, SIDES).unwrap()),
        }
    }
}

impl<const SIDES: u8, R: Random<u8>> Dice for D<SIDES, R> {
    fn sides(&self) -> u8 {
        SIDES
    }

    fn roll(&self) -> u8 {
        self.roll_pool(1)[0]
    }

    fn roll_pool(&self, pool: usize) -> Vec<u8> {
        self.rng.lock().unwrap().take(pool)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx;
    use rstest::rstest;

    const SAMPLES: u32 = 1_000_000;
    const ERROR_TOLERANCE_PCT: f64 = 0.05;

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
        let mut buckets = vec![0u32; d.sides() as usize];

        for _ in 0..SAMPLES {
            buckets[(d.roll() - 1) as usize] += 1;
        }

        let approx_expect = SAMPLES / d.sides() as u32;
        let error_tolerance = (approx_expect as f64 * ERROR_TOLERANCE_PCT) as usize;

        assert_eq!(buckets.iter().sum::<u32>(), SAMPLES, "should have recorded {} samples", SAMPLES);
        for bucket in buckets {
            assert_approx!(approx_expect, bucket, error_tolerance);
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

        let approx_expect = SAMPLES / d.sides() as u32;
        let error_tolerance = (approx_expect as f64 * ERROR_TOLERANCE_PCT) as usize;

        assert_eq!(buckets.iter().sum::<u32>(), SAMPLES, "should have recorded {} samples", SAMPLES);
        for bucket in buckets {
            assert_approx!(approx_expect, bucket, error_tolerance);
        }
    }
}
