use super::*;

/// Stores the script context by entity.
pub struct DomainContext<P: IntoScriptPluginParams>(HashMap<Domain, Arc<Mutex<P::C>>>);

impl<P: IntoScriptPluginParams> Default for DomainContext<P> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for DomainContext<P> {
    fn hash(&self, context_key: &ContextKey) -> Option<u64> {
        context_key.domain.map(|d| DefaultHashBuilder::default().hash_one(d))
    }
    fn get(&self, context_key: &ContextKey) -> Option<&Arc<Mutex<P::C>>> {
        context_key.domain.and_then(|id| self.0.get(&id))
    }
    fn insert(&mut self, context_key: ContextKey, context: P::C) -> Result<(), P::C> {
        if let Some(id) = context_key.domain {
            self.0.insert(id, Arc::new(Mutex::new(context)));
            Ok(())
        } else {
            Err(context)
        }
    }
    fn contains(&self, context_key: &ContextKey) -> bool {
        context_key.domain.map(|id| self.0.contains_key(&id)).unwrap_or(false)
    }
    fn values(&self) -> impl Iterator<Item = &Arc<Mutex<P::C>>> {
        self.0.values()
    }
    fn remove(&mut self, context_key: &ContextKey) -> bool {
        context_key.domain.map(|id| self.0.remove(&id).is_some()).unwrap_or(false)
    }
    fn iter(&self) -> impl Iterator<Item = (ContextKey, &Arc<Mutex<P::C>>)> {
        self.0.iter().map(|(domain, c)| ((*domain).into(), c))
    }
}
