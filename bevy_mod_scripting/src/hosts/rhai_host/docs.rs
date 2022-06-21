use crate::{DocFragment, ScriptError};



pub struct RhaiDocFragment;

impl DocFragment for RhaiDocFragment {
    fn merge(self, o : Self) -> Self {
        todo!()
    }

    fn gen_docs(self) -> Result<(),ScriptError>{
        todo!()
    }
}