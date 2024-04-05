use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions[
    r#"
    #[lua(kind="Function")]
    fn my_fn() -> usize;
    "#,
])]
pub struct MyStruct;
impl MyStruct {
    pub fn my_fn() -> Self {
        MyStruct
    }
}
pub fn main() {}
