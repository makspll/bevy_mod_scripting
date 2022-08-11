use crate::error::ScriptError;


/// A documentation piece exported by an `APIProvider`
pub trait DocFragment: 'static {
    fn merge(self, o: Self) -> Self;
    fn gen_docs(self) -> Result<(), ScriptError>;
}
