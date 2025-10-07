//! All the switchable special functions used by language implementors
use super::{FromScriptRef, FunctionCallContext, IntoScriptRef};
use crate::{error::InteropError, PartialReflectExt, ReflectReference, ReflectionPathExt, ScriptValue};
use bevy_mod_scripting_derive::DebugWithTypeInfo;
use bevy_mod_scripting_display::OrFakeId;
use bevy_reflect::{ParsedPath, PartialReflect, ReflectRef};

/// A list of magic methods, these only have one replacable implementation, and apply to all `ReflectReferences`.
/// It's up to the language implementer to call these in the right order (after any type specific overrides).
///
/// These live in a separate mini registry since they are so commonly needed. This helps us avoid needless hashing and lookups as well as script value conversions
#[derive(DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub struct MagicFunctions {
    /// Indexer function
    #[debug_with_type_info(skip)]
    pub get:
        fn(FunctionCallContext, ReflectReference, ScriptValue) -> Result<ScriptValue, InteropError>,
    /// Indexer setter function
    #[debug_with_type_info(skip)]
    pub set: fn(
        FunctionCallContext,
        ReflectReference,
        ScriptValue,
        ScriptValue,
    ) -> Result<(), InteropError>,
}

impl MagicFunctions {
    /// Calls the currently set `get` function with the given arguments.
    pub fn get(
        &self,
        ctxt: FunctionCallContext,
        reference: ReflectReference,
        key: ScriptValue,
    ) -> Result<ScriptValue, InteropError> {
        (self.get)(ctxt, reference, key)
    }

    /// Calls the currently set `set` function with the given arguments.
    pub fn set(
        &self,
        ctxt: FunctionCallContext,
        reference: ReflectReference,
        key: ScriptValue,
        value: ScriptValue,
    ) -> Result<(), InteropError> {
        (self.set)(ctxt, reference, key, value)
    }

    /// Indexes into the given reference and if the nested type is a reference type, returns a deeper reference, otherwise
    /// returns the concrete value.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to index into.
    /// * `key`: The key to index with.
    ///
    /// Returns:
    /// * `value`: The value at the key, if the reference is indexable.
    pub fn default_get(
        ctxt: FunctionCallContext,
        mut reference: ReflectReference,
        key: ScriptValue,
    ) -> Result<ScriptValue, InteropError> {
        let world = ctxt.world()?;
        
        // Check if the reference is a map type
        let is_map = reference.with_reflect(world.clone(), |r| {
            matches!(r.reflect_ref(), ReflectRef::Map(_))
        })?;
        
        if is_map {
            // Handle map indexing specially - need to get the key type and convert the script value
            let key = <Box<dyn PartialReflect>>::from_script_ref(
                reference.key_type_id(world.clone())?.ok_or_else(|| {
                    InteropError::unsupported_operation(
                        reference.tail_type_id(world.clone()).unwrap_or_default(),
                        Some(Box::new(key.clone())),
                        "Could not get key type id. Are you trying to index into a type that's not a map?".to_owned(),
                    )
                })?,
                key,
                world.clone(),
            )?;
            
            reference.with_reflect(world.clone(), |s| match s.try_map_get(key.as_ref())? {
                Some(value) => {
                    let reference = {
                        let allocator = world.allocator();
                        let mut allocator = allocator.write();
                        let owned_value = <dyn PartialReflect>::from_reflect(value, world.clone())?;
                        ReflectReference::new_allocated_boxed(owned_value, &mut allocator)
                    };
                    ReflectReference::into_script_ref(reference, world)
                }
                None => {
                    // Return None option if key doesn't exist
                    let none_option: Option<()> = None;
                    let reference = {
                        let allocator = world.allocator();
                        let mut allocator = allocator.write();
                        ReflectReference::new_allocated_boxed(Box::new(none_option), &mut allocator)
                    };
                    ReflectReference::into_script_ref(reference, world)
                }
            })?
        } else {
            // Handle path-based indexing for non-map types
            let mut path: ParsedPath = key.try_into()?;
            if ctxt.convert_to_0_indexed() {
                path.convert_to_0_indexed();
            }
            reference.index_path(path);
            ReflectReference::into_script_ref(reference, world)
        }
    }

    /// Sets the value under the specified path on the underlying value.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to set the value on.
    /// * `key`: The key to set the value at.
    /// * `value`: The value to set.
    ///
    /// Returns:
    /// * `result`: Nothing if the value was set successfully.
    pub fn default_set(
        ctxt: FunctionCallContext,
        mut reference: ReflectReference,
        key: ScriptValue,
        value: ScriptValue,
    ) -> Result<(), InteropError> {
        let world = ctxt.world()?;
        
        // Check if the reference is a map type
        let is_map = reference.with_reflect(world.clone(), |r| {
            matches!(r.reflect_ref(), ReflectRef::Map(_))
        })?;
        
        if is_map {
            // Handle map setting specially - need to get the key type and convert the script value
            let key = <Box<dyn PartialReflect>>::from_script_ref(
                reference.key_type_id(world.clone())?.ok_or_else(|| {
                    InteropError::unsupported_operation(
                        reference.tail_type_id(world.clone()).unwrap_or_default(),
                        Some(Box::new(key.clone())),
                        "Could not get key type id. Are you trying to index into a type that's not a map?".to_owned(),
                    )
                })?,
                key,
                world.clone(),
            )?;
            
            // Get the value type for the map and convert the script value
            let value_type_id = reference.element_type_id(world.clone())?.ok_or_else(|| {
                InteropError::unsupported_operation(
                    reference.tail_type_id(world.clone()).unwrap_or_default(),
                    Some(Box::new(value.clone())),
                    "Could not get value type id. Are you trying to set a value in a type that's not a map?".to_owned(),
                )
            })?;
            
            let value = <Box<dyn PartialReflect>>::from_script_ref(
                value_type_id,
                value,
                world.clone(),
            )?;
            
            reference.with_reflect_mut(world, |s| {
                s.try_insert_boxed(key, value)
            })??;
        } else {
            let mut path: ParsedPath = key.try_into()?;
            if ctxt.convert_to_0_indexed() {
                path.convert_to_0_indexed();
            }
            reference.index_path(path);
            
            let target_type_id = reference.with_reflect(world.clone(), |r| {
                r.get_represented_type_info()
                    .map(|i| i.type_id())
                    .or_fake_id()
            })?;
            
            let other = <Box<dyn PartialReflect>>::from_script_ref(target_type_id, value, world.clone())?;
            
            reference.with_reflect_mut(world, |r| {
                r.try_apply(other.as_partial_reflect())
                    .map_err(InteropError::reflect_apply_error)
            })??;
        }
        
        Ok(())
    }
}

impl Default for MagicFunctions {
    fn default() -> Self {
        Self {
            get: MagicFunctions::default_get,
            set: MagicFunctions::default_set,
        }
    }
}
