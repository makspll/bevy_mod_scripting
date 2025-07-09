use super::*;

/// Contains the shared context.
pub struct SharedContext<P: IntoScriptPluginParams>(pub Option<Arc<Mutex<P::C>>>);

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for SharedContext<P> {
    fn hash(&self, _id: Option<Entity>, _script_id: &ScriptId, _domain: &Option<Domain>) -> Option<u64> {
        self.0.is_some().then_some(0)
    }

    fn get(&self, _id: Option<Entity>, _script_id: &ScriptId, _domain: &Option<Domain>) -> Option<&Arc<Mutex<P::C>>> {
        self.0.as_ref()
    }
    fn insert(&mut self, _id: Option<Entity>, _script_id: &ScriptId, _domain: &Option<Domain>, context: P::C) -> Result<(), P::C> {
        self.0 = Some(Arc::new(Mutex::new(context)));
        Ok(())
    }
    fn contains(&self, _id: Option<Entity>, _script_id: &ScriptId, _domain: &Option<Domain>) -> bool {
        self.0.is_some()
    }

    fn iter(&self) -> impl Iterator<Item = &Arc<Mutex<P::C>>> {
        self.0.as_ref().into_iter()
    }

    fn remove(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> bool {
        false
        // self.0 = None;
        // true
    }
}

impl<P: IntoScriptPluginParams> Default for SharedContext<P> {
    fn default() -> Self {
        Self(None)
    }
}
