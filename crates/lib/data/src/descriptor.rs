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
//! Provides a `Descriptor` comprised of label and description for a game entity.

use std::borrow::Cow;

use serde::Deserialize;
use uuid::Uuid;

/// An entity descriptor containing a UUID, label, and description.
///
/// # Example
///
/// ```rust
/// use darkforge_data::descriptor::Descriptor;
/// use darkforge_data::JSONDeserialize;
///
/// const JSON: &str = r#"
/// {
///     "id": "9f5c2c9e-4f4e-4fbf-8a7f-0a1ecf1a7c12",
///     "label": "Mighty",
///     "description": "You are powerful as a lion and resilient as an ox."
/// }
/// "#;
///
/// let desc = Descriptor::from_json(JSON.as_bytes()).expect("should have deserialized descriptor");
///
/// ```
#[derive(Deserialize, Debug, PartialEq)]
pub struct Descriptor<'a> {
    id: Uuid,
    label: Cow<'a, str>,
    description: Cow<'a, str>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::JSONDeserialize;

    #[test]
    fn test_deserialize_from_json() {
        const JSON: &str = r#"
            {
                "id": "9f5c2c9e-4f4e-4fbf-8a7f-0a1ecf1a7c12",
                "label": "Cunning",
                "description": "You are quick-witted and resourceful, often thinking on your feet."
            }
        "#;

        let actual = Descriptor::from_json(JSON.as_bytes()).expect("should have deserialized descriptor");

        assert_eq!(
            Descriptor {
                id: Uuid::parse_str("9f5c2c9e-4f4e-4fbf-8a7f-0a1ecf1a7c12").expect("should have parsed uuid"),
                label: "Cunning".into(),
                description: "You are quick-witted and resourceful, often thinking on your feet.".into(),
            },
            actual
        );
    }
}
