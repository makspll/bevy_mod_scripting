use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions[
    r#"
    #[lua(kind="MetaFunction", metamethod="ToString")]
    fn ToString(#[proxy] my_struct:  &Self) -> String {
        my_struct.some_string.clone()
    }
    "#,

    r#"
    #[lua(kind="MetaFunction", output(proxy), metamethod="Index")]
    fn Index(#[proxy] my_struct: &Self, _i: usize) -> Self {
        my_struct.clone()
    }
    "#,
])]
pub struct MyStruct {
    some_string: String,
    me_vec: Vec<usize>,
}

pub fn main() {}
