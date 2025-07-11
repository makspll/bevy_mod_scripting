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
        context_key.entity.zip(context_key.script_id.as_ref()).map(|(id, script_id)| DefaultHashBuilder::default().hash_one((id, script_id.id())))
    }
    fn get(&self, context_key: &ContextKey) -> Option<&Arc<Mutex<P::C>>> {
        context_key.entity.zip(context_key.script_id.as_ref()).and_then(|(e, h)| self.0.get(&(e, h.id())))
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
        if let Some((e, h)) = context_key.entity.zip(context_key.script_id.as_ref()){
            self.0.insert((e, h.id()), Arc::new(Mutex::new(context)));
            Ok(())
        } else {
            Err(context)
        }
    }
    fn contains(&self, context_key: &ContextKey) -> bool {
        context_key.entity.zip(context_key.script_id.as_ref()).map(|(e, h)| self.0.contains_key(&(e, h.id()))).unwrap_or(false)
    }
    fn values(&self) -> impl Iterator<Item = &Arc<Mutex<P::C>>> {
        self.0.values()
    }
    fn remove(&mut self, context_key: &ContextKey) -> bool {
        context_key.entity.zip(context_key.script_id.as_ref()).map(|(e, h)| self.0.remove(&(e, h.id())).is_some()).unwrap_or(false)
    }
    fn iter(&self) -> impl Iterator<Item = (ContextKey, &Arc<Mutex<P::C>>)> {
        self.0.iter().map(|((id, script_id), c)| (ContextKey {
            entity: Some(*id),
            script_id: Some(Handle::Weak(*script_id)),
            domain: None,
        }, c))
    }
}
