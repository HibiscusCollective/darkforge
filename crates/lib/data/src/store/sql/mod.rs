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

use thiserror::Error;
use uuid::Uuid;

pub mod sqlite;

#[macro_export]
/// Creates a new SQL query from a string literal.
macro_rules! sql {
    ($sql:expr) => {
        SqlQuery::new($sql, $crate::store::sql::Params::None)
    };
    ($sql:expr, { $($key:expr => $param:expr),+ $(,)? }) => {
        SqlQuery::new($sql, $crate::store::sql::Params::Named(vec![$(($key.into(), $param.into())),+]))
    };
    ($sql:expr, $($param:expr),*) => {
        SqlQuery::new($sql, $crate::store::sql::Params::Positional(vec![$($param.into()),*]))
    };
}

macro_rules! implement_into_param {
    ($param:ident => $v:ty) => {
        impl From<$v> for crate::store::sql::Param {
            fn from(value: $v) -> Param {
                crate::store::sql::Param::$param(value)
            }
        }
    };
}

/// Error type for SQL operations in the data store.
#[derive(Error, Debug)]
#[error(transparent)]
pub struct SqlError(#[from] anyhow::Error);

/// Represents a parameter for SQL queries.
#[derive(Debug, Clone, PartialEq)]
pub enum Param {
    /// Null value.
    Null,
    /// Unsigned 8-bit integer.
    U8(u8),
    /// Unsigned 16-bit integer.
    U16(u16),
    /// Unsigned 32-bit integer.
    U32(u32),
    /// Unsigned 64-bit integer.
    U64(u64),
    /// Unsigned 128-bit integer.
    U128(u128),
    /// Unsigned size integer.
    USize(usize),
    /// Signed 8-bit integer.
    I8(i8),
    /// Signed 16-bit integer.
    I16(i16),
    /// Signed 32-bit integer.
    I32(i32),
    /// Signed 64-bit integer.
    I64(i64),
    /// Signed 128-bit integer.
    I128(i128),
    /// Signed size integer.
    ISize(isize),
    /// 32-bit floating point.
    F32(f32),
    /// 64-bit floating point.
    F64(f64),
    /// String value.
    String(String),
    /// Byte array.
    Bytes(Vec<u8>),
    /// UUID value.
    Uuid(Uuid),
}

/// Represents the parameters for a SQL query.
#[derive(Clone, Debug, PartialEq)]
pub enum Params {
    /// No parameters.
    None,
    /// Positional parameters.
    Positional(Vec<Param>),
    /// Named parameters.
    Named(Vec<(String, Param)>),
}

/// Represents a SQL query and its parameters.
#[derive(Clone, Debug, PartialEq)]
pub struct SqlQuery {
    /// The SQL query string.
    pub query: String,
    /// The parameters for the query.
    pub params: Params,
}

impl SqlQuery {
    /// Creates a new `SqlQuery` from a query string and parameters.
    pub fn new(query: impl Into<String>, params: Params) -> Self {
        Self { query: query.into(), params }
    }
}

implement_into_param!(U8 => u8);
implement_into_param!(U16 => u16);
implement_into_param!(U32 => u32);
implement_into_param!(U64 => u64);
implement_into_param!(U128 => u128);
implement_into_param!(USize => usize);
implement_into_param!(I8 => i8);
implement_into_param!(I16 => i16);
implement_into_param!(I32 => i32);
implement_into_param!(I64 => i64);
implement_into_param!(I128 => i128);
implement_into_param!(ISize => isize);
implement_into_param!(F32 => f32);
implement_into_param!(F64 => f64);
implement_into_param!(String => String);
implement_into_param!(Uuid => Uuid);
implement_into_param!(Bytes => Vec<u8>);

impl From<&str> for Param {
    fn from(value: &str) -> Param {
        Param::String(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use uuid::{Uuid, uuid};

    use super::*;

    const ID: Uuid = uuid!("F9168C5E-FEB2-4FAA-B6BF-329BF39FA1E4");

    #[rstest]
    #[case::empty_params(sql!("SELECT * FROM test"), SqlQuery{query: "SELECT * FROM test".into(), params: Params::None})]
    #[case::u8_param(sql!("SELECT * FROM test WHERE id = ?", 0u8), SqlQuery{query: "SELECT * FROM test WHERE id = ?".into(), params: Params::Positional(vec![Param::U8(0)])})]
    #[case::u16_param(sql!("SELECT * FROM test WHERE id = ?", 1u16), SqlQuery{query: "SELECT * FROM test WHERE id = ?".into(), params: Params::Positional(vec![Param::U16(1)])})]
    #[case::u32_param(sql!("SELECT * FROM test WHERE id = ?", 2u32), SqlQuery{query: "SELECT * FROM test WHERE id = ?".into(), params: Params::Positional(vec![Param::U32(2)])})]
    #[case::u64_param(sql!("SELECT * FROM test WHERE id = ?", 3u64), SqlQuery{query: "SELECT * FROM test WHERE id = ?".into(), params: Params::Positional(vec![Param::U64(3)])})]
    #[case::u128_param(sql!("SELECT * FROM test WHERE id = ?", 4u128), SqlQuery{query: "SELECT * FROM test WHERE id = ?".into(), params: Params::Positional(vec![Param::U128(4)])})]
    #[case::usize_param(sql!("SELECT * FROM test WHERE id = ?", 5usize), SqlQuery{query: "SELECT * FROM test WHERE id = ?".into(), params: Params::Positional(vec![Param::USize(5)])})]
    #[case::i8_param(sql!("SELECT * FROM test WHERE id = ?", -6i8), SqlQuery{query: "SELECT * FROM test WHERE id = ?".into(), params: Params::Positional(vec![Param::I8(-6)])})]
    #[case::i16_param(sql!("SELECT * FROM test WHERE id = ?", -7i16), SqlQuery{query: "SELECT * FROM test WHERE id = ?".into(), params: Params::Positional(vec![Param::I16(-7)])})]
    #[case::i32_param(sql!("SELECT * FROM test WHERE id = ?", -8i32), SqlQuery{query: "SELECT * FROM test WHERE id = ?".into(), params: Params::Positional(vec![Param::I32(-8)])})]
    #[case::i64_param(sql!("SELECT * FROM test WHERE id = ?", -9i64), SqlQuery{query: "SELECT * FROM test WHERE id = ?".into(), params: Params::Positional(vec![Param::I64(-9)])})]
    #[case::i128_param(sql!("SELECT * FROM test WHERE id = ?", -10i128), SqlQuery{query: "SELECT * FROM test WHERE id = ?".into(), params: Params::Positional(vec![Param::I128(-10)])})]
    #[case::isize_param(sql!("SELECT * FROM test WHERE id = ?", -11isize), SqlQuery{query: "SELECT * FROM test WHERE id = ?".into(), params: Params::Positional(vec![Param::ISize(-11)])})]
    #[case::f32_param(sql!("SELECT * FROM test WHERE val = ?", 0.42f32), SqlQuery{query: "SELECT * FROM test WHERE val = ?".into(), params: Params::Positional(vec![Param::F32(0.42)])})]
    #[case::f64_param(sql!("SELECT * FROM test WHERE val = ?", 0.42f64), SqlQuery{query: "SELECT * FROM test WHERE val = ?".into(), params: Params::Positional(vec![Param::F64(0.42)])})]
    #[case::string_param(sql!("SELECT * FROM test WHERE name = ?", "John Doe".to_string()), SqlQuery{query: "SELECT * FROM test WHERE name = ?".into(), params: Params::Positional(vec![Param::String("John Doe".into())])})]
    #[case::str_param(sql!("SELECT * FROM test WHERE name = ?", "John Doe"), SqlQuery{query: "SELECT * FROM test WHERE name = ?".into(), params: Params::Positional(vec![Param::String("John Doe".into())])})]
    #[case::uuid_param(sql!("SELECT * FROM test WHERE uuid = ?", ID), SqlQuery{query: "SELECT * FROM test WHERE uuid = ?".into(), params: Params::Positional(vec![Param::Uuid(ID)])})]
    fn should_create_query_with_indexed_params(#[case] query: SqlQuery, #[case] expect: SqlQuery) {
        assert_eq!(expect, query);
    }

    #[rstest]
    #[case::empty_params(sql!("SELECT * FROM test"), SqlQuery{query: "SELECT * FROM test".into(), params: Params::None})]
    #[case::u8_param(sql!("SELECT * FROM test WHERE id = :key1", {"key1" => 0u8}), SqlQuery{query: "SELECT * FROM test WHERE id = :key1".into(), params: Params::Named(vec![("key1".into(), Param::U8(0))])})]
    #[case::u16_param(sql!("SELECT * FROM test WHERE id = :key1", {"key1" => 1u16}), SqlQuery{query: "SELECT * FROM test WHERE id = :key1".into(), params: Params::Named(vec![("key1".into(), Param::U16(1))])})]
    #[case::u32_param(sql!("SELECT * FROM test WHERE id = :key1", {"key1" => 2u32}), SqlQuery{query: "SELECT * FROM test WHERE id = :key1".into(), params: Params::Named(vec![("key1".into(), Param::U32(2))])})]
    #[case::u64_param(sql!("SELECT * FROM test WHERE id = :key1", {"key1" => 3u64}), SqlQuery{query: "SELECT * FROM test WHERE id = :key1".into(), params: Params::Named(vec![("key1".into(), Param::U64(3))])})]
    #[case::u128_param(sql!("SELECT * FROM test WHERE id = :key1", {"key1" => 4u128}), SqlQuery{query: "SELECT * FROM test WHERE id = :key1".into(), params: Params::Named(vec![("key1".into(), Param::U128(4))])})]
    #[case::usize_param(sql!("SELECT * FROM test WHERE id = :key1", {"key1" => 5usize}), SqlQuery{query: "SELECT * FROM test WHERE id = :key1".into(), params: Params::Named(vec![("key1".into(), Param::USize(5))])})]
    #[case::i8_param(sql!("SELECT * FROM test WHERE id = :key2", {"key2" => -6i8}), SqlQuery{query: "SELECT * FROM test WHERE id = :key2".into(), params: Params::Named(vec![("key2".into(), Param::I8(-6))])})]
    #[case::i16_param(sql!("SELECT * FROM test WHERE id = :key2", {"key2" => -7i16}), SqlQuery{query: "SELECT * FROM test WHERE id = :key2".into(), params: Params::Named(vec![("key2".into(), Param::I16(-7))])})]
    #[case::i32_param(sql!("SELECT * FROM test WHERE id = :key2", {"key2" => -8i32}), SqlQuery{query: "SELECT * FROM test WHERE id = :key2".into(), params: Params::Named(vec![("key2".into(), Param::I32(-8))])})]
    #[case::i64_param(sql!("SELECT * FROM test WHERE id = :key2", {"key2" => -9i64}), SqlQuery{query: "SELECT * FROM test WHERE id = :key2".into(), params: Params::Named(vec![("key2".into(), Param::I64(-9))])})]
    #[case::i128_param(sql!("SELECT * FROM test WHERE id = :key2", {"key2" => -10i128}), SqlQuery{query: "SELECT * FROM test WHERE id = :key2".into(), params: Params::Named(vec![("key2".into(), Param::I128(-10))])})]
    #[case::isize_param(sql!("SELECT * FROM test WHERE id = :key2", {"key2" => -11isize}), SqlQuery{query: "SELECT * FROM test WHERE id = :key2".into(), params: Params::Named(vec![("key2".into(), Param::ISize(-11))])})]
    #[case::f32_param(sql!("SELECT * FROM test WHERE val = :key3", {"key3" => 0.42f32}), SqlQuery{query: "SELECT * FROM test WHERE val = :key3".into(), params: Params::Named(vec![("key3".into(), Param::F32(0.42))])})]
    #[case::f64_param(sql!("SELECT * FROM test WHERE val = :key3", {"key3" => 0.42f64}), SqlQuery{query: "SELECT * FROM test WHERE val = :key3".into(), params: Params::Named(vec![("key3".into(), Param::F64(0.42))])})]
    #[case::string_param(sql!("SELECT * FROM test WHERE name = :key3", {"key3" => "John Doe".to_string()}), SqlQuery{query: "SELECT * FROM test WHERE name = :key3".into(), params: Params::Named(vec![("key3".into(), Param::String("John Doe".into()))])})]
    #[case::str_param(sql!("SELECT * FROM test WHERE name = :key3", {"key3" => "John Doe"}), SqlQuery{query: "SELECT * FROM test WHERE name = :key3".into(), params: Params::Named(vec![("key3".into(), Param::String("John Doe".into()))])})]
    #[case::uuid_param(sql!("SELECT * FROM test WHERE uuid = :key3", {"key3" => ID}), SqlQuery{query: "SELECT * FROM test WHERE uuid = :key3".into(), params: Params::Named(vec![("key3".into(), Param::Uuid(ID))])})]
    fn should_create_query_with_named_params(#[case] query: SqlQuery, #[case] expect: SqlQuery) {
        assert_eq!(expect, query);
    }
}
