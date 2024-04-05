use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions[
    r#"
    #[lua(kind="AMASJDIASDKAW")]
    fn fn_taking_nothing() {

    }
    "#,
])]
pub struct MyStruct;

pub fn main() {}
