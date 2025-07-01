use super::*;

/// Stores the script context by entity.
pub struct ScriptIdContext<P: IntoScriptPluginParams>(HashMap<ScriptId, Arc<Mutex<P::C>>>);

impl<P: IntoScriptPluginParams> Default for ScriptIdContext<P> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for ScriptIdContext<P> {
    fn hash(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> Option<u64> {
        let mut hasher = DefaultHashBuilder::default().build_hasher();
        script_id.hash(&mut hasher);
        Some(hasher.finish())
    }
    fn get(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> Option<&Arc<Mutex<P::C>>> {
        self.0.get(script_id)
    }
    fn insert(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>, context: P::C) -> Result<(), P::C> {
        self.0.insert(script_id.clone(), Arc::new(Mutex::new(context)));
        Ok(())
    }
    fn contains(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> bool {
        self.0.contains_key(&script_id)
    }
}
