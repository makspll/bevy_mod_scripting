use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(ScriptProxy, Reflect)]
#[proxy(languages("lua"), derive(Clone))]
#[functions[
    #[lua(AMASJDIASDKAW)]
    fn fn_taking_nothing() {

    }
]]
#[derive(Clone)]
pub struct MyStruct;

pub fn main() {}
