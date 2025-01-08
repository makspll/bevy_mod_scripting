use std::sync::Arc;

use crate::NamespaceBuilder;
use bevy::{
    prelude::{Entity, World},
    reflect::{Reflect, TypeRegistration},
};
use bevy_mod_scripting_core::{
    bindings::{
        access_map::ReflectAccessId,
        function::{
            script_function::{CallerContext, DynamicScriptFunctionMut},
            CallScriptFunction,
        },
        pretty_print::DisplayWithWorld,
        ReflectReference, ScriptTypeRegistration, WorldCallbackAccess,
    },
    error::InteropError,
};
use test_utils::test_data::EnumerateTestComponents;

pub fn register_test_functions(world: &mut World) {
    NamespaceBuilder::<World>::new_unregistered(world)
        .register("_get_mock_type", |s: WorldCallbackAccess| {
            let world = s.try_read().unwrap();
            #[derive(Reflect)]
            struct Dummy;
            let reg =
                ScriptTypeRegistration::new(Arc::new(TypeRegistration::of::<Dummy>()), None, None);
            let allocator = world.allocator();
            let mut allocator = allocator.write();
            ReflectReference::new_allocated(reg, &mut allocator)
        })
        .register(
            "_get_entity_with_test_component",
            |s: WorldCallbackAccess, name: String| {
                let world = s.try_read().unwrap();
                World::enumerate_test_components()
                    .iter()
                    .find(|(n, _, _)| n.contains(&name))
                    .map(|(_, _, c)| {
                        let allocator = world.allocator();
                        let mut allocator = allocator.write();

                        ReflectReference::new_allocated(
                            c.unwrap_or(Entity::from_raw(9999)),
                            &mut allocator,
                        )
                    })
            },
        )
        .register(
            "_assert_throws",
            |s: WorldCallbackAccess, mut f: DynamicScriptFunctionMut, reg: String| {
                let world = s.try_read().unwrap();

                let result =
                    f.call_script_function(vec![], world.clone(), CallerContext::default());
                let err = match result {
                    Ok(_) => {
                        return Err(InteropError::external_error(
                            "Expected function to throw error, but it did not.".into(),
                        ))
                    }
                    Err(e) => e.display_with_world(world.clone()),
                };

                let regex = regex::Regex::new(&reg).unwrap();
                if regex.is_match(&err) {
                    Ok(())
                } else {
                    Err(InteropError::external_error(
                        format!(
                            "Expected error message to match the regex: \n{}\n\nBut got:\n{}",
                            regex.as_str(),
                            err
                        )
                        .into(),
                    ))
                }
            },
        )
        .register(
            "_set_write_access",
            |s: WorldCallbackAccess, ref_: ReflectReference| {
                let world = s.try_read().unwrap();

                world
                    .claim_write_access(ReflectAccessId::for_reference(ref_.base.base_id).unwrap());
            },
        )
        .register(
            "_set_read_access",
            |s: WorldCallbackAccess, ref_: ReflectReference| {
                let world = s.try_read().unwrap();

                world.claim_read_access(ReflectAccessId::for_reference(ref_.base.base_id).unwrap());
            },
        )
        .register("_claim_global_access", |s: WorldCallbackAccess| {
            let world = s.try_read().unwrap();

            world.claim_global_access();
        });
}
