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

//! Simple uuid abstraction to keep dependencies isolated. Includes some error handling utilities.

use std::fmt::Debug;

use thiserror::Error;

#[derive(Debug, PartialEq)]
/// Simple wrapper around `uuid::Uuid`.
pub struct Uuid(uuid::Uuid);

#[derive(Error, Debug, PartialEq)]
/// Errors for parsing or converting UUIDs.
pub enum UuidError {
    #[error("failed to parse uuid: {0}")]
    ParseError(#[from] uuid::Error),
}

impl TryFrom<[u8; 16]> for Uuid {
    type Error = UuidError;

    /// Try to create a `Uuid` from a 16-byte array.
    fn try_from(value: [u8; 16]) -> Result<Self, Self::Error> {
        Ok(Uuid(uuid::Uuid::from_bytes(uuid::Bytes::from(value))))
    }
}

impl TryFrom<String> for Uuid {
    type Error = UuidError;

    /// Try to parse a `Uuid` from a string.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(uuid::Uuid::parse_str(value.as_str())?.into())
    }
}

impl TryFrom<&str> for Uuid {
    type Error = UuidError;

    /// Try to parse a `Uuid` from a string slice.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(uuid::Uuid::parse_str(value)?.into())
    }
}

impl TryFrom<u128> for Uuid {
    type Error = UuidError;

    /// Create a `Uuid` from a 128-bit integer.
    fn try_from(value: u128) -> Result<Self, Self::Error> {
        Ok(uuid::Uuid::from_u128(value).into())
    }
}

impl From<uuid::Uuid> for Uuid {
    /// Convert from `uuid::Uuid` to this wrapper type.
    fn from(value: uuid::Uuid) -> Self {
        Uuid(value)
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Self::new()
    }
}

impl Uuid {
    /// Generate a new random UUID.
    pub fn new() -> Self {
        Uuid(uuid::Uuid::new_v4())
    }

    /// Get the zero UUID.
    pub fn zero() -> Self {
        Uuid(uuid::Uuid::nil())
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use rstest::rstest;
    use uuid::{Variant, Version, uuid};

    use super::*;

    #[rstest]
    #[case::from_bytes([0xf4, 0x7a, 0xc1, 0x0b, 0x58, 0xcc, 0x43, 0x72, 0xa5, 0x67, 0x0e, 0x02, 0xb2, 0xc3, 0xd4, 0x79], uuid!("f47ac10b-58cc-4372-a567-0e02b2c3d479").into())]
    #[case::from_string_hyphenated("f47ac10b-58cc-4372-a567-0e02b2c3d479", uuid!("f47ac10b-58cc-4372-a567-0e02b2c3d479").into())]
    #[case::from_string_hyphenated_uppercase("F47AC10B-58CC-4372-A567-0E02B2C3D479", uuid!("f47ac10b-58cc-4372-a567-0e02b2c3d479").into())]
    #[case::from_string_no_hyphens("f47ac10b58cc4372a5670e02b2c3d479", uuid!("f47ac10b-58cc-4372-a567-0e02b2c3d479").into())]
    #[case::from_string_no_hyphens_uppercase("F47AC10B58CC4372A5670E02B2C3D479", uuid!("f47ac10b-58cc-4372-a567-0e02b2c3d479").into())]
    #[case::from_u128(0xf47a_c10b_58cc_4372_a567_0e02_b2c3_d479_u128, uuid!("f47ac10b-58cc-4372-a567-0e02b2c3d479").into())]
    fn test_parse_uuid<T>(#[case] input: T, #[case] expect: Uuid)
    where
        T: TryInto<Uuid, Error = UuidError> + Debug,
    {
        let actual = input.try_into().expect("should have parsed uuid");
        assert_eq!(expect, actual);
    }

    proptest! {
        #[test]
        fn uuid_new_is_valid_v4(_ in 0u8..1) {
            let uuid = Uuid::new();

            assert_eq!(uuid.0.get_version(), Some(Version::Random));
            assert_eq!(uuid.0.get_variant(), Variant::RFC4122);
            assert_ne!(uuid.0.as_u128(), 0);
        }

        #[test]
        fn uuid_default_is_valid_v4_uuid(_ in 0u8..1) {
            let uuid = Uuid::default();

            assert_eq!(uuid.0.get_version(), Some(Version::Random));
            assert_eq!(uuid.0.get_variant(), Variant::RFC4122);
            assert_ne!(uuid.0.as_u128(), 0);
        }
    }

    #[test]
    fn test_from_uuid() {
        let uuid = uuid!("f47ac10b-58cc-4372-a567-0e02b2c3d479");
        let actual = Uuid::from(uuid);

        assert_eq!(Uuid(uuid), actual);
    }

    #[test]
    fn uuid_zero_is_zero() {
        let zero = Uuid::zero();
        assert_eq!(zero.0.as_u128(), 0);
    }
}
