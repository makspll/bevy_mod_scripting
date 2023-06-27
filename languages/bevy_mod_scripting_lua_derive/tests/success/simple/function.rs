use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(ScriptProxy, Reflect)]
#[proxy(languages("lua"), derive(Clone))]
#[functions[
    #[lua(Function)]
    fn fn_returning_some_string() -> String;

    #[lua(Function, output(proxy))]
    fn fn_returning_proxy() -> Self;
]]
#[derive(Clone)]
pub struct MyStruct {
    some_string: String,
    me_vec: Vec<usize>,
}

impl MyStruct {
    pub fn fn_returning_some_string() -> String {
        "hello".to_owned()
    }

    pub fn fn_returning_proxy() -> MyStruct {
        MyStruct {
            some_string: "hello".to_owned(),
            me_vec: vec![1, 2, 3],
        }
    }
}

pub fn main() {}
