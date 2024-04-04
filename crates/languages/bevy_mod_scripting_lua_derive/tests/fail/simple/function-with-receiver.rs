use bevy::prelude::*;
use bevy_mod_scripting::api::*;
#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions = [
    r#"
    #[lua(kind="Function")]
    fn fn_returning_some_string(self) {

    }
    "#,

    r#"
    #[lua(kind="Function")]
    fn fn_returning_proxy(&self) {
        
    }
    "#,
])]
pub struct MyStruct;

pub fn main() {}
