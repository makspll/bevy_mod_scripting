use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions = [
    r#"
    #[lua(kind="Function", output(proxy))]
    fn fn_returning_proxy() -> Self {
        2
    }
    "#,
])]
pub struct MyStruct;
pub fn main() {}
