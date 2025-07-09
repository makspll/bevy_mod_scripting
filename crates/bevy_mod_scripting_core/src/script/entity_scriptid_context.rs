use super::*;

/// Stores the script context by entity and script ID.
pub struct EntityScriptIdContext<P: IntoScriptPluginParams>(HashMap<(Entity, ScriptId), Arc<Mutex<P::C>>>)

impl<P: IntoScriptPluginParams> Default for EntityScriptIdContext<P> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for EntityScriptIdContext<P> {
    fn hash(&self, id: Option<Entity>, script_id: &ScriptId, _domain: &Option<Domain>) -> Option<u64> {
        id.map(|id| DefaultHashBuilder::default().hash_one((id, script_id)))
    }
    fn get(&self, id: Option<Entity>, script_id: &ScriptId, _domain: &Option<Domain>) -> Option<&Arc<Mutex<P::C>>> {
        id.and_then(|id| self.0.get(&(id, *script_id))).or_else(|| {
            // TODO: This is not performant.
            // Look up using only the script_id.
            for ((_, sid), context) in &self.0.iter() {
                if sid == script_id

            }
        })
    }
    fn insert(&mut self, id: Option<Entity>, script_id: &ScriptId, _domain: &Option<Domain>, context: P::C) -> Result<(), P::C> {
        if let Some(id) = id {
            self.0.insert((id, *script_id), Arc::new(Mutex::new(context)));
            Ok(())
        } else {
            Err(context)
        }
    }
    fn contains(&self, id: Option<Entity>, script_id: &ScriptId, _domain: &Option<Domain>) -> bool {
        id.map(|id| self.0.contains_key(&(id, *script_id))).unwrap_or(false)
    }
    fn iter(&self) -> impl Iterator<Item = &Arc<Mutex<P::C>>> {
        self.0.values()
    }
    fn remove(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> bool {
        id.map(|id| self.0.remove(&(id, *script_id)).is_some()).unwrap_or(false)
    }
}
