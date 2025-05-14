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

//! Serialization/deserialization helpers with unified error handling. All types implementing `Serialize` and `Deserialize` from serde should be automatically compatible.

use std::io::{Read, Write};

use anyhow::anyhow;
use thiserror::Error;

/// Error type for serialization and deserialization failures.
#[derive(Error, Debug)]
pub enum CodecError {
    /// Error during serialization.
    #[error("failed to serialize: {0}")]
    Serialize(#[source] anyhow::Error),
    /// Error during deserialization.
    #[error("failed to deserialize: {0}")]
    Deserialize(#[source] anyhow::Error),
}

/// Result type for codec operations.
pub type Result<T> = std::result::Result<T, CodecError>;

/// Trait for serializing types to JSON.
pub trait JSONSerialize: serde::Serialize {
    /// Serialize self as JSON to the provided writer.
    ///
    /// # Errors
    ///
    /// Returns [`CodecError::Serialize`] if serialization fails.
    fn to_json(&self, w: &mut impl Write) -> Result<()> {
        serde_json::to_writer(w, self).map_err(|e| CodecError::Serialize(anyhow!(e)))
    }
}

/// Trait for deserializing types from JSON.
pub trait JSONDeserialize: serde::de::DeserializeOwned {
    /// Deserialize self from the given JSON reader.
    ///
    /// # Errors
    ///
    /// Returns [`CodecError::Deserialize`] if deserialization fails.
    fn from_json(json: impl Read) -> Result<Self> {
        serde_json::from_reader(json).map_err(|e| CodecError::Deserialize(anyhow!(e)))
    }
}

impl<T> JSONSerialize for T where T: serde::Serialize {}
impl<T> JSONDeserialize for T where T: serde::de::DeserializeOwned {}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use proptest_derive::Arbitrary;
    use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

    use super::*;

    #[derive(Arbitrary, Debug, Clone, PartialEq, SerdeSerialize, SerdeDeserialize)]
    struct DummyType {
        a: i32,
        b: Option<String>,
        c: Vec<u8>,
    }

    proptest! {
        #[test]
        fn test_to_json(dummy in any::<DummyType>()) {
            let mut json = Vec::new();

            dummy.to_json(&mut json).expect("should have serialized");

            let _: serde_json::Value = serde_json::from_slice(&json).expect("should be valid JSON");
            let deserialized: DummyType = serde_json::from_slice(&json).expect("should deserialize back");

            prop_assert_eq!(dummy, deserialized);
        }

        #[test]
        fn test_from_json(dummy in any::<DummyType>()) {
            let json = serde_json::to_vec(&dummy).expect("should be valid JSON");

            let deserialized = DummyType::from_json(json.as_slice()).expect("should have deserialized");

            prop_assert_eq!(dummy, deserialized);
        }
    }
}
