use super::*;

/// Stores the script context by entity and script ID.
pub struct EntityScriptIdContext<P: IntoScriptPluginParams>(HashMap<(Entity, ScriptId), Arc<Mutex<P::C>>>);

impl<P: IntoScriptPluginParams> Default for EntityScriptIdContext<P> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for EntityScriptIdContext<P> {
    fn hash(&self, context_key: &ContextKey) -> Option<u64> {
        context_key.entity.zip(context_key.script_id).map(|(id, script_id)| DefaultHashBuilder::default().hash_one((id, script_id)))
    }
    fn get(&self, context_key: &ContextKey) -> Option<&Arc<Mutex<P::C>>> {
        context_key.entity.zip(context_key.script_id).and_then(|key| self.0.get(key))
        //   .or_else(|| {
        //     // TODO: This is not performant.
        //     // Look up using only the script_id.
        //     for ((_, sid), context) in &self.0.iter() {
        //         if sid == script_id {
        //             return Some(context)

        //     }
        // })
    }
    fn insert(&mut self, context_key: ContextKey, context: P::C) -> Result<(), P::C> {
        if let Some(key) = context_key.entity.zip(context_key.script_id){
            self.0.insert(key, Arc::new(Mutex::new(context)));
            Ok(())
        } else {
            Err(context)
        }
    }
    fn contains(&self, context_key: &ContextKey) -> bool {
        context_key.entity.zip(context_key.script_id).map(|key| self.0.contains_key(key)).unwrap_or(false)
    }
    fn values(&self) -> impl Iterator<Item = &Arc<Mutex<P::C>>> {
        self.0.values()
    }
    fn remove(&mut self, context_key: &ContextKey) -> bool {
        context_key.entity.zip(context_key.script_id).map(|key| self.0.remove(key).is_some()).unwrap_or(false)
    }
    fn iter(&self) -> impl Iterator<Item = (ContextKey, &Arc<Mutex<P::C>>)> {
        self.0.iter().map(|((id, script_id), c)| (ContextKey {
            entity: Some(*id),
            script_id: Some(*script_id),
            domain: None,
        }, c))
    }
}
