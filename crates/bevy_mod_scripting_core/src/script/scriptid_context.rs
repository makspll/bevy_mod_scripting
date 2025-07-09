use super::*;

/// Stores the script context by entity.
pub struct ScriptIdContext<P: IntoScriptPluginParams>(HashMap<ScriptId, Arc<Mutex<P::C>>>);

impl<P: IntoScriptPluginParams> Default for ScriptIdContext<P> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for ScriptIdContext<P> {
    fn hash(&self, _id: Option<Entity>, script_id: &ScriptId, _domain: &Option<Domain>) -> Option<u64> {
        Some(DefaultHashBuilder::default().hash_one(script_id))
    }
    fn get(&self, _id: Option<Entity>, script_id: &ScriptId, _domain: &Option<Domain>) -> Option<&Arc<Mutex<P::C>>> {
        self.0.get(script_id)
    }
    fn insert(&mut self, _id: Option<Entity>, script_id: &ScriptId, _domain: &Option<Domain>, context: P::C) -> Result<(), P::C> {
        self.0.insert(*script_id, Arc::new(Mutex::new(context)));
        Ok(())
    }
    fn contains(&self, _id: Option<Entity>, script_id: &ScriptId, _domain: &Option<Domain>) -> bool {
        self.0.contains_key(script_id)
    }
    fn iter(&self) -> impl Iterator<Item = &Arc<Mutex<P::C>>> {
        self.0.values()
    }
    fn remove(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> bool {
        self.0.remove(script_id).is_some()
    }
}
