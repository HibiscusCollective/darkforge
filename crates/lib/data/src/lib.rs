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
#![deny(missing_docs)]

//! Data module for the Dark Forge library.
//!
//! This module provides data storage for games, with flexible structures to create different settings and support multiple languages.
//! By default, the data is stored in two tiers:
//!     - The first tier is read-only static data, usually loaded on startup and during transition. This store is read-optimised.
//!     - The second tier is dynamic static data, typically game state that we may want to serialize into a save file. Mostly held in memory, but disk backed if extra storage is required.

/// Module for data storage.
pub mod store;
mod uuid;

use std::sync::{Arc, RwLock};

use thiserror::Error;

use crate::uuid::Uuid;

/// Result type for operations in the data crate.
pub type Result<T> = std::result::Result<T, DataError>;

/// Pointer to a component, wrapped in an `Arc` and `RwLock`.
pub type ComponentPtr<'a> = Arc<RwLock<dyn Component + 'a>>;

/// Entity identifier type, implemented as a UUID.
pub type Entity = Uuid;

/// Trait for data components.
pub trait Component {}

/// Error type for operations in the data crate.
#[derive(Error, Debug)]
pub enum DataError {}

/// Creates a new component pointer from a component.
pub fn component_ptr<'a>(component: impl Component + 'a) -> ComponentPtr<'a> {
    Arc::new(RwLock::new(component))
}
