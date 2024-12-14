use std::{borrow::Cow, ops::Deref, sync::Arc};

use bevy::reflect::{
    func::{
        args::{Arg, ArgInfo, Ownership},
        ArgList, ArgValue, DynamicFunction, FunctionResult, Return,
    },
    PartialReflect,
};

use crate::error::{ScriptError, ScriptResult, ValueConversionError};

use super::{
    access_map::ReflectAccessId,
    pretty_print::DisplayWithWorld,
    script_val::{FromScriptValue, IntoScriptValue, ScriptValue},
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
    ) -> ScriptResult<O>;

    fn dynamic_call<I: IntoIterator<Item = ScriptValue>>(
        &self,
        args: I,
        world: Arc<WorldAccessGuard>,
    ) -> ScriptResult<ScriptValue> {
        self.with_call(args, world.clone(), |r| match r {
            Return::Owned(partial_reflect) => partial_reflect.as_ref().into_script_value(world),
            Return::Ref(ref_) => ref_.into_script_value(world),
            Return::Mut(mut_ref) => mut_ref.into_script_value(world),
        })?
    }
}

impl CallableWithAccess for DynamicFunction<'_> {
    fn with_call<O, F: FnOnce(Return) -> O, I: IntoIterator<Item = ScriptValue>>(
        &self,
        args: I,
        world: Arc<WorldAccessGuard>,
        f: F,
    ) -> ScriptResult<O> {
        let info = self.info().args();

        // We need to:
        // 1. Claim the correct access for each argument
        // 2. Convert to ArgsList for the function call
        // 3. Call the function
        // 4. Relinquish access to the caller for the return value
        // 5. Release the access for each argument
        // 6. Return the result
        let arg_iter = args.into_iter();

        let (mut args_list, mut accesses) =
            arg_iter.into_args_list_with_access(info, world.clone())?;

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

        let return_val = match self.call(final_args_list) {
            Ok(return_val) => return_val,
            Err(e) => {
                // Safety: we have not generated any unsafe aliases
                // - we are releasing only the access we have claimed
                accesses.drain(..).for_each(|(id, _)| {
                    unsafe { world.release_access(id) };
                });

                return Err(e.into());
            }
        };

        let out = f(return_val);
        // Safety: we have not generated any unsafe aliases
        // - we are releasing only the access we have claimed
        accesses.drain(..).for_each(|(id, _)| {
            unsafe { world.release_access(id) };
        });

        Ok(out)
    }
}

/// Trait implementable by lists of things representing arguments which can be converted into an `ArgList`.
///
/// The call needs to collect the correct access id's and types for each argument.
pub trait IntoArgsListWithAccess {
    fn into_args_list_with_access<'w>(
        self,
        arg_info: &[ArgInfo],
        world: WorldGuard<'w>,
    ) -> ScriptResult<(Vec<ArgValue<'w>>, Vec<(ReflectAccessId, Ownership)>)>;
}

impl<I: Iterator<Item = ScriptValue>> IntoArgsListWithAccess for I {
    /// Converts a list of references into an `ArgList` and collects the access id's and types for each argument.
    ///
    /// This is actually safe to call, since we are not actually releasing the access here.
    /// Meaning that only after releasing access is it possible to create unsafe aliases
    fn into_args_list_with_access<'w>(
        self,
        arg_info: &[ArgInfo],
        world: WorldGuard<'w>,
    ) -> ScriptResult<(Vec<ArgValue<'w>>, Vec<(ReflectAccessId, Ownership)>)> {
        let mut accesses = Vec::default();
        let mut arg_list = Vec::default();

        let release_accesses = |accesses: &mut Vec<(ReflectAccessId, Ownership)>| {
            accesses.iter().for_each(|(id, _)| {
                // Safety: we have not generated any unsafe aliases
                unsafe { world.release_access(*id) };
            });
        };

        for (value, arg_info) in self.zip(arg_info.iter()) {
            match value {
                ScriptValue::Reference(arg_ref) => {
                    let access_id =
                    ReflectAccessId::for_reference(arg_ref.base.base_id.clone()).ok_or_else(|| {
                        ScriptError::new_reflection_error(format!(
                            "Could not call function, argument: {:?}, with type: {} is not a valid reference. Have you registered the type?",
                            arg_info.name().map(str::to_owned).unwrap_or_else(|| arg_info.index().to_string()),
                            arg_ref.display_with_world(world.clone())
                        ))
                    })?;

                    let is_write = matches!(arg_info.ownership(), Ownership::Mut);

                    let success = if is_write {
                        world.claim_write_access(access_id)
                    } else {
                        world.claim_read_access(access_id)
                    };

                    if !success {
                        release_accesses(&mut accesses);
                        return Err(ScriptError::new_reflection_error(format!(
                            "Could not claim access for argument {}",
                            arg_ref.display_with_world(world.clone())
                        )));
                    }

                    accesses.push((access_id, arg_info.ownership()));

                    let val = unsafe { arg_ref.clone().into_arg_value(world.clone(), arg_info) };
                    let val = match val {
                        Ok(v) => v,
                        Err(e) => {
                            release_accesses(&mut accesses);
                            return Err(e);
                        }
                    };
                    arg_list.push(val);
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
                        arg_info.type_id(),
                    ) {
                        Some(Ok(v)) => v,
                        Some(Err(e)) => {
                            // Safety: Same as above
                            accesses.iter().for_each(|(id, _)| {
                                unsafe { world.release_access(*id) };
                            });
                            return Err(e);
                        }
                        None => {
                            release_accesses(&mut accesses);
                            return Err(ValueConversionError::TypeMismatch {
                                expected_type: arg_info.type_path().into(),
                                actual_type: None,
                            }
                            .into());
                        }
                    };

                    arg_list.push(ArgValue::Owned(value));
                }
            }
        }

        Ok((arg_list, accesses))
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
