use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(ScriptProxy, Reflect)]
#[proxy(languages("lua"), derive(Clone))]
#[functions[
    #[lua(Function, output(proxy))]
    fn fn_returning_some_string(#[proxy] ref_: &Self ) -> &Self {
        ref_
    }
]]
#[derive(Clone)]
pub struct MyStruct;

pub fn main() {}
