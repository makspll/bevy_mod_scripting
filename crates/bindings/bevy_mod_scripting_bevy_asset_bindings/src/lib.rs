//! Script bindings for bevy assets: loading, handles and adding assets.
//!
//! Handles are generic, so these functions take the asset type as a runtime type registration.

use std::any::TypeId;

use bevy_app::{App, Plugin};
use bevy_asset::{AssetServer, LoadState, ReflectAsset, ReflectHandle, UntypedHandle};
// `script_bindings` expects these in scope
use bevy_ecs::world::World;
use bevy_mod_scripting_bindings::{
    InteropError, ReflectReference, ScriptTypeRegistration, WorldExtensions, WorldGuard,
    function::{from::V, script_function::FunctionCallContext},
};
use bevy_mod_scripting_derive::script_bindings;

/// Registers the asset script bindings.
#[derive(Default)]
pub struct BevyAssetBindingsPlugin;

impl Plugin for BevyAssetBindingsPlugin {
    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        register_world_asset_functions(world);
    }
}

/// Wraps an untyped handle in a reference to a `Handle<T>`, where `T` is the asset type with `asset_type_id`.
///
/// The handle type is found through the [`ReflectHandle`] type data of the registered handle types.
fn typed_handle_reference(
    world: &WorldGuard,
    asset_type_id: TypeId,
    handle: UntypedHandle,
) -> Result<ReflectReference, InteropError> {
    let boxed = {
        let registry = world.type_registry();
        let registry = registry.read();
        let reflect_handle = registry
            .iter()
            .find_map(|registration| {
                registration
                    .data::<ReflectHandle>()
                    .filter(|h| h.asset_type_id() == asset_type_id)
            })
            .ok_or_else(|| {
                InteropError::str(
                    "the asset type is not registered for asset reflection, use `register_asset_reflect`",
                )
            })?;
        reflect_handle.typed(handle)
    };
    let allocator = world.allocator();
    let mut allocator = allocator.write();
    Ok(ReflectReference::new_allocated_boxed(boxed, &mut allocator))
}

#[script_bindings(
    remote,
    name = "world_asset_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    unregistered
)]
impl World {
    /// Starts loading an asset of the given type from a path, returning a handle to it right away.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `path`: The asset path, relative to the assets folder.
    /// * `asset_type`: The type of the asset, i.e. `bevy_mod_scripting_asset::script_asset::ScriptAsset`.
    /// Returns:
    /// * `handle`: A handle to the asset, which is loaded in the background.
    fn load_asset(
        ctxt: FunctionCallContext,
        path: String,
        asset_type: V<ScriptTypeRegistration>,
    ) -> Result<ReflectReference, InteropError> {
        let world = ctxt.world()?;
        let asset_type_id = asset_type.type_id();
        let handle = world.with_resource(|asset_server: &AssetServer| {
            asset_server.load_builder().load_erased(asset_type_id, path)
        })?;
        typed_handle_reference(&world, asset_type_id, handle)
    }

    /// Whether the asset and all of its dependencies are loaded.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `handle`: The handle to the asset.
    /// Returns:
    /// * `loaded`: Whether the asset and its dependencies are loaded.
    fn is_loaded(
        ctxt: FunctionCallContext,
        handle: ReflectReference,
    ) -> Result<bool, InteropError> {
        let world = ctxt.world()?;
        let handle = handle.try_untyped_asset_handle(world.clone())?;
        world.with_resource(|asset_server: &AssetServer| {
            asset_server.is_loaded_with_dependencies(handle.id())
        })
    }

    /// The loading state of the asset: `"NotLoaded"`, `"Loading"`, `"Loaded"` or `"Failed"`.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `handle`: The handle to the asset.
    /// Returns:
    /// * `state`: The name of the loading state.
    fn load_state(
        ctxt: FunctionCallContext,
        handle: ReflectReference,
    ) -> Result<String, InteropError> {
        let world = ctxt.world()?;
        let handle = handle.try_untyped_asset_handle(world.clone())?;
        world.with_resource(|asset_server: &AssetServer| {
            match asset_server.load_state(handle.id()) {
                LoadState::NotLoaded => "NotLoaded",
                LoadState::Loading => "Loading",
                LoadState::Loaded => "Loaded",
                LoadState::Failed(_) => "Failed",
            }
            .to_owned()
        })
    }

    /// The path the asset was loaded from, or nil if the handle is invalid or the asset was not loaded from a path.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `handle`: The handle to the asset.
    /// Returns:
    /// * `path`: The asset path, if known.
    fn asset_path(ctxt: FunctionCallContext, handle: ReflectReference) -> Option<String> {
        let world = ctxt.world().ok()?;
        let handle = handle.try_untyped_asset_handle(world.clone()).ok()?;
        world
            .with_resource(|asset_server: &AssetServer| {
                asset_server.get_path(handle.id()).map(|p| p.to_string())
            })
            .ok()
            .flatten()
    }

    /// Adds a value as a new asset of its type, returning a strong handle to it.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `value`: The asset value, of a type registered for asset reflection.
    /// Returns:
    /// * `handle`: A handle to the new asset.
    fn add_asset(
        ctxt: FunctionCallContext,
        value: ReflectReference,
    ) -> Result<ReflectReference, InteropError> {
        let world = ctxt.world()?;
        let asset_type_id = value
            .tail_type_id(world.clone())?
            .ok_or_else(|| InteropError::str("cannot determine the type of the asset value"))?;
        let owned = value.to_owned_value(world.clone())?;
        let reflect_asset = {
            let registry = world.type_registry();
            let registry = registry.read();
            registry
                .get_type_data::<ReflectAsset>(asset_type_id)
                .cloned()
        }
        .ok_or_else(|| {
            InteropError::str(
                "the asset type is not registered for asset reflection, use `register_asset_reflect`",
            )
        })?;
        let handle = world.with_world_mut(|w| reflect_asset.add(w, &*owned))?;
        typed_handle_reference(&world, asset_type_id, handle)
    }
}
