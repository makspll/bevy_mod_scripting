use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions[
    r#"
    #[lua(kind="Method")]
    fn fn_taking_nothing() {

    }
    "#,

    r#"
    #[lua(kind="Method")]
    fn fn_taking_usize(arg: usize) {

    }
    "#,
])]
pub struct MyStruct;

pub fn main() {}
