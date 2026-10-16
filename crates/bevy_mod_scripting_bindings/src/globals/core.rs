//! The plugin setting up the namespaces scripts import

use bevy_app::{App, Plugin};
use bevy_reflect::TypeRegistration;

use super::{AppScriptNamespaceMembers, AppScriptTypeCatalogue};

/// A plugin setting up the namespaces scripts import, i.e. `import_namespace("math").Vec3`.
///
/// There are no globals besides the builtins: types, and functions or values registered into namespaces, are reached by importing their namespace,
/// which avoids types with the same name shadowing each other.
pub struct CoreScriptGlobalsPlugin {
    /// The filter function used to determine which types scripts can look up by name.
    /// When `true` for the given type registration, the type is part of the script type catalogue.
    pub filter: fn(&TypeRegistration) -> bool,
}

impl Default for CoreScriptGlobalsPlugin {
    fn default() -> Self {
        Self { filter: |_| true }
    }
}

impl Plugin for CoreScriptGlobalsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppScriptNamespaceMembers>();
        app.insert_resource(AppScriptTypeCatalogue::new(self.filter));
    }
}
