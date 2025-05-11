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

pub struct SqliteMigrator {
    db: Arc<Database>,
}

impl SqliteMigrator {
    pub fn new(db: Arc<Database>) -> SqliteMigrator {
        SqliteMigrator { db }
    }
}

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("connection error: {0}")]
    ConnectionError(#[from] libsql::Error),
    #[error("migration error: {0}")]
    DatabaseError(#[from] LibsqlDirMigratorError),
}

impl Migrator for SqliteMigrator {
    type Error = MigrationError;
    type Result<T> = Result<T, MigrationError>;

    async fn apply(&self, path: impl Into<PathBuf>) -> Self::Result<()> {
        let conn = self.db.connect()?;
        libsql_migration::dir::migrate(&conn, path.into()).await?;

        Ok(())
    }
}
