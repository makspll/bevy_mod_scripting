use std::{any::TypeId, borrow::Cow, ops::Deref, sync::Arc};

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
    access_map::ReflectAccessId,
    pretty_print::DisplayWithWorld,
    script_value::{FromScriptValue, IntoScriptValue, ScriptValue},
    ReflectBase, ReflectReference, WorldAccessGuard, WorldCallbackAccess, WorldGuard,
};

/// Can be implemented for callables which require dynamic access to the world to be called.
///
/// The claim and release functions must be used to scope the access to the world such that function output .
pub trait CallableWithAccess {
    fn with_call<O, F: FnOnce(Return) -> O, I: IntoIterator<Item = ScriptValue>>(
        &self,
        args: I,
        world: Arc<WorldAccessGuard>,
        f: F,
    ) -> Result<O, InteropError>;

    fn dynamic_call<I: IntoIterator<Item = ScriptValue>>(
        &self,
        args: I,
        world: Arc<WorldAccessGuard>,
    ) -> Result<ScriptValue, InteropError>;
}

impl CallableWithAccess for DynamicFunction<'_> {
    fn with_call<O, F: FnOnce(Return) -> O, I: IntoIterator<Item = ScriptValue>>(
        &self,
        args: I,
        world: Arc<WorldAccessGuard>,
        f: F,
    ) -> Result<O, InteropError> {
        let info = self.info().args();

        // We need to:
        // 1. Claim the correct access for each argument
        // 2. Convert to ArgsList for the function call
        // 3. Call the function
        // 4. Relinquish access to the caller for the return value
        // 5. Release the access for each argument
        // 6. Return the result
        let mut arg_iter = args.into_iter().peekable();

        // we also want to add the world arg if it's required as the first arg but not present
        let (mut args_list, mut accesses) = if self.first_arg_is_world()
            && !arg_iter
                .peek()
                .map(|a| a == &ScriptValue::World)
                .unwrap_or(false)
        {
            std::iter::once(ScriptValue::World)
                .chain(arg_iter)
                .into_args_list_with_access(self.info(), world.clone())?
        } else {
            arg_iter.into_args_list_with_access(self.info(), world.clone())?
        };

        let mut final_args_list = ArgList::default();

        // we sometimes want to use the boxed value in the arg instead of allocating and refing to it.
        // for this reason let's be lenient in calling functions. Allow passing owned values as refs
        for (arg, info) in args_list.iter_mut().zip(info.iter()) {
            let next_arg = match (arg, info.ownership()) {
                (ArgValue::Owned(r), Ownership::Ref) => {
                    ArgValue::Ref((r as &Box<dyn PartialReflect>).as_ref())
                }
                (ArgValue::Owned(r), Ownership::Mut) => ArgValue::Mut(r.as_mut()),
                (v, _) => {
                    // muahaha, shouldn't allocate due to ZST
                    let a = std::mem::replace(v, ArgValue::Owned(Box::new(())));
                    a
                }
            };
            final_args_list = final_args_list.push_arg(next_arg);
        }

        bevy::log::trace!(
            "Calling function: {:?} with args: {:?}",
            self.info().name(),
            final_args_list
        );

        let return_val = match self.call(final_args_list) {
            Ok(return_val) => return_val,
            Err(e) => {
                // Safety: we have not generated any unsafe aliases
                // - we are releasing only the access we have claimed
                accesses.drain(..).for_each(|(id, _)| {
                    unsafe { world.release_access(id) };
                });

                return Err(InteropError::function_call_error(e));
            }
        };

        bevy::log::trace!(
            "Function: {:?} returned: {:?}",
            self.info().name(),
            return_val
        );

        let out = f(return_val);
        // Safety: we have not generated any unsafe aliases
        // - we are releasing only the access we have claimed
        accesses.drain(..).for_each(|(id, _)| {
            unsafe { world.release_access(id) };
        });

        Ok(out)
    }

    fn dynamic_call<I: IntoIterator<Item = ScriptValue>>(
        &self,
        args: I,
        world: Arc<WorldAccessGuard>,
    ) -> Result<ScriptValue, InteropError> {
        bevy::log::debug!("Dynamic call to function: {:?}", self.info().name());
        self.with_call(args, world.clone(), |r| {
            let conversion = match r {
                Return::Owned(partial_reflect) => {
                    match partial_reflect.as_ref().into_script_value(world.clone()) {
                        Err(e)
                            if matches!(
                                e.inner(),
                                InteropErrorInner::BetterConversionExists { .. }
                            ) =>
                        {
                            let allocator = world.allocator();
                            let mut allocator = allocator.write();
                            ReflectReference::new_allocated_boxed(partial_reflect, &mut allocator)
                                .into_script_value(world.clone())
                        }
                        e => e,
                    }
                }
                v => {
                    let ref_ = v.as_ref();

                    match ref_.into_script_value(world.clone()) {
                        Err(e)
                            if matches!(
                                e.inner(),
                                InteropErrorInner::BetterConversionExists { .. }
                            ) =>
                        {
                            let val =
                                <dyn PartialReflect>::from_reflect_or_clone(ref_, world.clone());
                            let allocator = world.allocator();
                            let mut allocator = allocator.write();

                            ReflectReference::new_allocated_boxed(val, &mut allocator)
                                .into_script_value(world.clone())
                        }
                        e => e,
                    }
                }
            };

            conversion
        })
        .flatten_interop_error()
        .map_err(|e| InteropError::function_interop_error(self.info(), None, e))
    }
}

/// Trait implementable by lists of things representing arguments which can be converted into an `ArgList`.
///
/// The call needs to collect the correct access id's and types for each argument.
pub trait IntoArgsListWithAccess {
    fn into_args_list_with_access<'w>(
        self,
        function_info: &FunctionInfo,
        world: WorldGuard<'w>,
    ) -> Result<(Vec<ArgValue<'w>>, Vec<(ReflectAccessId, Ownership)>), InteropError>;
}

impl<I: Iterator<Item = ScriptValue>> IntoArgsListWithAccess for I {
    /// Converts a list of references into an `ArgList` and collects the access id's and types for each argument.
    ///
    /// This is actually safe to call, since we are not actually releasing the access here.
    /// Meaning that only after releasing access is it possible to create unsafe aliases
    fn into_args_list_with_access<'w>(
        self,
        function_info: &FunctionInfo,
        world: WorldGuard<'w>,
    ) -> Result<(Vec<ArgValue<'w>>, Vec<(ReflectAccessId, Ownership)>), InteropError> {
        let arg_info = function_info.args();
        let mut accesses = Vec::default();
        let mut arg_list = Vec::default();

        let release_accesses = |accesses: &mut Vec<(ReflectAccessId, Ownership)>| {
            accesses.iter().for_each(|(id, _)| {
                // Safety: we have not generated any unsafe aliases
                unsafe { world.release_access(*id) };
            });
        };

        for (value, argument_info) in self.zip(arg_info.iter()) {
            match value {
                // TODO: I'd see the logic be a bit cleaner, this if is needed to support 'Raw' ScriptValue arguments
                // as we do not want to claim any access since we're not using the value yet

                // for owned values the from_script_value impl for ReflectReferences will deal with the case
                // here we try to make an actual reference, or otherwise clone the value
                ScriptValue::Reference(arg_ref)
                    if argument_info.type_id() != TypeId::of::<ScriptValue>()
                        && matches!(argument_info.ownership(), Ownership::Mut | Ownership::Ref) =>
                {
                    let access_id = ReflectAccessId::for_reference(arg_ref.base.base_id.clone())
                        .ok_or_else(|| {
                            InteropError::function_interop_error(
                                function_info,
                                Some(argument_info),
                                InteropError::unregistered_base(arg_ref.base.clone()),
                            )
                        })?;

                    let arg = match argument_info.ownership() {
                        Ownership::Ref => {
                            if world.claim_read_access(access_id) {
                                accesses.push((access_id, Ownership::Ref));
                                let ref_ = unsafe { arg_ref.reflect_unsafe(world.clone()) };
                                if let Ok(ref_) = ref_ {
                                    Ok(ArgValue::Ref(ref_))
                                } else {
                                    Err(ref_.expect_err("invariant"))
                                }
                            } else {
                                Err(InteropError::cannot_claim_access(arg_ref.base.clone()))
                            }
                        }
                        Ownership::Mut => {
                            if world.claim_write_access(access_id) {
                                accesses.push((access_id, Ownership::Mut));
                                let mut_ref = unsafe { arg_ref.reflect_mut_unsafe(world.clone()) };
                                if let Ok(mut_ref) = mut_ref {
                                    Ok(ArgValue::Mut(mut_ref))
                                } else {
                                    Err(mut_ref.expect_err("invariant"))
                                }
                            } else {
                                Err(InteropError::cannot_claim_access(arg_ref.base.clone()))
                            }
                        }
                        _ => unreachable!(),
                    };

                    // TODO: check if the value is actually a `dyn Reflect` and not a dynamic

                    match arg {
                        Err(e) => {
                            release_accesses(&mut accesses);
                            return Err(InteropError::function_interop_error(
                                function_info,
                                Some(argument_info),
                                e,
                            ));
                        }
                        Ok(arg) => arg_list.push(arg),
                    }
                }
                ScriptValue::World => {
                    arg_list.push(ArgValue::Owned(Box::new(WorldCallbackAccess::from_guard(
                        world.clone(),
                    ))));
                }
                value => {
                    let value = match <dyn PartialReflect>::from_script_value(
                        value,
                        world.clone(),
                        argument_info.type_id(),
                    ) {
                        Some(Ok(v)) => v,
                        Some(Err(e)) => {
                            // Safety: Same as above
                            accesses.iter().for_each(|(id, _)| {
                                unsafe { world.release_access(*id) };
                            });
                            return Err(InteropError::function_interop_error(
                                function_info,
                                Some(argument_info),
                                e,
                            ));
                        }
                        None => {
                            release_accesses(&mut accesses);
                            return Err(InteropError::function_interop_error(
                                function_info,
                                Some(argument_info),
                                InteropError::impossible_conversion(argument_info.type_id()),
                            ));
                        }
                    };

                    arg_list.push(ArgValue::Owned(value));
                }
            }
        }

        Ok((arg_list, accesses))
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

pub trait ArgValueExt {
    fn is_type_id(&self, type_id: std::any::TypeId) -> bool;
}

impl ArgValueExt for ArgValue<'_> {
    fn is_type_id(&self, type_id: std::any::TypeId) -> bool {
        match self {
            ArgValue::Owned(r) => r
                .get_represented_type_info()
                .map_or(false, |t| t.type_id() == type_id),
            ArgValue::Ref(r) => r
                .get_represented_type_info()
                .map_or(false, |t| t.type_id() == type_id),
            ArgValue::Mut(r) => r
                .get_represented_type_info()
                .map_or(false, |t| t.type_id() == type_id),
        }
    }
}

#[cfg(test)]
mod test {
    use bevy::{
        prelude::{Entity, IntoFunction, World},
        reflect::{DynamicList, ParsedPath, ReflectFromReflect},
    };
    use test_utils::test_data::TestResourceWithVariousFields;

    use crate::prelude::AppReflectAllocator;

    use super::*;

    fn setup_world() -> World {
        test_utils::test_data::setup_world(|world, t| {
            world.insert_resource(AppReflectAllocator::default());
            t.register::<WorldCallbackAccess>()
        })
    }

    #[test]
    fn call_t_owned() {
        let mut world = setup_world();

        let world = WorldAccessGuard::new(&mut world);

        let function = (|usize: usize| usize).into_function();

        let allocator = world.allocator();
        let mut allocator = allocator.write();
        let arg_ref = ReflectReference::new_allocated(2usize, &mut allocator);
        drop(allocator);

        let arc_world = Arc::new(world);

        function
            .with_call(vec![ScriptValue::Reference(arg_ref)], arc_world, |r| {
                assert!(r.unwrap_owned().reflect_partial_eq(&2usize).unwrap());
            })
            .unwrap();
    }

    #[test]
    fn call_t_ref() {
        let mut world = setup_world();

        let world = WorldAccessGuard::new(&mut world);
        let world = WorldGuard::new(world);
        let function = (|usize: &usize| *usize).into_function();

        let mut arg_ref =
            ReflectReference::new_resource_ref::<TestResourceWithVariousFields>(world.clone())
                .unwrap();
        arg_ref.index_path(ParsedPath::parse_static("usize").unwrap());

        function
            .with_call(vec![ScriptValue::Reference(arg_ref)], world, |r| {
                assert!(r.unwrap_owned().reflect_partial_eq(&22usize).unwrap());
            })
            .unwrap();
    }

    #[test]
    fn call_t_mut() {
        let mut world = setup_world();

        let world = WorldAccessGuard::new(&mut world);
        let world = WorldGuard::new(world);

        let function = (|usize: &mut usize| {
            *usize = 42;
            *usize
        })
        .into_function();

        let mut arg_ref =
            ReflectReference::new_resource_ref::<TestResourceWithVariousFields>(world.clone())
                .unwrap();

        arg_ref.index_path(ParsedPath::parse_static("usize").unwrap());

        function
            .with_call(vec![ScriptValue::Reference(arg_ref)], world, |r| {
                assert!(r.unwrap_owned().reflect_partial_eq(&42usize).unwrap());
            })
            .unwrap();
    }

    #[test]
    fn call_dynamic_t_owned() {
        let mut world = setup_world();

        let world = WorldAccessGuard::new(&mut world);
        let world = WorldGuard::new(world);

        let type_registry = world.type_registry();
        let mut type_registry = type_registry.write();
        type_registry.register::<Vec<usize>>();
        type_registry.register_type_data::<Vec<usize>, ReflectFromReflect>();

        let function = (|arr: Vec<usize>| arr).into_function();

        let allocator = world.allocator();
        let mut allocator = allocator.write();

        let mut dynamic = DynamicList::from_iter(vec![2usize]);
        dynamic.set_represented_type(
            type_registry.get_type_info(std::any::TypeId::of::<Vec<usize>>()),
        );

        let arg_ref = ReflectReference::new_allocated(dynamic, &mut allocator);
        drop(type_registry);
        drop(allocator);

        function
            .with_call(vec![ScriptValue::Reference(arg_ref)], world, |r| {
                assert!(r.unwrap_owned().reflect_partial_eq(&vec![2usize]).unwrap());
            })
            .unwrap();
    }

    #[test]
    fn call_dynamic_t_ref() {
        let mut world = setup_world();

        let world = WorldAccessGuard::new(&mut world);
        let world = WorldGuard::new(world);

        let type_registry = world.type_registry();
        let mut type_registry = type_registry.write();
        type_registry.register::<Vec<usize>>();
        type_registry.register_type_data::<Vec<usize>, ReflectFromReflect>();

        let function = (|arr: &Vec<usize>| arr.clone()).into_function();

        let allocator = world.allocator();
        let mut allocator = allocator.write();

        let mut dynamic = DynamicList::from_iter(vec![2usize]);
        dynamic.set_represented_type(
            type_registry.get_type_info(std::any::TypeId::of::<Vec<usize>>()),
        );

        let arg_ref = ReflectReference::new_allocated(dynamic, &mut allocator);
        drop(type_registry);
        drop(allocator);

        function
            .with_call(vec![ScriptValue::Reference(arg_ref)], world, |r| {
                assert!(r.unwrap_owned().reflect_partial_eq(&vec![2usize]).unwrap());
            })
            .unwrap();
    }

    #[test]
    fn call_dynamic_t_mut() {
        let mut world = setup_world();

        let world = WorldAccessGuard::new(&mut world);
        let world = WorldGuard::new(world);

        let type_registry = world.type_registry();
        let mut type_registry = type_registry.write();
        type_registry.register::<Vec<usize>>();
        type_registry.register_type_data::<Vec<usize>, ReflectFromReflect>();

        let function = (|arr: &mut Vec<usize>| {
            arr.push(42);
            arr.clone()
        })
        .into_function();

        let allocator = world.allocator();
        let mut allocator = allocator.write();

        let mut dynamic = DynamicList::from_iter(vec![2usize]);
        dynamic.set_represented_type(
            type_registry.get_type_info(std::any::TypeId::of::<Vec<usize>>()),
        );

        let arg_ref = ReflectReference::new_allocated(dynamic, &mut allocator);
        drop(type_registry);
        drop(allocator);

        function
            .with_call(vec![ScriptValue::Reference(arg_ref)], world, |r| {
                assert!(r
                    .unwrap_owned()
                    .reflect_partial_eq(&vec![2usize, 42usize])
                    .unwrap());
            })
            .unwrap();
    }

    #[test]
    pub fn call_world_access_callback() {
        let mut world = setup_world();

        let world = WorldAccessGuard::new(&mut world);

        let function = (|world: WorldCallbackAccess| world.spawn()).into_function();

        let arc_world = Arc::new(world);
        let cloned_world = arc_world.clone();
        drop(arc_world);
        function
            .with_call(vec![ScriptValue::World], cloned_world, |r| {
                assert!(r
                    .unwrap_owned()
                    .reflect_partial_eq(&Entity::from_raw(5))
                    .unwrap());
            })
            .unwrap();
    }
}
