use super::*;

/// Stores the script context by entity and script ID.
pub struct EntityScriptIdContext<P: IntoScriptPluginParams>(
    HashMap<(Entity, ScriptId), Arc<Mutex<P::C>>>,
);

impl<P: IntoScriptPluginParams> Default for EntityScriptIdContext<P> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for EntityScriptIdContext<P> {
    fn hash(&self, context_key: &ContextKey) -> Option<u64> {
        context_key
            .entity
            .zip(context_key.script.as_ref())
            .map(|(id, script)| DefaultHashBuilder::default().hash_one((id, script.id())))
    }
    fn get(&self, context_key: &ContextKey) -> Option<&Arc<Mutex<P::C>>> {
        context_key
            .entity
            .zip(context_key.script.as_ref())
            .and_then(|(e, h)| self.0.get(&(e, h.id())))
    }
    fn insert(&mut self, context_key: ContextKey, context: P::C) -> Result<(), P::C> {
        if let Some((e, h)) = context_key.entity.zip(context_key.script.as_ref()) {
            self.0.insert((e, h.id()), Arc::new(Mutex::new(context)));
            Ok(())
        } else {
            Err(context)
        }
    }
    fn contains(&self, context_key: &ContextKey) -> bool {
        context_key
            .entity
            .zip(context_key.script.as_ref())
            .map(|(e, h)| self.0.contains_key(&(e, h.id())))
            .unwrap_or(false)
    }
    fn values(&self) -> impl Iterator<Item = &Arc<Mutex<P::C>>> {
        self.0.values()
    }
    /// Remove the (entity, script_id) pair. Or if an entity is given alone, remove all entity pairs.
    fn remove(&mut self, context_key: &ContextKey) -> bool {
        context_key
            .entity
            .zip(context_key.script.as_ref())
            .map(|(e, h)| self.0.remove(&(e, h.id())).is_some())
            .unwrap_or(false)
            || (context_key.script.is_none()
                && context_key
                    .entity
                    .map(|id| {
                        let keys: Vec<(Entity, ScriptId)> = self
                            .0
                            .keys()
                            .filter(|(entity, _)| *entity == id)
                            .cloned()
                            .collect();
                        let mut removed = false;
                        for key in keys {
                            removed |= self.0.remove(&key).is_some();
                        }
                        removed
                    })
                    .unwrap_or(false))
    }
    fn iter(&self) -> impl Iterator<Item = (ContextKey, &Arc<Mutex<P::C>>)> {
        self.0.iter().map(|((id, script_id), c)| {
            (
                ContextKey {
                    entity: Some(*id),
                    script: Some(Handle::Weak(*script_id)),
                    domain: None,
                },
                c,
            )
        })
    }
}
