#![allow(clippy::arc_with_non_send_sync)]

use crate::event::ScriptErrorEvent;
use asset::{ScriptAsset, ScriptAssetLoader, ScriptAssetSettings};
use bevy::prelude::*;
use bindings::ReflectAllocator;
use context::{
    Context, ContextAssigner, ContextBuilder, ContextInitializer, ContextLoadingSettings,
    ContextPreHandlingInitializer, ScriptContexts,
};
use handler::{Args, CallbackSettings, HandlerFn};
use prelude::{
    initialize_runtime,
    runtime::{RuntimeInitializer, RuntimeSettings},
    sync_script_data, Documentation, DocumentationFragment, ScriptCallbackEvent,
};
use runtime::{Runtime, RuntimeContainer};
use script::Scripts;
use systems::garbage_collector;

pub mod asset;
pub mod bindings;
pub mod commands;
pub mod context;
pub mod docs;
pub mod error;
pub mod event;
pub mod handler;
pub mod reflection_extensions;
pub mod runtime;
pub mod script;
pub mod systems;
pub mod world;
pub mod prelude {
    pub use {crate::docs::*, crate::error::*, crate::event::*, crate::systems::*, crate::*};
}

/// Bevy plugin enabling scripting within the bevy mod scripting framework
pub struct ScriptingPlugin<A: Args, C: Context, R: Runtime> {
    /// Callback for initiating the runtime
    pub runtime_builder: fn() -> R,
    /// Settings for the runtime
    pub runtime_settings: Option<RuntimeSettings<R>>,
    /// The handler used for executing callbacks in scripts
    pub callback_handler: Option<HandlerFn<A, C, R>>,
    /// The context builder for loading contexts
    pub context_builder: Option<ContextBuilder<C, R>>,
    /// The context assigner for assigning contexts to scripts, if not provided default strategy of keeping each script in its own context is used
    pub context_assigner: Option<ContextAssigner<C>>,
}

impl<A: Args, C: Context, R: Runtime + Default> Default for ScriptingPlugin<A, C, R> {
    fn default() -> Self {
        Self {
            runtime_builder: R::default,
            runtime_settings: Default::default(),
            callback_handler: Default::default(),
            context_builder: Default::default(),
            context_assigner: Default::default(),
        }
    }
}

impl<A: Args, C: Context, R: Runtime> Plugin for ScriptingPlugin<A, C, R> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ScriptErrorEvent>()
            .add_event::<ScriptCallbackEvent<A>>()
            .init_resource::<ReflectAllocator>()
            .init_resource::<ScriptAssetSettings>()
            .init_resource::<Scripts>()
            .init_asset::<ScriptAsset>()
            .register_asset_loader(ScriptAssetLoader {
                language: "<>".into(),
                extensions: &[],
                preprocessor: None,
            })
            .insert_resource(self.runtime_settings.as_ref().cloned().unwrap_or_default())
            .insert_non_send_resource::<RuntimeContainer<R>>(RuntimeContainer {
                runtime: (self.runtime_builder)(),
            })
            .init_non_send_resource::<ScriptContexts<C>>()
            .insert_resource::<CallbackSettings<A, C, R>>(CallbackSettings {
                callback_handler: self.callback_handler,
            })
            .insert_resource::<ContextLoadingSettings<C, R>>(ContextLoadingSettings {
                loader: self.context_builder.clone(),
                assigner: Some(self.context_assigner.clone().unwrap_or_default()),
                context_initializers: vec![],
                context_pre_handling_initializers: vec![],
            })
            .add_systems(PostUpdate, (garbage_collector, sync_script_data::<C, R>))
            .add_systems(PostStartup, initialize_runtime::<R>);
    }
}

pub trait AddRuntimeInitializer<R: Runtime> {
    fn add_runtime_initializer(&mut self, initializer: RuntimeInitializer<R>) -> &mut Self;
}

impl<R: Runtime> AddRuntimeInitializer<R> for App {
    fn add_runtime_initializer(&mut self, initializer: RuntimeInitializer<R>) -> &mut Self {
        if !self.world_mut().contains_resource::<RuntimeSettings<R>>() {
            self.world_mut().init_resource::<RuntimeSettings<R>>();
        }
        self.world_mut()
            .resource_mut::<RuntimeSettings<R>>()
            .as_mut()
            .initializers
            .push(initializer);
        self
    }
}

pub trait AddContextInitializer<C: Context> {
    fn add_context_initializer<R: Runtime>(
        &mut self,
        initializer: ContextInitializer<C>,
    ) -> &mut Self;
}

impl<C: Context> AddContextInitializer<C> for App {
    fn add_context_initializer<R: Runtime>(
        &mut self,
        initializer: ContextInitializer<C>,
    ) -> &mut Self {
        self.world_mut()
            .init_resource::<ContextLoadingSettings<C, R>>();
        self.world_mut()
            .resource_mut::<ContextLoadingSettings<C, R>>()
            .as_mut()
            .context_initializers
            .push(initializer);
        self
    }
}

pub trait AddContextPreHandlingInitializer<C: Context> {
    fn add_context_pre_handling_initializer<R: Runtime>(
        &mut self,
        initializer: ContextPreHandlingInitializer<C>,
    ) -> &mut Self;
}

impl<C: Context> AddContextPreHandlingInitializer<C> for App {
    fn add_context_pre_handling_initializer<R: Runtime>(
        &mut self,
        initializer: ContextPreHandlingInitializer<C>,
    ) -> &mut Self {
        self.world_mut()
            .resource_mut::<ContextLoadingSettings<C, R>>()
            .as_mut()
            .context_pre_handling_initializers
            .push(initializer);
        self
    }
}

pub trait StoreDocumentation<D: DocumentationFragment> {
    /// Adds a documentation fragment to the documentation store.
    fn add_documentation_fragment(&mut self, fragment: D) -> &mut Self;
    /// Consumes all the stored documentation fragments, and merges them into one, then generates the documentation.
    fn generate_docs(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

impl<D: DocumentationFragment> StoreDocumentation<D> for App {
    fn add_documentation_fragment(&mut self, fragment: D) -> &mut Self {
        self.world_mut()
            .init_non_send_resource::<Documentation<D>>();
        self.world_mut()
            .non_send_resource_mut::<Documentation<D>>()
            .as_mut()
            .fragments
            .push(fragment);
        self
    }

    fn generate_docs(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut docs = match self
            .world_mut()
            .remove_non_send_resource::<Documentation<D>>()
        {
            Some(docs) => docs,
            None => return Ok(()),
        };

        let mut top_fragment = match docs.fragments.pop() {
            Some(fragment) => fragment,
            None => return Ok(()),
        };

        for fragment in docs.fragments.into_iter() {
            top_fragment = top_fragment.merge(fragment);
        }

        top_fragment.gen_docs()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_scripting_plugin_initializes_all_resources_correctly() {
        let mut app = App::new();
        #[derive(Default, Clone)]
        struct A;
        #[derive(Default, Clone)]
        struct C;
        #[derive(Default, Clone)]
        struct R;
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(ScriptingPlugin::<A, C, R>::default());

        assert!(app.world().contains_resource::<Scripts>());
        assert!(app.world().contains_resource::<ReflectAllocator>());
        assert!(app.world().contains_resource::<ScriptAssetSettings>());
        assert!(app.world().contains_resource::<RuntimeSettings<R>>());
        assert!(app.world().contains_resource::<CallbackSettings<A, C, R>>());
        assert!(app
            .world()
            .contains_resource::<ContextLoadingSettings<C, R>>());
        assert!(app.world().contains_non_send::<RuntimeContainer<R>>());
        assert!(app.world().contains_non_send::<ScriptContexts<C>>());
    }
}
