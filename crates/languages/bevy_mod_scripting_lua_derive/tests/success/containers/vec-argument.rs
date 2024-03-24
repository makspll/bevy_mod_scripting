use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(ScriptProxy, Reflect, Clone)]
#[proxy(languages("lua"), derive(Clone))]
#[functions[
    #[lua(Function)]
    fn fn_returning_string_vec(_vec: Vec<String>) {
    }

    #[lua(Function)]
    fn fn_returning_proxy_vec(#[proxy] _vec: Vec<Self>)  {
        
    }

]]
pub struct MyStruct;

pub fn main() {}
