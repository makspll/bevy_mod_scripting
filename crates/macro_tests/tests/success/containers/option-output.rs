use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions[
    r#"
    #[lua(kind="Function")]
    fn fn_returning_string_option() -> Option<String> {
        Some("hello".to_owned())
    }
    "#,

    r#"
    #[lua(kind="Function")]
    fn fn_returning_string_option_none() -> Option<String> {
        None
    }
    "#,

    r#"
    #[lua(kind="Function", output(proxy))]
    fn fn_returning_some_proxy() -> Option<Self> {
        Some(MyStruct)
    }
    "#,

    r#"
    #[lua(kind="Function", output(proxy))]
    fn fn_returning_none_proxy() -> Option<Self> {
        None
    }
    "#,
])]
pub struct MyStruct;

pub fn main() {}
