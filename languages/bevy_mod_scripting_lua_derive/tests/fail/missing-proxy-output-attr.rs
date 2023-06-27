use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(ScriptProxy, Reflect)]
#[proxy(languages("lua"), derive(Clone))]
#[functions[
    #[lua(Function)]
    fn fn_returning_proxy() -> Self {
        MyStruct
    }
]]
#[derive(Clone)]
pub struct MyStruct;

pub fn main() {}
