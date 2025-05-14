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
use std::{path::PathBuf, sync::Arc};

use libsql::Database;
use libsql_migration::errors::LibsqlDirMigratorError;
use thiserror::Error;

use crate::store::Migrator;

/// Handles database migrations for `SQlite` using libsql.
pub struct SqliteMigrator {
    db: Arc<Database>,
}

impl SqliteMigrator {
    /// Creates a new `SqliteMigrator` with the given database.
    pub fn new(db: Arc<Database>) -> SqliteMigrator {
        SqliteMigrator { db }
    }
}

/// Error type for migration operations in `SQlite`.
#[derive(Error, Debug)]
pub enum MigrationError {
    /// Error connecting to the database.
    #[error("connection error: {0}")]
    ConnectionError(#[from] libsql::Error),
    /// Error during migration.
    #[error("migration error: {0}")]
    DatabaseError(#[from] LibsqlDirMigratorError),
}

impl Migrator for SqliteMigrator {
    type Error = MigrationError;
    type Result<T> = Result<T, MigrationError>;

    /// Applies migrations from the given path.
    async fn apply(&self, path: impl Into<PathBuf>) -> Self::Result<()> {
        let conn = self.db.connect()?;
        libsql_migration::dir::migrate(&conn, path.into()).await?;

        Ok(())
    }
}
