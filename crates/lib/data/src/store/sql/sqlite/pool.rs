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

use bb8::ManageConnection;
use libsql::{Database, errors};

pub struct LibSqlConnectionManager(pub Database);

impl ManageConnection for LibSqlConnectionManager {
    type Connection = libsql::Connection;
    type Error = errors::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.0.connect()
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.execute("SELECT 1;", ()).await.map(|_| ())
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
