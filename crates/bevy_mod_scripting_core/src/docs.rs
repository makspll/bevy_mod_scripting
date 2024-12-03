use bevy::ecs::system::Resource;

/// A documentation piece which can be used to make a piece of documentation, most often a module.
pub trait DocumentationFragment: 'static + Sized {
    /// Merges two documentation fragments into one, retaining the title of the first fragment.
    fn merge(self, o: Self) -> Self;
    fn gen_docs(self) -> Result<(), Box<dyn std::error::Error>>;

    /// Retrieves the name of the documentation fragment, most likely the name of your game!
    fn name(&self) -> &'static str;
}

#[derive(Resource)]
pub struct Documentation<T: DocumentationFragment> {
    pub fragments: Vec<T>,
}

impl<T: DocumentationFragment> Default for Documentation<T> {
    fn default() -> Self {
        Self {
            fragments: Default::default(),
        }
    }
}
