use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(ScriptProxy, Reflect, Clone)]
#[proxy(languages("lua"), derive(Clone))]
#[functions[
    #[lua(Function)]
    fn fn_returning_string_option() -> Option<String> {
        Some("hello".to_owned())
    }

    #[lua(Function)]
    fn fn_returning_string_option_none() -> Option<String> {
        None
    }

    #[lua(Function, output(proxy))]
    fn fn_returning_some_proxy() -> Option<Self> {
        Some(MyStruct)
    }

    #[lua(Function, output(proxy))]
    fn fn_returning_none_proxy() -> Option<Self> {
        None
    }
]]
pub struct MyStruct;

pub fn main() {}
