//! Script bindings for bevy assets: loading, handles and adding assets.
//!
//! Every asset type has `load`, `add_asset` and `get_asset`, i.e. `Image.load(path)`, which are documented with the
//! type's own handle, i.e. `Handle<Image>`. The same functions exist on `world`, taking the asset type as an argument.

use std::any::TypeId;

use bevy_app::{App, Plugin};
use bevy_asset::{AssetServer, LoadState, ReflectAsset, ReflectHandle, UntypedHandle};
// `script_bindings` expects these in scope
use bevy_ecs::world::World;
use bevy_mod_scripting_bindings::{
    InteropError, ReflectReference, ScriptTypeRegistration, StaticTypeRef, WorldExtensions,
    WorldGuard,
    function::{
        bound::{BoundHandle, BoundValue},
        from::V,
        script_function::FunctionCallContext,
    },
};
use bevy_mod_scripting_derive::script_bindings;

/// Registers the asset script bindings.
#[derive(Default)]
pub struct BevyAssetBindingsPlugin;

impl Plugin for BevyAssetBindingsPlugin {
    fn finish(&self, app: &mut App) {
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &["bevy_asset", "bevy_scene", "bevy_gltf"],
            "asset",
        );
    }

    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        register_world_asset_functions(world);
        register_asset_type_functions(world);
    }
}

/// The [`ReflectAsset`] type data of the asset type, or an error saying how to register it.
fn reflect_asset(world: &WorldGuard, asset_type_id: TypeId) -> Result<ReflectAsset, InteropError> {
    let registry = world.type_registry();
    let registry = registry.read();
    registry
        .get_type_data::<ReflectAsset>(asset_type_id)
        .cloned()
        .ok_or_else(|| {
            InteropError::str(
                "the type is not registered as an asset, use `register_asset_reflect`",
            )
        })
}

/// Starts loading an asset of the given type, returning a reference to its `Handle<T>`.
fn load(
    world: &WorldGuard,
    asset_type_id: TypeId,
    path: String,
) -> Result<ReflectReference, InteropError> {
    let handle = world.with_resource(|asset_server: &AssetServer| {
        asset_server.load_builder().load_erased(asset_type_id, path)
    })?;
    typed_handle_reference(world, asset_type_id, handle)
}

/// Adds a reflected value as a new asset of its type, returning a reference to its `Handle<T>`.
fn add(world: &WorldGuard, value: ReflectReference) -> Result<ReflectReference, InteropError> {
    let asset_type_id = value
        .tail_type_id(world.clone())?
        .ok_or_else(|| InteropError::str("cannot determine the type of the asset value"))?;
    let owned = value.to_owned_value(world.clone())?;
    let reflect_asset = reflect_asset(world, asset_type_id)?;
    let handle = world.with_world_mut(|w| reflect_asset.add(w, &*owned))?;
    typed_handle_reference(world, asset_type_id, handle)
}

/// A reference to the asset of the given type a handle points to, or `None` if there is no such asset (yet).
fn get(
    world: &WorldGuard,
    asset_type_id: TypeId,
    handle: ReflectReference,
) -> Result<Option<ReflectReference>, InteropError> {
    let handle = handle.try_untyped_asset_handle(world.clone())?;
    let reflect_asset = reflect_asset(world, asset_type_id)?;
    if !world.with_world(|w| reflect_asset.get(w, handle.id()).is_some())? {
        return Ok(None);
    }
    ReflectReference::new_asset_ref(handle, asset_type_id, world.clone()).map(Some)
}

#[script_bindings(
    remote,
    name = "asset_type_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
impl StaticTypeRef {
    /// Starts loading an asset of this type from a path, returning a handle to it right away.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `asset_type`: The asset type, filled in automatically.
    /// * `path`: The asset path, relative to the assets folder.
    /// Returns:
    /// * `handle`: A handle to the asset, which is loaded in the background.
    fn load(
        ctxt: FunctionCallContext,
        asset_type: V<StaticTypeRef>,
        path: String,
    ) -> Result<BoundHandle, InteropError> {
        let world = ctxt.world()?;
        let asset_type_id = asset_type.type_info(&world)?.type_id();
        load(&world, asset_type_id, path).map(BoundHandle)
    }

    /// Adds a value of this type as a new asset, returning a strong handle to it.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `asset_type`: The asset type, filled in automatically.
    /// * `value`: The asset value.
    /// Returns:
    /// * `handle`: A handle to the new asset.
    fn add_asset(
        ctxt: FunctionCallContext,
        asset_type: V<StaticTypeRef>,
        value: BoundValue,
    ) -> Result<BoundHandle, InteropError> {
        let world = ctxt.world()?;
        let asset_type_id = asset_type.type_info(&world)?.type_id();
        if value.0.tail_type_id(world.clone())? != Some(asset_type_id) {
            return Err(InteropError::str(
                "the value is not of the asset type it is added as",
            ));
        }
        add(&world, value.0).map(BoundHandle)
    }

    /// The asset a handle points to, or nil if it isn't loaded or added yet.
    ///
    /// Changes to the returned value change the asset.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `asset_type`: The asset type, filled in automatically.
    /// * `handle`: The handle to the asset.
    /// Returns:
    /// * `asset`: The asset, or nil.
    fn get_asset(
        ctxt: FunctionCallContext,
        asset_type: V<StaticTypeRef>,
        handle: BoundHandle,
    ) -> Result<Option<BoundValue>, InteropError> {
        let world = ctxt.world()?;
        let asset_type_id = asset_type.type_info(&world)?.type_id();
        Ok(get(&world, asset_type_id, handle.0)?.map(BoundValue))
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
        load(&ctxt.world()?, asset_type.type_id(), path)
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
        add(&ctxt.world()?, value)
    }
}
