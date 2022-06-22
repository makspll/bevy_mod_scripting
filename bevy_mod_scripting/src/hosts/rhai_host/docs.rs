use crate::{DocFragment, ScriptError};

pub struct RhaiDocFragment;

impl DocFragment for RhaiDocFragment {
    fn merge(self, _o: Self) -> Self {
        todo!()
    }

    fn gen_docs(self) -> Result<(), ScriptError> {
        todo!()
    }
}
