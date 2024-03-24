use bevy::prelude::*;
use bevy_mod_scripting::api::*;

#[derive(ScriptProxy, Reflect)]
#[proxy(languages("lua"), derive(Clone))]
#[functions[
    #[lua(MetaMethod)]
    fn ToString(self) -> String {
        self.some_string.clone()
    }

    #[lua(MetaMethod, output(proxy))]
    fn Index(self, _idx: usize) -> Self {
        self.clone()
    }

    #[lua(MetaMethod, output(proxy))]
    fn Custom(self)  {
        
    }
]]
#[derive(Clone)]
pub struct MyStruct {
    some_string: String,
    me_vec: Vec<usize>,
}

pub fn main() {}
