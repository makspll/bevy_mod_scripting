use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(ScriptProxy, Reflect)]
#[proxy(languages("lua"), derive(Clone))]
#[functions[
    #[lua(Method)]
    fn my_fn(&self);
]]
#[derive(Clone)]
pub struct MyStruct;

impl MyStruct {
    pub fn my_fn(&self, _: usize) {}
}

pub fn main() {}
