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

use bb8::Pool;
use libsql::{Value, de, params, params::IntoValue};
use serde::Deserialize;

use crate::store::{
    Query, Store,
    sql::{
        Param, Params, SqlQuery,
        sqlite::{Result, SqliteError, pool::LibSqlConnectionManager},
    },
};

pub struct SqliteStore {
    pool: Pool<LibSqlConnectionManager>,
}

trait IntoParams {
    fn to_params(&self) -> Result<params::Params>;
}

impl Store for SqliteStore {
    type Error = SqliteError;
    type Result<T> = result::Result<T, SqliteError>;
}

impl SqliteStore {
    pub fn new(pool: Pool<LibSqlConnectionManager>) -> SqliteStore {
        SqliteStore { pool }
    }
}

impl IntoValue for &Param {
    fn into_value(self) -> libsql::Result<Value> {
        match self {
            Param::Null => Ok(Value::Null),
            Param::U8(p) => p.into_value(),
            Param::U16(p) => p.into_value(),
            Param::U32(p) => p.into_value(),
            Param::U64(p) => p.into_value(),
            Param::U128(p) => p.to_ne_bytes().into_value(),
            Param::USize(p) => p.to_ne_bytes().into_value(),
            Param::I8(p) => p.into_value(),
            Param::I16(p) => p.into_value(),
            Param::I32(p) => p.into_value(),
            Param::I64(p) => p.into_value(),
            Param::I128(p) => p.to_ne_bytes().into_value(),
            Param::ISize(p) => p.to_ne_bytes().into_value(),
            Param::F32(p) => p.into_value(),
            Param::F64(p) => p.into_value(),
            Param::String(p) => p.clone().into_value(),
            Param::Bytes(p) => p.clone().into_value(),
            Param::Uuid(p) => p.as_bytes().into_value(),
        }
    }
}

impl IntoParams for SqlQuery {
    fn to_params(&self) -> Result<params::Params> {
        match self.clone().params {
            Params::None => Ok(params::Params::None),
            Params::Positional(p) => {
                let mut values = Vec::with_capacity(p.len());
                for param in p.iter() {
                    values.push(param.into_value()?);
                }
                Ok(params::Params::Positional(values))
            }
            Params::Named(np) => {
                let mut named_values = Vec::with_capacity(np.len());
                for (k, v) in np.iter() {
                    named_values.push((k.clone(), v.into_value()?));
                }
                Ok(params::Params::Named(named_values))
            }
        }
    }
}

impl<T: for<'de> Deserialize<'de>> Query<'_, SqliteStore, T> for SqlQuery {
    async fn run(&self, store: &mut SqliteStore) -> Result<Vec<T>> {
        let conn = store.pool.get().await?;
        let mut rows = conn.query(self.query.as_str(), self.to_params()?).await?;

        let mut vals = Vec::new();
        while let Some(row) = rows.next().await? {
            vals.push(de::from_row::<T>(&row)?);
        }

        Ok(vals)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    use crate::sql;

    #[derive(serde::Deserialize, PartialEq, Eq, Debug)]
    struct Test {
        name: String,
        age: i32,
    }

    #[tokio::test]
    async fn should_get_record_by_id() {
        let uuid = Uuid::parse_str("f4f77f73-e1e8-4289-b77f-73e1e86289e0").expect("should have parsed uuid");
        let pool = prepare_db(vec![
            sql!(
                "
                CREATE TABLE test (
                    id   BLOB NOT NULL,
                    name TEXT NOT NULL,
                    age  INTEGER NOT NULL,
                    CONSTRAINT prefabs_pk PRIMARY KEY (id)
                );
                "
            ),
            sql!("INSERT INTO test (id, name, age) VALUES (?, 'John Doe', 42);", Param::Uuid(uuid)),
        ])
        .await;

        let mut store = SqliteStore::new(pool);
        let actual = sql!("SELECT name, age FROM test WHERE id = ?", uuid).run(&mut store).await.unwrap();

        assert_eq!(
            vec![Test {
                name: "John Doe".to_string(),
                age: 42
            }],
            actual
        );
    }

    async fn prepare_db(setup: Vec<SqlQuery>) -> Pool<LibSqlConnectionManager> {
        let db = libsql::Builder::new_local(":memory:")
            .build()
            .await
            .expect("should have in created memory db");

        let pool = Pool::builder()
            .test_on_check_out(false)
            .build(LibSqlConnectionManager(db))
            .await
            .expect("should have created pool");

        for sql in setup {
            pool.get()
                .await
                .expect("should have taken a connection from the pool")
                .execute(sql.query.as_str(), sql.to_params().unwrap())
                .await
                .expect("should have initialised database");
        }

        let mut debug = pool.get().await.unwrap().query("SELECT * FROM test", ()).await.unwrap();
        while let Some(v) = debug.next().await.unwrap() {
            let thing = de::from_row::<Test>(&v).unwrap();

            println!("Id: {thing:?}");
        }

        pool
    }
}
