use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(ScriptProxy, Reflect, Clone)]
#[proxy(languages("lua"), derive(Clone))]
#[functions[
    #[lua(Function)]
    fn fn_returning_string_option(_opt: Option<String>) {
        
    }


    #[lua(Function)]
    fn fn_returning_some_proxy(#[proxy] _opt: Option<Self>) {
        
    }
]]
pub struct MyStruct;

pub fn main() {}
