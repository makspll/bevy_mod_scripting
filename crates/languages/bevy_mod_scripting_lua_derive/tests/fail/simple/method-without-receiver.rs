use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(ScriptProxy, Reflect)]
#[proxy(languages("lua"), derive(Clone))]
#[functions[
    #[lua(Method)]
    fn fn_taking_nothing() {

    }

    #[lua(Method)]
    fn fn_taking_usize(arg: usize) {

    }
]]
#[derive(Clone)]
pub struct MyStruct;

pub fn main() {}
