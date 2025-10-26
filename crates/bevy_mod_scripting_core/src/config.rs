//! Init only configuration and relevant types.

use bevy_ecs::world::WorldId;

use crate::{
    IntoScriptPluginParams,
    context::{ContextInitializer, ContextPreHandlingInitializer},
};

/// A set of global* configs keyed by the plugin params type.
///
/// Configuration is immutable after initialization.
///
/// Configs contained here should be
///
/// *global meaning stored in thread-locals, i.e. not annoyingly global, but pretty global.
#[derive(Debug)]
pub struct ScriptingPluginConfiguration<P: IntoScriptPluginParams + ?Sized> {
    /// callbacks executed before a handler callback is executed every time
    pub pre_handling_callbacks: &'static [ContextPreHandlingInitializer<P>],
    /// callbacks executed once after creating a context but before executing it for the first time
    pub context_initialization_callbacks: &'static [ContextInitializer<P>],
    /// Whether to emit responses from the core callbacks like `on_script_loaded`.
    pub emit_responses: bool,
    /// The configured runtime for the plugin
    pub runtime: &'static P::R,
    /// The language extensions this plugin supports
    pub language_extensions: &'static crate::LanguageExtensions,
}

impl<P: IntoScriptPluginParams + ?Sized> Clone for ScriptingPluginConfiguration<P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P: IntoScriptPluginParams + ?Sized> Copy for ScriptingPluginConfiguration<P> {}

/// A utility trait for accessing the readonly configuration for types that provide some.
///
/// This is typically implemented using the `make_plugin_config_static!` macro.
///
/// The default implementation will allow you to statically retrieve the configuration for a given world id.
///
/// I.e. this config is not quite thread-local but world-local, meaning it should play nice with tests.
pub trait GetPluginThreadConfig<P: IntoScriptPluginParams + ?Sized> {
    /// Get a reference to the readonly configuration.
    fn readonly_configuration(world: WorldId) -> ScriptingPluginConfiguration<P>;

    /// Set the configuration or overwrites it if already set.
    fn set_world_local_config(world: WorldId, config: ScriptingPluginConfiguration<P>);
}

#[macro_export]
/// A macro to implement `WithReadonlyConfiguration` for a given plugin type using thread-local storage.
macro_rules! make_plugin_config_static {
    ($ty:ty) => {
        static CONFIG: std::sync::RwLock<
            bevy_platform::prelude::Vec<
                Option<ScriptingPluginConfiguration<$ty>>,
            >,
        > = std::sync::RwLock::new(bevy_platform::prelude::Vec::new());
        impl GetPluginThreadConfig<$ty> for $ty {
            fn readonly_configuration(
                world: bevy_ecs::world::WorldId,
            ) -> ScriptingPluginConfiguration<$ty> {
                CONFIG
                    .read()
                    .unwrap()
                    .get(<bevy_ecs::world::WorldId as bevy_ecs::storage::SparseSetIndex>::sparse_set_index(&world))
                    .and_then(|c| *c)
                    .unwrap_or_else(||
                        panic!(
                            "Configuration for plugin {} not set for world {:?}. Did you add the plugin to the app?",
                            stringify!($ty),
                            world
                        ),
                    )
            }

            fn set_world_local_config(
                world: bevy_ecs::world::WorldId,
                config: ScriptingPluginConfiguration<$ty>,
            ) {
                let mut guard = CONFIG.write().unwrap();
                let index = <bevy_ecs::world::WorldId as bevy_ecs::storage::SparseSetIndex>::sparse_set_index(&world) as usize;
                if index >= guard.len() {
                    guard.resize_with(index + 1, || None);
                }
                guard[index] = Some(config);
            }
        }
    };
}
