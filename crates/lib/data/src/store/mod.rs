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
mod sql;

use std::{error, fmt::Debug, future::Future, path::PathBuf};

use serde::Deserialize;
use thiserror::Error;

/// Error type for store operations in the data crate.
#[derive(Debug, Error)]
#[error(transparent)]
pub struct StoreError(#[from] Box<dyn error::Error>);

/// Trait for values that can be converted to and from storage representations.
pub trait Value: Sized {
    /// Error type for value conversion.
    type Error: error::Error + Debug;
    /// Result type for value conversion.
    type Result<T>: Into<Result<T, Self::Error>>;
    /// The underlying value type.
    type ValueType;

    /// Converts the value to another type if possible.
    fn convert_to<T: TryFrom<Self::ValueType>>(&self) -> Self::Result<Option<T>>;
}

/// Trait for queries that can be run against a store.
pub trait Query<'a, S: Store, T: Deserialize<'a> = ()> {
    /// Runs the query on the given store.
    fn run(&self, store: &mut S) -> impl Future<Output = S::Result<Vec<T>>>;
}

/// Marker trait for key types used in stores.
pub trait Key {}

/// Trait for store backends.
pub trait Store {
    /// Error type for store operations.
    type Error: error::Error + Debug;
    /// Result type for store operations.
    type Result<T>: Into<Result<T, Self::Error>>;
}

/// Trait for database migrators.
pub trait Migrator {
    /// Error type for migration operations.
    type Error: error::Error + Debug;
    /// Result type for migration operations.
    type Result<T>: Into<Result<T, Self::Error>>;

    /// Applies migrations from the given path.
    fn apply(&self, path: impl Into<PathBuf>) -> impl Future<Output = Self::Result<()>>;
}
