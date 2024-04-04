use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect)]
#[proxy(functions = [
    r#"
    #[lua(kind="Function")]
    fn fn_returning_some_string(#[proxy] other: &Self) -> String
    "#,
])]
pub struct MyStruct {
    some_string: String,
    me_vec: Vec<usize>,
}

impl MyStruct {
    pub fn fn_returning_some_string(other: &Self) -> String {
        other.some_string.clone()
    }
}

pub fn main() {}
