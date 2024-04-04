use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions[
    r#"
    #[lua(kind="Function")]
    fn fn_returning_string_vec() -> Vec<String> {
        vec!["hello".to_owned()]
    }
    "#,

    r#"
    #[lua(kind="Function", output(proxy))]
    fn fn_returning_proxy_vec() -> Vec<Self> {
        vec![MyStruct, MyStruct]
    }
    "#,

    r#"
    #[lua(kind="Function", output(proxy))]
    fn fn_returning_proxy_vec_empty() -> Vec<Self> {
        Vec::default()
    }
    "#,
])]
pub struct MyStruct;

pub fn main() {}
