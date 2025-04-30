use godot::{meta::PropertyHintInfo, prelude::*};

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct Character {
    base: Base<Resource>,
    name: GString,
}

#[godot_api]
impl IResource for Character {
    fn init(base: Base<Resource>) -> Self {
        Self {
            base,
            name: "Character".into(),
        }
    }
}
