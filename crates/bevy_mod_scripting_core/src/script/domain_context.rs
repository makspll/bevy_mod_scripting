use super::*;

/// Stores the script context by entity.
pub struct DomainContext<P: IntoScriptPluginParams>(HashMap<Cow<'static, str>, Arc<Mutex<P::C>>>);

impl<P: IntoScriptPluginParams> Default for DomainContext<P> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for DomainContext<P> {
    fn get(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> Option<&Arc<Mutex<P::C>>> {
        domain.as_ref().and_then(|id| self.0.get(id))
    }
    fn insert(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>, context: P::C) -> Result<(), P::C> {
        if let Some(id) = domain {
            self.0.insert(id.clone(), Arc::new(Mutex::new(context)));
            Ok(())
        } else {
            Err(context)
        }
    }
    fn contains(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> bool {
        domain.as_ref().map(|id| self.0.contains_key(id)).unwrap_or(false)
    }
}
