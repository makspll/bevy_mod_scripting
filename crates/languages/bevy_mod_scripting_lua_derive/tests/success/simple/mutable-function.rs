use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions = [
    r#"
    #[lua(kind="MutableFunction")]
    fn fn_returning_some_string(&mut self) -> String {
        self.some_string.clone()
    }
    "#,

    r#"
    #[lua(kind="MutableFunction", output(proxy))]
    fn fn_returning_proxy(&mut self) -> Self {
        self.clone()
    }
    "#,
])]
pub struct MyStruct {
    some_string: String,
    me_vec: Vec<usize>,
}
impl MyStruct {
    pub fn fn_returning_some_string() -> String {
        "hello".to_owned()
    }

    pub fn fn_returning_proxy() -> Self {
        Self {
            some_string: "hello".to_owned(),
            me_vec: Default::default(),
        }
    }
}

pub fn main() {}
