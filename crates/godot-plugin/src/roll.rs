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
use crate::rng::{D6, Dice, Random};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Outcome {
    Failure(Vec<u8>),
    Partial(Vec<u8>),
    Success(Vec<u8>),
    Critical(Vec<u8>),
}

pub trait DiceRoll {
    fn try_roll(&mut self, pool: usize) -> Outcome;
}

impl<R: Random<u8>> DiceRoll for D6<R> {
    fn try_roll(&mut self, pool: usize) -> Outcome {
        if pool == 0 {
            check_untrained_d6_roll(self.roll_pool(2))
        } else if pool == 1 {
            check_d6_roll(self.roll())
        } else {
            check_d6_pool(self.roll_pool(pool))
        }
    }
}

fn check_untrained_d6_roll(rolls: Vec<u8>) -> Outcome {
    let outcome = check_d6_roll(*rolls.clone().iter().min().unwrap());
    match outcome {
        Outcome::Success(_) => Outcome::Success(rolls),
        Outcome::Partial(_) => Outcome::Partial(rolls),
        Outcome::Failure(_) => Outcome::Failure(rolls),
        _ => panic!("Invalid roll value"),
    }
}

fn check_d6_roll(roll: u8) -> Outcome {
    match roll {
        1..=3 => Outcome::Failure(vec![roll]),
        4 | 5 => Outcome::Partial(vec![roll]),
        6 => Outcome::Success(vec![roll]),
        _ => panic!("Invalid roll value"),
    }
}

fn check_d6_pool(rolls: Vec<u8>) -> Outcome {
    let mut sorted = rolls.clone();
    sorted.sort_unstable_by(|a, b| a.cmp(b).reverse()); // sort in descending order, so highest is first
    let value = (*sorted.first().unwrap_or(&1), *sorted.get(1).unwrap_or(&1));

    match value {
        (6, 6) => Outcome::Critical(rolls),
        (1..=3, _) => Outcome::Failure(rolls),
        (4, _) | (5, _) => Outcome::Partial(rolls),
        (6, _) => Outcome::Success(rolls),
        _ => panic!("Invalid roll value"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::{D6, Random, Repeat};
    use rstest::rstest;
    use std::iter::Iterator;

    #[rstest]
    #[case::one_die_6_success(Repeat(6), 1, Outcome::Success(vec![6]))]
    #[case::one_die_5_partial(Repeat(5), 1, Outcome::Partial(vec![5]))]
    #[case::one_die_4_partial(Repeat(4), 1, Outcome::Partial(vec![4]))]
    #[case::one_die_3_failure(Repeat(3), 1, Outcome::Failure(vec![3]))]
    #[case::one_die_2_failure(Repeat(2), 1, Outcome::Failure(vec![2]))]
    #[case::one_die_1_failure(Repeat(1), 1, Outcome::Failure(vec![1]))]
    #[case::two_dice_crit_success(Repeat(6), 2, Outcome::Critical(vec![6, 6]))]
    #[case::pool_crit_success(vec![1, 2, 3, 4, 5, 6, 6].into_iter().cycle(), 7, Outcome::Critical(vec![1, 2, 3, 4, 5, 6, 6]))]
    #[case::pool_success_6(vec![1, 2, 3, 4, 5, 6].into_iter().cycle(), 6, Outcome::Success(vec![1, 2, 3, 4, 5, 6]))]
    #[case::pool_partial_5(vec![1, 2, 3, 4, 5].into_iter().cycle(), 5, Outcome::Partial(vec![1, 2, 3, 4, 5]))]
    #[case::pool_partial_4(vec![1, 2, 3, 4].into_iter().cycle(), 4, Outcome::Partial(vec![1, 2, 3, 4]))]
    #[case::pool_failure_3(vec![1, 2, 3].into_iter().cycle(), 3, Outcome::Failure(vec![1, 2, 3]))]
    #[case::pool_failure_2(vec![1, 2].into_iter().cycle(), 2, Outcome::Failure(vec![1, 2]))]
    #[case::pool_failure_1(vec![1, 1].into_iter().cycle(), 2, Outcome::Failure(vec![1, 1]))]
    #[case::empty_dice_pool_take_lowest_and_fail(vec![1, 6].into_iter().cycle(), 0, Outcome::Failure(vec![1, 6]))]
    #[case::empty_dice_pool_take_lowest_and_partial(vec![4, 6].into_iter().cycle(), 0, Outcome::Partial(vec![4, 6]))]
    #[case::empty_dice_pool_take_lowest_and_success(vec![6, 6].into_iter().cycle(), 0, Outcome::Success(vec![6, 6]))]
    fn should_roll_dice(#[case] rng: impl Random<u8>, #[case] pool: u8, #[case] expect: Outcome) {
        let actual = D6::new(rng).try_roll(pool as usize);
        assert_eq!(expect, actual, "wanted {:?}, got: {:?}", expect, actual);
    }
}
