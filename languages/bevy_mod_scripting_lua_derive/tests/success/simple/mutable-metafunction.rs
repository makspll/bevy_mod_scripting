use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(ScriptProxy, Reflect)]
#[proxy(languages("lua"), derive(Clone))]
#[functions[
    #[lua(MutableMetaFunction)]
    fn ToString(#[proxy] my_struct:  &Self) -> String {
        my_struct.some_string.clone()
    }

    #[lua(MutableMetaFunction, output(proxy))]
    fn Index(#[proxy] my_struct: &Self, _i: usize) -> Self {
        my_struct.clone()
    }
]]
#[derive(Clone)]
pub struct MyStruct {
    some_string: String,
    me_vec: Vec<usize>,
}

pub fn main() {}
