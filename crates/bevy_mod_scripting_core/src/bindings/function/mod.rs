use std::{any::TypeId, borrow::Cow, ops::Deref, sync::Arc};

pub mod from;
pub mod from_ref;
pub mod into;
pub mod into_ref;
pub mod script_function;

use bevy::reflect::{
    func::{
        args::{Arg, ArgInfo, Ownership},
        ArgList, ArgValue, DynamicFunction, FunctionInfo, FunctionResult, Return,
    },
    PartialReflect,
};
use script_function::{CallerContext, DynamicScriptFunction, DynamicScriptFunctionMut};

use crate::{
    error::{FlattenError, InteropError, InteropErrorInner, ScriptError, ScriptResult},
    reflection_extensions::{PartialReflectExt, ReturnValExt},
};

use super::{
    access_map::ReflectAccessId, pretty_print::DisplayWithWorld, script_value::ScriptValue,
    ReflectBase, ReflectReference, WorldAccessGuard, WorldCallbackAccess, WorldGuard,
};

/// Can be implemented for callables which require dynamic access to the world to be called.
///
/// The claim and release functions must be used to scope the access to the world such that function output .
pub trait CallScriptFunction {
    fn call_script_function<I: IntoIterator<Item = ScriptValue>>(
        &mut self,
        args: I,
        world: WorldGuard,
        context: CallerContext,
    ) -> Result<ScriptValue, InteropError>;
}

impl CallScriptFunction for DynamicScriptFunction {
    fn call_script_function<I: IntoIterator<Item = ScriptValue>>(
        &mut self,
        args: I,
        world: WorldGuard,
        context: CallerContext,
    ) -> Result<ScriptValue, InteropError> {
        let args = args.into_iter().collect::<Vec<_>>();
        let world_callback_access = WorldCallbackAccess::from_guard(world.clone());
        // should we be inlining call errors into the return value?
        let return_val = self.call(context, world_callback_access, args);
        match return_val {
            ScriptValue::Error(e) => Err(InteropError::function_interop_error(self.name(), e)),
            v => Ok(v),
        }
    }
}

impl CallScriptFunction for DynamicScriptFunctionMut {
    fn call_script_function<I: IntoIterator<Item = ScriptValue>>(
        &mut self,
        args: I,
        world: WorldGuard,
        context: CallerContext,
    ) -> Result<ScriptValue, InteropError> {
        let args = args.into_iter().collect::<Vec<_>>();
        let world_callback_access = WorldCallbackAccess::from_guard(world.clone());
        // should we be inlining call errors into the return value?
        let return_val = self.call(context, world_callback_access, args);
        match return_val {
            ScriptValue::Error(e) => Err(InteropError::function_interop_error(self.name(), e)),
            v => Ok(v),
        }
    }
}
