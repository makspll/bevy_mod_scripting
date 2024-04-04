use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions[
    r#"
    #[lua(kind="Function", output(proxy))]
    fn fn_returning_some_string(ref_: &Self ) -> &Self {
        ref_
    }
    "#,
])]
pub struct MyStruct;

pub fn main() {}
