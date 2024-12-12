use std::borrow::Cow;

use bevy::reflect::func::{
    args::{Arg, ArgInfo, Ownership},
    ArgList, ArgValue, DynamicFunction, FunctionResult, Return,
};

use crate::error::{ScriptError, ScriptResult};

use super::{
    access_map::ReflectAccessId, pretty_print::DisplayWithWorld, ReflectReference, WorldAccessGuard,
};

/// Can be implemented for callables which require dynamic access to the world to be called.
///
/// The claim and release functions must be used to scope the access to the world such that function output .
pub trait CallableWithAccess {
    fn with_call<O, F: FnOnce(Return) -> O>(
        &self,
        args: &[ReflectReference],
        world: &WorldAccessGuard,
        f: F,
    ) -> ScriptResult<O>;
}

impl<'env> CallableWithAccess for DynamicFunction<'env> {
    fn with_call<O, F: FnOnce(Return) -> O>(
        &self,
        args: &[ReflectReference],
        world: &WorldAccessGuard,
        f: F,
    ) -> ScriptResult<O> {
        let info = self.info().args();

        if info.len() != args.len() {
            return Err(ScriptError::new_reflection_error(format!(
                "Expected {} arguments, got {}",
                info.len(),
                args.len()
            )));
        }

        // We need to:
        // 1. Claim the correct access for each argument
        // 2. Convert to ArgsList for the function call
        // 3. Call the function
        // 4. Relinquish access to the caller for the return value
        // 5. Release the access for each argument
        // 6. Return the result
        let (args_list, mut accesses) = args.into_args_list_with_access(info, world)?;

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
        world: &'w WorldAccessGuard,
    ) -> ScriptResult<(ArgList<'w>, Vec<(ReflectAccessId, Ownership)>)>;
}

impl IntoArgsListWithAccess for &[ReflectReference] {
    /// Converts a list of references into an `ArgList` and collects the access id's and types for each argument.
    ///
    /// This is actually safe to call, since we are not actually releasing the access here.
    /// Meaning that only after releasing access is it possible to create unsafe aliases
    fn into_args_list_with_access<'w>(
        self,
        arg_info: &[ArgInfo],
        world: &'w WorldAccessGuard,
    ) -> ScriptResult<(ArgList<'w>, Vec<(ReflectAccessId, Ownership)>)> {
        if self.len() != arg_info.len() {
            return Err(ScriptError::new_reflection_error(format!(
                "Expected {} arguments, got {}",
                arg_info.len(),
                self.len()
            )));
        }

        let mut accesses = Vec::default();

        let _ = self
            .iter()
            .zip(arg_info.iter())
            .map(|(arg_ref, arg_info)| {
                let access_id =
                    ReflectAccessId::for_reference(arg_ref.base.base_id.clone()).ok_or_else(|| {
                        ScriptError::new_reflection_error(format!(
                            "Could not call function, argument {} is not a valid reference. Have you registered the type?",
                            arg_ref.display_with_world(world)
                        ))
                    })?;

                let is_write = matches!(arg_info.ownership(), Ownership::Mut);

                let success = if is_write {
                    world.claim_write_access(access_id)
                } else {
                    world.claim_read_access(access_id)
                };

                if !success {
                    return Err(ScriptError::new_reflection_error(format!(
                        "Could not claim access for argument {}",
                        arg_ref.display_with_world(world)
                    )));
                }


                accesses.push((access_id, arg_info.ownership()));

                Ok(())
            })
            .collect::<ScriptResult<Vec<_>>>()
            .inspect_err(|_| {
                // we don't want to leave the world in an inconsistent state
                // Safety: we have not generated any unsafe aliases
                // - we are releassing only the access we have claimed
                accesses.iter().for_each(|(id,_)| {
                    unsafe {world.release_access(*id)};
                });
            })?;

        let mut arg_list = ArgList::new();

        for (r, info) in self.iter().zip(arg_info.iter()) {
            // Safety: we have claimed access for each argument
            let val = unsafe { r.clone().into_arg_value(world, info) };
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

        Ok((arg_list, accesses))
    }
}

#[cfg(test)]
mod test {
    use bevy::{
        prelude::{IntoFunction, World},
        reflect::{DynamicList, ParsedPath, ReflectFromReflect},
    };
    use test_utils::test_data::TestResourceWithVariousFields;

    use crate::prelude::AppReflectAllocator;

    use super::*;

    fn setup_world() -> World {
        test_utils::test_data::setup_world(|world, _| {
            world.insert_resource(AppReflectAllocator::default());
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

        function
            .with_call(&[arg_ref], &world, |r| {
                assert!(r.unwrap_owned().reflect_partial_eq(&2usize).unwrap());
            })
            .unwrap();
    }

    #[test]
    fn call_t_ref() {
        let mut world = setup_world();

        let world = WorldAccessGuard::new(&mut world);

        let function = (|usize: &usize| *usize).into_function();

        let mut arg_ref =
            ReflectReference::new_resource_ref::<TestResourceWithVariousFields>(&world).unwrap();
        arg_ref.index_path(ParsedPath::parse_static("usize").unwrap());

        function
            .with_call(&[arg_ref], &world, |r| {
                assert!(r.unwrap_owned().reflect_partial_eq(&22usize).unwrap());
            })
            .unwrap();
    }

    #[test]
    fn call_t_mut() {
        let mut world = setup_world();

        let world = WorldAccessGuard::new(&mut world);

        let function = (|usize: &mut usize| {
            *usize = 42;
            *usize
        })
        .into_function();

        let mut arg_ref =
            ReflectReference::new_resource_ref::<TestResourceWithVariousFields>(&world).unwrap();

        arg_ref.index_path(ParsedPath::parse_static("usize").unwrap());

        function
            .with_call(&[arg_ref], &world, |r| {
                assert!(r.unwrap_owned().reflect_partial_eq(&42usize).unwrap());
            })
            .unwrap();
    }

    #[test]
    fn call_dynamic_t_owned() {
        let mut world = setup_world();

        let world = WorldAccessGuard::new(&mut world);

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
            .with_call(&[arg_ref], &world, |r| {
                assert!(r.unwrap_owned().reflect_partial_eq(&vec![2usize]).unwrap());
            })
            .unwrap();
    }

    #[test]
    fn call_dynamic_t_ref() {
        let mut world = setup_world();

        let world = WorldAccessGuard::new(&mut world);

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
            .with_call(&[arg_ref], &world, |r| {
                assert!(r.unwrap_owned().reflect_partial_eq(&vec![2usize]).unwrap());
            })
            .unwrap();
    }

    #[test]
    fn call_dynamic_t_mut() {
        let mut world = setup_world();

        let world = WorldAccessGuard::new(&mut world);

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
            .with_call(&[arg_ref], &world, |r| {
                assert!(r
                    .unwrap_owned()
                    .reflect_partial_eq(&vec![2usize, 42usize])
                    .unwrap());
            })
            .unwrap();
    }
}
