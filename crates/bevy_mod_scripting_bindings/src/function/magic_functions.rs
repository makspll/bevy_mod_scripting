//! All the switchable special functions used by language implementors
use super::FunctionCallContext;
use crate::{error::InteropError, ReflectReference, ScriptValue};
use bevy_mod_scripting_derive::DebugWithTypeInfo;

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
        reference.get_indexed(key, world, ctxt.convert_to_0_indexed())
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
        reference.set_indexed(key, value, world, ctxt.convert_to_0_indexed())
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
