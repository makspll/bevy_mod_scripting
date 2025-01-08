pub mod from;
pub mod from_ref;
pub mod into;
pub mod into_ref;
pub mod script_function;

use script_function::{CallerContext, DynamicScriptFunction, DynamicScriptFunctionMut};

use crate::error::InteropError;

use super::{script_value::ScriptValue, WorldCallbackAccess, WorldGuard};

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
            ScriptValue::Error(e) => Err(InteropError::function_interop_error(
                self.name(),
                context.self_type,
                e,
            )),
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
            ScriptValue::Error(e) => Err(InteropError::function_interop_error(
                self.name(),
                context.self_type,
                e,
            )),
            v => Ok(v),
        }
    }
}
