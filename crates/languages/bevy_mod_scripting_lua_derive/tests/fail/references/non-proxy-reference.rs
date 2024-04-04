use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions = [
    r#"
    #[lua(kind="Function")]
    fn fn_returning_some_string(some_str: &str) -> String
    "#,
])]
pub struct MyStruct {
    some_string: String,
    me_vec: Vec<usize>,
}
impl MyStruct {
    pub fn fn_returning_some_string(some_str: &str) -> String {
        some_str.to_owned()
    }
}

pub fn main() {}
