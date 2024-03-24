use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(ScriptProxy, Reflect)]
#[proxy(languages("lua"), derive(Clone))]
#[functions[
    #[lua(MutatingMetaMethod)]
    fn ToString(&mut self) -> String {
        self.some_string = "lol".to_string();
        self.some_string.clone()
    }

    #[lua(MutatingMetaMethod, output(proxy))]
    fn Index(&mut self, _idx: usize) -> Self {
        self.clone()
    }
]]
#[derive(Clone)]
pub struct MyStruct {
    some_string: String,
    me_vec: Vec<usize>,
}

pub fn main() {}
