//! All the switchable special functions used by language implementors
use super::{FromScriptRef, FunctionCallContext, IntoScriptRef};
use crate::{
    bindings::{ReflectReference, ReflectionPathExt, ScriptValue},
    error::InteropError,
    reflection_extensions::TypeIdExtensions,
};
use bevy_reflect::{ParsedPath, PartialReflect};

/// A list of magic methods, these only have one replacable implementation, and apply to all `ReflectReferences`.
/// It's up to the language implementer to call these in the right order (after any type specific overrides).
///
/// These live in a separate mini registry since they are so commonly needed. This helps us avoid needless hashing and lookups as well as script value conversions
#[derive(Debug)]
pub struct MagicFunctions {
    /// Indexer function
    pub get:
        fn(FunctionCallContext, ReflectReference, ScriptValue) -> Result<ScriptValue, InteropError>,
    /// Indexer setter function
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
    /// Does not support map types at the moment, for maps see `map_get`
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
        let mut path: ParsedPath = key.try_into()?;
        if ctxt.convert_to_0_indexed() {
            path.convert_to_0_indexed();
        }
        reference.index_path(path);
        let world = ctxt.world()?;
        ReflectReference::into_script_ref(reference, world)
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
        let mut path: ParsedPath = key.try_into()?;
        if ctxt.convert_to_0_indexed() {
            path.convert_to_0_indexed();
        }
        reference.index_path(path);
        reference.with_reflect_mut(world.clone(), |r| {
            let target_type_id = r
                .get_represented_type_info()
                .map(|i| i.type_id())
                .or_fake_id();
            let other =
                <Box<dyn PartialReflect>>::from_script_ref(target_type_id, value, world.clone())?;
            r.try_apply(other.as_partial_reflect())
                .map_err(|e| InteropError::external_error(Box::new(e)))?;
            Ok::<_, InteropError>(())
        })?
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
