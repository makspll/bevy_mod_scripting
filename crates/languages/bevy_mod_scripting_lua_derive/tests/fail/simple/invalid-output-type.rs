use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(ScriptProxy, Reflect)]
#[proxy(languages("lua"), derive(Clone))]
#[functions[
    #[lua(Function)]
    fn my_fn() -> usize;
]]
#[derive(Clone)]
pub struct MyStruct;

impl MyStruct {
    pub fn my_fn() -> Self {
        MyStruct
    }
}
pub fn main() {}
