use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions[
    r#"
    #[lua(kind="Method")]
    fn my_fn(&self);
    "#,
])]
pub struct MyStruct;

impl MyStruct {
    pub fn my_fn(&self, _: usize) {}
}

pub fn main() {}
