extern crate core;

pub mod store;

use std::sync::{Arc, RwLock};

use thiserror::Error;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, DataError>;
pub type ComponentPtr<'a> = Arc<RwLock<dyn Component + 'a>>;

pub trait Component {}

#[derive(Error, Debug)]
pub enum DataError {}

struct EntityBuilder<'a> {
    uuid: Box<dyn FnOnce() -> Uuid>,
    components: Vec<ComponentPtr<'a>>,
}

impl Default for EntityBuilder<'_> {
    fn default() -> Self {
        Self {
            uuid: Box::new(Uuid::new_v4),
            components: Vec::new(),
        }
    }
}

impl<'a> EntityBuilder<'a> {
    #[cfg(test)]
    fn with_uuid_provider(mut self, provider: Box<dyn Fn() -> Uuid>) -> Self {
        self.uuid = provider;
        self
    }

    fn with_component(mut self, component: ComponentPtr<'a>) -> Self {
        self.components.push(component);
        self
    }
}

pub fn component_ptr<'a>(component: impl Component + 'a) -> ComponentPtr<'a> {
    Arc::new(RwLock::new(component))
}

#[cfg(test)]
mod tests {
    use blake3::Hasher;
    use uuid::Uuid;

    use super::*;

    #[test]
    fn should_create_entity() {
        // let store = TestEntityStore;
        //
        // let entity = EntityBuilder::default()
        //     .with_uuid_provider(deterministic_uuid("test-entity-1"))
        //     .with_component(component_ptr(TestComponent))
        //     .save(store);

        // assert_eq!(entity, Entity(uuid::Uuid::new_v4()));
    }

    struct TestComponent;

    impl Component for TestComponent {}

    fn deterministic_uuid(value: impl Into<String>) -> Box<dyn Fn() -> Uuid> {
        let mut hash = [0u8; 16];
        Hasher::new().update(value.into().as_bytes()).finalize_xof().fill(&mut hash);

        Box::new(move || Uuid::from_bytes(hash))
    }
}
