use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(ScriptProxy, Reflect, Clone)]
#[proxy(languages("lua"), derive(Clone))]
#[functions[
    #[lua(Function)]
    fn fn_returning_string_vec() -> Vec<String> {
        vec!["hello".to_owned()]
    }

    #[lua(Function, output(proxy))]
    fn fn_returning_proxy_vec() -> Vec<Self> {
        vec![MyStruct,MyStruct]
    }

    #[lua(Function, output(proxy))]
    fn fn_returning_proxy_vec_empty() -> Vec<Self> {
        Vec::default()
    }
]]
pub struct MyStruct;

pub fn main() {}
