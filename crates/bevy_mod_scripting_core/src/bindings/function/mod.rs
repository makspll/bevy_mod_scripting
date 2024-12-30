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
use script_function::CallerContext;

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
        &self,
        args: I,
        world: WorldGuard,
        context: CallerContext,
    ) -> Result<ScriptValue, InteropError>;
}

impl CallScriptFunction for DynamicFunction<'_> {
    fn call_script_function<I: IntoIterator<Item = ScriptValue>>(
        &self,
        args: I,
        world: WorldGuard,
        context: CallerContext,
    ) -> Result<ScriptValue, InteropError> {
        let args = args.into_iter();

        let add_context = self.is_script_function();
        let mut args_list = ArgList::new();

        if add_context {
            args_list = args_list.push_arg(ArgValue::Owned(Box::new(context)));
            args_list = args_list.push_arg(ArgValue::Owned(Box::new(
                WorldCallbackAccess::from_guard(world.clone()),
            )));
        }

        for arg in args {
            let arg_val = ArgValue::Owned(Box::new(arg));
            args_list = args_list.push_arg(arg_val);
        }

        let return_val = self
            .call(args_list)
            .map_err(InteropError::function_call_error)?;

        match return_val.try_into_or_boxed::<ScriptValue>() {
            Ok(ScriptValue::Error(e)) => Err(InteropError::function_interop_error(self.info(), e)),
            Ok(v) => Ok(v),
            Err(b) => {
                let allocator = world.allocator();
                let mut allocator = allocator.write();

                Ok(ReflectReference::new_allocated_boxed(b, &mut allocator).into())
            }
        }
    }
}

pub trait DynamicFunctionExt {
    fn is_script_function(&self) -> bool;
}

impl DynamicFunctionExt for DynamicFunction<'_> {
    fn is_script_function(&self) -> bool {
        self.info().args().first().map_or(false, |arg| {
            arg.type_id() == std::any::TypeId::of::<CallerContext>()
                || arg.type_id() == std::any::TypeId::of::<WorldCallbackAccess>()
        })
    }
}
