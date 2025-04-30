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
mod sql;

use std::{error, fmt::Debug, future::Future, path::PathBuf};

use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct StoreError(#[from] Box<dyn error::Error>);

pub trait Value: Sized {
    type Error: error::Error + Debug;
    type Result<T>: Into<Result<T, Self::Error>>;
    type ValueType;

    fn convert_to<T: TryFrom<Self::ValueType>>(&self) -> Self::Result<Option<T>>;
}

pub trait Query<'a, S: Store, T: Deserialize<'a> = ()> {
    fn run(&self, store: &mut S) -> impl Future<Output = S::Result<Vec<T>>>;
}

pub trait Key {}

pub trait Store {
    type Error: error::Error + Debug;
    type Result<T>: Into<Result<T, Self::Error>>;
}

pub trait Migrator {
    type Error: error::Error + Debug;
    type Result<T>: Into<Result<T, Self::Error>>;

    fn apply(&self, path: impl Into<PathBuf>) -> impl Future<Output = Self::Result<()>>;
}
