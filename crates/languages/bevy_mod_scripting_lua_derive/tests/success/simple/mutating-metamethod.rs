use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions[
    r#"
    #[lua(kind="MutatingMetaMethod", metamethod="ToString")]
    fn ToString(&mut self) -> String {
        self.some_string = "lol".to_string();
        self.some_string.clone()
    }
    "#,

    r#"
    #[lua(kind="MutatingMetaMethod", metamethod="Index", output(proxy))]
    fn Index(&mut self, _idx: usize) -> Self {
        self.clone()
    }
    "#,
])]
pub struct MyStruct {
    some_string: String,
    me_vec: Vec<usize>,
}
pub fn main() {}
