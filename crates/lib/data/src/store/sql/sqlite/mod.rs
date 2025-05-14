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
use std::result;

use bb8::{PooledConnection, RunError};
use serde::de::value::Error as SerdeError;
use thiserror::Error;

use crate::store::sql::sqlite::{migration::MigrationError, pool::LibSqlConnectionManager};

/// Module for database migration functionality.
mod migration;
/// Module for database connection pooling functionality.
mod pool;
/// Module for database store functionality.
mod store;

/// Type alias for a pooled database connection.
type Connection<'a> = PooledConnection<'a, LibSqlConnectionManager>;
/// Type alias for a result type that uses the `SqliteError` error type.
type Result<T> = result::Result<T, SqliteError>;

/// Error type for `SQlite` operations in the data store.
#[derive(Error, Debug)]
pub enum SqliteError {
    /// An error from the underlying libsql library.
    #[error("libsql error: {0}")]
    LibSql(#[from] libsql::Error),
    /// An error related to connection pooling.
    #[error("connection error: {0}")]
    Connection(#[from] RunError<libsql::Error>),
    /// An error during deserialization.
    #[error("deserialization error: {0}")]
    Deserialization(#[from] SerdeError),
    /// An error during migration.
    #[error(transparent)]
    MigrationError(#[from] MigrationError),
}
