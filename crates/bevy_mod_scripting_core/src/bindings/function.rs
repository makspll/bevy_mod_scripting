use std::{borrow::Cow, sync::Arc};

use bevy::reflect::func::{
    args::{Arg, ArgInfo, Ownership},
    ArgList, ArgValue, DynamicFunction, FunctionResult, Return,
};

use crate::error::{ScriptError, ScriptResult};

use super::{
    access_map::ReflectAccessId, pretty_print::DisplayWithWorld, script_val::ScriptValue,
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
}

impl CallableWithAccess for DynamicFunction<'_> {
    fn with_call<O, F: FnOnce(Return) -> O, I: IntoIterator<Item = ScriptValue>>(
        &self,
        args: I,
        world: Arc<WorldAccessGuard>,
        f: F,
    ) -> ScriptResult<O> {
        let info = self.info().args();

        // if info.len() != args.len() {
        //     return Err(ScriptError::new_reflection_error(format!(
        //         "Expected {} arguments, got {}",
        //         info.len(),
        //         args.len()
        //     )));
        // }

        // We need to:
        // 1. Claim the correct access for each argument
        // 2. Convert to ArgsList for the function call
        // 3. Call the function
        // 4. Relinquish access to the caller for the return value
        // 5. Release the access for each argument
        // 6. Return the result
        let arg_iter = args.into_iter();

        let (args_list, mut accesses) = arg_iter.into_args_list_with_access(info, world.clone())?;
        // let arc_world = &arc_world;

        let return_val = match self.call(args_list) {
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
    ) -> ScriptResult<(ArgList<'w>, Vec<(ReflectAccessId, Ownership)>)>;
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
    ) -> ScriptResult<(ArgList<'w>, Vec<(ReflectAccessId, Ownership)>)> {
        // if self.len() != arg_info.len() {
        //     return Err(ScriptError::new_reflection_error(format!(
        //         "Expected {} arguments, got {}",
        //         arg_info.len(),
        //         self.len()
        //     )));
        // }

        let mut accesses = Vec::default();
        let mut arg_list = ArgList::new();

        for (value, arg_info) in self.zip(arg_info.iter()) {
            match value {
                ScriptValue::Reference(arg_ref) => {
                    let access_id =
                    ReflectAccessId::for_reference(arg_ref.base.base_id.clone()).ok_or_else(|| {
                        ScriptError::new_reflection_error(format!(
                            "Could not call function, argument: {:?}, with type: {} is not a valid reference. Have you registered the type?",
                            arg_info.name(),
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
                        accesses.drain(..).for_each(|(id, _)| {
                            // Safety: we have not generated any unsafe aliases
                            unsafe { world.release_access(id) };
                        });
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
                            // Safety: Same as above

                            accesses.iter().for_each(|(id, _)| {
                                unsafe { world.release_access(*id) };
                            });
                            return Err(e);
                        }
                    };
                    arg_list = arg_list.push_arg(val);
                }
                ScriptValue::World => {
                    arg_list = arg_list.push_arg(ArgValue::Owned(Box::new(
                        WorldCallbackAccess::from_guard(world.clone()),
                    )));
                }
                v => todo!(),
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
