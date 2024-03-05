use bevy_mod_scripting_core::prelude::*;

pub struct RuneDocFragment;

impl DocFragment for RuneDocFragment {
    fn merge(self, _o: Self) -> Self {
        todo!()
    }

    fn gen_docs(self) -> Result<(), ScriptError> {
        todo!()
    }

    fn name(&self) -> &'static str {
        todo!()
    }
}
