use godot::prelude::*;

struct HungryGoblins;

mod character;

#[gdextension]
unsafe impl ExtensionLibrary for HungryGoblins {}
