use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(derive(clone), functions[
    r#"
    #[lua(kind="Function")]
    fn fn_returning_string_vec(_vec: Vec<String>) {
    }
    "#,

    r#"
    #[lua(kind="Function", output(proxy))]
    fn fn_returning_proxy_vec(_vec: Vec<Self>)  {
        
    }
    "#,
])]
pub struct MyStruct;

pub fn main() {}
