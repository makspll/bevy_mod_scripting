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
    ) -> Result<ScriptValue, InteropError>;

    // fn with_call<O, F: FnOnce(Return) -> O, I: IntoIterator<Item = ScriptValue>>(
    //     &self,
    //     args: I,
    //     world: Arc<WorldAccessGuard>,
    //     f: F,
    // ) -> Result<O, InteropError>;

    // fn dynamic_call<I: IntoIterator<Item = ScriptValue>>(
    //     &self,
    //     args: I,
    //     world: Arc<WorldAccessGuard>,
    // ) -> Result<ScriptValue, InteropError>;
}

// impl CallAsScriptFunction for DynamicFunction<'_> {
//     fn with_call<O, F: FnOnce(Return) -> O, I: IntoIterator<Item = ScriptValue>>(
//         &self,
//         args: I,
//         world: Arc<WorldAccessGuard>,
//         f: F,
//     ) -> Result<O, InteropError> {
//         let info = self.info().args();

//         let mut arg_iter = args.into_iter().peekable();

//         // we also want to add the world arg if it's required as the first arg but not present
//         // let (mut args_list, mut accesses) = if self.first_arg_is_world()
//         //     && !arg_iter
//         //         .peek()
//         //         .map(|a| a == &ScriptValue::World)
//         //         .unwrap_or(false)
//         // {
//         //     std::iter::once(ScriptValue::World)
//         //         .chain(arg_iter)
//         //         .a(self.info(), world.clone())?
//         // } else {
//         //     arg_iter.into_args_list_with_access(self.info(), world.clone())?
//         // };

//         // let mut final_args_list = ArgList::default();

//         // // we sometimes want to use the boxed value in the arg instead of allocating and refing to it.
//         // // for this reason let's be lenient in calling functions. Allow passing owned values as refs
//         // for (arg, info) in args_list.iter_mut().zip(info.iter()) {
//         //     let next_arg = match (arg, info.ownership()) {
//         //         (ArgValue::Owned(r), Ownership::Ref) => {
//         //             ArgValue::Ref((r as &Box<dyn PartialReflect>).as_ref())
//         //         }
//         //         (ArgValue::Owned(r), Ownership::Mut) => ArgValue::Mut(r.as_mut()),
//         //         (v, _) => {
//         //             // muahaha, shouldn't allocate due to ZST
//         //             let a = std::mem::replace(v, ArgValue::Owned(Box::new(())));
//         //             a
//         //         }
//         //     };
//         //     final_args_list = final_args_list.push_arg(next_arg);
//         // }

//         // bevy::log::trace!(
//         //     "Calling function: {:?} with args: {:?}",
//         //     self.info().name(),
//         //     final_args_list
//         // );

//         // let return_val = match self.call(final_args_list) {
//         //     Ok(return_val) => return_val,
//         //     Err(e) => {
//         //         // Safety: we have not generated any unsafe aliases
//         //         // - we are releasing only the access we have claimed
//         //         accesses.drain(..).for_each(|(id, _)| {
//         //             unsafe { world.release_access(id) };
//         //         });

//         //         return Err(InteropError::function_call_error(e));
//         //     }
//         // };

//         bevy::log::trace!(
//             "Function: {:?} returned: {:?}",
//             self.info().name(),
//             return_val
//         );

//         let out = f(return_val);
//         // Safety: we have not generated any unsafe aliases
//         // - we are releasing only the access we have claimed
//         accesses.drain(..).for_each(|(id, _)| {
//             unsafe { world.release_access(id) };
//         });

//         Ok(out)
//     }

//     fn dynamic_call<I: IntoIterator<Item = ScriptValue>>(
//         &self,
//         args: I,
//         world: Arc<WorldAccessGuard>,
//     ) -> Result<ScriptValue, InteropError> {
//         bevy::log::debug!("Dynamic call to function: {:?}", self.info().name());
//         self.with_call(args, world.clone(), |r| {
//             match r.try_into_or_boxed::<ScriptValue>() {
//                 Ok(script_val) => Ok(script_val),
//                 Err(e) => {
//                     let allocator = world.allocator();
//                     let mut allocator = allocator.write();

//                     Ok(ReflectReference::new_allocated_boxed(e, &mut allocator).into())
//                 }
//             }
//         })
//         .flatten_interop_error()
//         .map_err(|e| InteropError::function_interop_error(self.info(), None, e))
//     }
// }

impl CallScriptFunction for DynamicFunction<'_> {
    fn call_script_function<I: IntoIterator<Item = ScriptValue>>(
        &self,
        args: I,
        world: WorldGuard,
    ) -> Result<ScriptValue, InteropError> {
        let mut args = args.into_iter().peekable();

        let add_world =
            self.first_arg_is_world() && args.peek().map_or(true, |a| a != &ScriptValue::World);

        let mut args_list = ArgList::new();

        if add_world {
            args_list = args_list.push_arg(ArgValue::Owned(Box::new(
                WorldCallbackAccess::from_guard(world.clone()),
            )));
        }

        for arg in args {
            let arg_val = ArgValue::Owned(match arg {
                ScriptValue::World => Box::new(WorldCallbackAccess::from_guard(world.clone())),
                _ => Box::new(arg),
            });

            args_list = args_list.push_arg(arg_val);
        }

        let return_val = self
            .call(args_list)
            .map_err(InteropError::function_call_error)?;

        match return_val.try_into_or_boxed::<ScriptValue>() {
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
    fn first_arg_is_world(&self) -> bool;
}

impl DynamicFunctionExt for DynamicFunction<'_> {
    fn first_arg_is_world(&self) -> bool {
        self.info().args().first().map_or(false, |arg| {
            arg.type_id() == std::any::TypeId::of::<WorldCallbackAccess>()
        })
    }
}

// pub trait ArgValueExt {
//     fn is_type_id(&self, type_id: std::any::TypeId) -> bool;
// }

// impl ArgValueExt for ArgValue<'_> {
//     fn is_type_id(&self, type_id: std::any::TypeId) -> bool {
//         match self {
//             ArgValue::Owned(r) => r
//                 .get_represented_type_info()
//                 .map_or(false, |t| t.type_id() == type_id),
//             ArgValue::Ref(r) => r
//                 .get_represented_type_info()
//                 .map_or(false, |t| t.type_id() == type_id),
//             ArgValue::Mut(r) => r
//                 .get_represented_type_info()
//                 .map_or(false, |t| t.type_id() == type_id),
//         }
//     }
// }
