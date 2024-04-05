use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(derive(clone), functions[
    r#"
    #[lua(kind="MutatingMethod")]
    fn fn_returning_some_string(mut self) -> String {
        self.some_string.clone()
    }
    "#,

    r#"
    #[lua(kind="MutatingMethod", output(proxy))]
    fn fn_returning_proxy(mut self) -> Self {
        self.clone()
    }
    "#,
])]
pub struct MyStruct {
    some_string: String,
    me_vec: Vec<usize>,
}

impl MyStruct {
    pub fn fn_returning_some_string(self) -> String {
        self.some_string
    }

    pub fn fn_returning_proxy(self) -> Self {
        self.clone()
    }
}

pub fn main() {}
