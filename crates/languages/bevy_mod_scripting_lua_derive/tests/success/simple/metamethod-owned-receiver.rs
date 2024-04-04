use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions[
    r#"
    #[lua(kind="MetaMethod", metamethod="ToString")]
    fn ToString(&self) -> String {
        self.some_string.clone()
    }
    "#,

    r#"
    #[lua(kind="MetaMethod", metamethod="Index", output(proxy))]
    fn Index(&self, _idx: usize) -> Self {
        self.clone()
    }
    "#,
])]
pub struct MyStruct {
    some_string: String,
    me_vec: Vec<usize>,
}

pub fn main() {}
