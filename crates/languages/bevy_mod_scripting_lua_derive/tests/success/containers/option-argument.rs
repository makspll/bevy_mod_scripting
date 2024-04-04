use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions[
r#"
    #[lua(kind="Function")]
    fn fn_returning_string_option(_opt: Option<String>)
"#,r#"
    #[lua(kind="Function")]
    fn fn_returning_some_proxy(#[proxy] _opt: Option<Self>)
"#
])]
pub struct MyStruct;

pub fn main() {}

#[lua(kind = "Function")]
fn fn_returning_string_option(_opt: Option<String>) {}

#[lua(kind = "Function")]
fn fn_returning_some_proxy(#[proxy] _opt: Option<Self>) {}

pub struct MyStruct;

pub fn main() {}
