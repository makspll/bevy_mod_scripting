use bevy::prelude::*;
use bevy_mod_scripting::api::*;
#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions = [
    r#"
    #[lua(kind="MutableMethod")]
    fn fn_returning_some_string(&mut self) -> String;
    "#,

    r#"
    #[lua(kind="MutableMethod", output(proxy))]
    fn fn_returning_proxy(&mut self) -> Self;
    "#,
])]
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
