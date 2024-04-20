use bevy_mod_scripting_lua_derive::LuaProxy;
use tealr::mlu::mlua::{Lua, UserData};

#[derive(LuaProxy)]
#[proxy(bms_lua_path = "crate", bms_core_path = "bevy_mod_scripting_core")]
#[proxy(functions[r#"
fn hello2(asd: usize, asd2: usize) -> ();

"#])]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn hello() {}
    pub fn hello2(asd: usize, asd2: usize) {}
}
