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
use std::result;

use bb8::{PooledConnection, RunError};
use serde::de::value::Error as SerdeError;
use thiserror::Error;

use crate::store::sql::sqlite::{migration::MigrationError, pool::LibSqlConnectionManager};

mod migration;
mod pool;
mod store;

type Connection<'a> = PooledConnection<'a, LibSqlConnectionManager>;
type Result<T> = result::Result<T, SqliteError>;

#[derive(Error, Debug)]
pub enum SqliteError {
    #[error("libsql error: {0}")]
    LibSql(#[from] libsql::Error),
    #[error("connection error: {0}")]
    Connection(#[from] RunError<libsql::Error>),
    #[error("deserialization error: {0}")]
    Deserialization(#[from] SerdeError),
    #[error(transparent)]
    MigrationError(#[from] MigrationError),
}
