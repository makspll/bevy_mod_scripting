use super::*;

/// Contains the shared context.
pub struct SharedContext<P: IntoScriptPluginParams>(pub Option<Arc<Mutex<P::C>>>);

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for SharedContext<P> {
    fn hash(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> Option<u64> {
        self.0.is_some().then_some(0)
    }

    fn get(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> Option<&Arc<Mutex<P::C>>> {
        self.0.as_ref()
    }
    fn insert(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>, context: P::C) -> Result<(), P::C> {
        self.0 = Some(Arc::new(Mutex::new(context)));
        Ok(())
    }
    fn contains(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> bool {
        self.0.is_some()
    }
}

impl<P: IntoScriptPluginParams> Default for SharedContext<P> {
    fn default() -> Self {
        Self(None)
    }
}
