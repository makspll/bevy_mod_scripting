use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions = [
    r#"
    #[lua(kind="Function")]
    fn fn_returning_some_string() -> String
    "#,

    r#"
    #[lua(kind="Function", output(proxy))]
    fn fn_returning_proxy() -> Self
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

    pub fn fn_returning_proxy() -> MyStruct {
        MyStruct {
            some_string: "hello".to_owned(),
            me_vec: vec![1, 2, 3],
        }
    }
}

pub fn main() {}
