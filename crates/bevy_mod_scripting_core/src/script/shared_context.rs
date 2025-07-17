use super::*;

/// Contains the shared context.
pub struct SharedContext<P: IntoScriptPluginParams>(pub Option<Arc<Mutex<P::C>>>);

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for SharedContext<P> {
    fn hash(&self, _context_key: &ContextKey) -> Option<u64> {
        self.0.is_some().then_some(0)
    }

    fn get(&self, _context_key: &ContextKey) -> Option<&Arc<Mutex<P::C>>> {
        self.0.as_ref()
    }
    fn insert(&mut self, _context_key: ContextKey, context: P::C) -> Result<(), P::C> {
        self.0 = Some(Arc::new(Mutex::new(context)));
        Ok(())
    }
    fn contains(&self, _context_key: &ContextKey) -> bool {
        self.0.is_some()
    }

    fn values(&self) -> impl Iterator<Item = &Arc<Mutex<P::C>>> {
        self.0.as_ref().into_iter()
    }
    fn iter(&self) -> impl Iterator<Item = (ContextKey, &Arc<Mutex<P::C>>)> {
        self.0.as_ref().into_iter().map(|c| (ContextKey::default(), c))
    }
    fn remove(&mut self, context_key: &ContextKey) -> bool {
        if context_key.is_empty() {
            // Only clear when nothing is specified.
            self.0 = None;
            true
        } else {
            false
        }
    }
}

impl<P: IntoScriptPluginParams> Default for SharedContext<P> {
    fn default() -> Self {
        Self(None)
    }
}
