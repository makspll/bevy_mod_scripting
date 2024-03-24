use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(ScriptProxy, Reflect)]
#[proxy(languages("lua"), derive(Clone))]
#[functions[
    #[lua(MutatingMethod)]
    fn fn_returning_some_string(&mut self) -> String;

    #[lua(MutatingMethod, output(proxy))]
    fn fn_returning_proxy(&mut self) -> Self;
]]
#[derive(Clone)]
pub struct MyStruct {
    some_string: String,
    me_vec: Vec<usize>,
}

impl MyStruct {
    pub fn fn_returning_some_string(&mut self) -> String {
        self.some_string.clone()
    }

    pub fn fn_returning_proxy(&mut self) -> Self {
        self.clone()
    }
}

pub fn main() {}
