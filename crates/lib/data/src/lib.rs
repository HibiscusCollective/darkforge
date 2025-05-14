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

extern crate core;

pub mod store;
mod uuid;

use std::sync::{Arc, RwLock};

use thiserror::Error;

use crate::uuid::Uuid;

pub type Result<T> = std::result::Result<T, DataError>;
pub type ComponentPtr<'a> = Arc<RwLock<dyn Component + 'a>>;
pub type Entity = Uuid;

pub trait Component {}

#[derive(Error, Debug)]
pub enum DataError {}

pub fn component_ptr<'a>(component: impl Component + 'a) -> ComponentPtr<'a> {
    Arc::new(RwLock::new(component))
}
