use super::*;

/// Stores the script context by entity.
pub struct EntityContext<P: IntoScriptPluginParams>(HashMap<Entity, Arc<Mutex<P::C>>>);

impl<P: IntoScriptPluginParams> Default for EntityContext<P> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for EntityContext<P> {
    fn get(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> Option<&Arc<Mutex<P::C>>> {
        id.and_then(|id| self.0.get(&id))
    }
    fn insert(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>, context: P::C) -> Result<(), P::C> {
        if let Some(id) = id {
            self.0.insert(id, Arc::new(Mutex::new(context)));
            Ok(())
        } else {
            Err(context)
        }
    }
    fn contains(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> bool {
        id.map(|id| self.0.contains_key(&id)).unwrap_or(false)
    }
}
