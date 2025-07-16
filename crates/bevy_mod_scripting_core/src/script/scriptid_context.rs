use super::*;

/// Stores the script context by entity.
pub struct ScriptIdContext<P: IntoScriptPluginParams>(HashMap<ScriptId, Arc<Mutex<P::C>>>);

impl<P: IntoScriptPluginParams> Default for ScriptIdContext<P> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for ScriptIdContext<P> {
    fn hash(&self, context_key: &ContextKey) -> Option<u64> {
        context_key.script.as_ref().map(|id| DefaultHashBuilder::default().hash_one(id))
    }
    fn get(&self, context_key: &ContextKey) -> Option<&Arc<Mutex<P::C>>> {
        context_key.script.as_ref().and_then(|id| self.0.get(&id.id()))
    }
    fn insert(&mut self, context_key: ContextKey, context: P::C) -> Result<(), P::C> {
        if let Some(id) = context_key.script {
            self.0.insert(id.id(), Arc::new(Mutex::new(context)));
            Ok(())
        } else {
            Err(context)
        }
    }
    fn contains(&self, context_key: &ContextKey) -> bool {
        context_key.script.as_ref().map(|id| self.0.contains_key(&id.id())).unwrap_or(false)
    }
    fn values(&self) -> impl Iterator<Item = &Arc<Mutex<P::C>>> {
        self.0.values()
    }
    fn remove(&mut self, context_key: &ContextKey) -> bool {
        context_key.script.as_ref().map(|id| self.0.remove(&id.id()).is_some()).unwrap_or(false)
    }
    fn iter(&self) -> impl Iterator<Item = (ContextKey, &Arc<Mutex<P::C>>)> {
        self.0.iter().map(|(script_id, c)| ((*script_id).into(), c))
    }
}
