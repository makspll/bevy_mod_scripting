use std::{collections::HashMap, sync::Arc};

use bevy::{
    app::App,
    ecs::component::ComponentId,
    prelude::{Entity, World},
    reflect::{Reflect, TypeRegistration},
};
use bevy_mod_scripting_core::{
    asset::Language,
    bindings::{
        function::{
            namespace::{GlobalNamespace, NamespaceBuilder},
            script_function::{DynamicScriptFunctionMut, FunctionCallContext},
        },
        pretty_print::DisplayWithWorld,
        ReflectReference, ScriptComponentRegistration, ScriptResourceRegistration,
        ScriptTypeRegistration,
    },
    error::InteropError,
};
use test_utils::test_data::EnumerateTestComponents;

pub fn register_test_functions(world: &mut App) {
    let world = world.world_mut();
    NamespaceBuilder::<World>::new_unregistered(world)
        .register("_get_mock_type", |s: FunctionCallContext| {
            let world = s.world().unwrap();
            #[derive(Reflect)]
            struct Dummy;
            let reg = ScriptTypeRegistration::new(Arc::new(TypeRegistration::of::<Dummy>()));
            let allocator = world.allocator();
            let mut allocator = allocator.write();
            ReflectReference::new_allocated(reg, &mut allocator)
        })
        .register("_get_mock_component_type", |s: FunctionCallContext| {
            let world = s.world().unwrap();
            #[derive(Reflect)]
            struct Dummy;
            let reg = ScriptTypeRegistration::new(Arc::new(TypeRegistration::of::<Dummy>()));
            let comp = ScriptComponentRegistration::new(reg, ComponentId::new(999999999999999));
            let allocator = world.allocator();
            let mut allocator = allocator.write();
            ReflectReference::new_allocated(comp, &mut allocator)
        })
        .register("_get_mock_resource_type", |s: FunctionCallContext| {
            let world = s.world().unwrap();
            #[derive(Reflect)]
            struct Dummy;
            let reg = ScriptTypeRegistration::new(Arc::new(TypeRegistration::of::<Dummy>()));
            let comp = ScriptResourceRegistration::new(reg, ComponentId::new(999999999999999));
            let allocator = world.allocator();
            let mut allocator = allocator.write();
            ReflectReference::new_allocated(comp, &mut allocator)
        })
        .register("_sleep", |time: f64| {
            std::thread::sleep(std::time::Duration::from_secs_f64(time));
            Ok(())
        })
        .register(
            "_get_entity_with_test_component",
            |s: FunctionCallContext, name: String| {
                let world = s.world().unwrap();
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
            |s: FunctionCallContext, f: DynamicScriptFunctionMut, reg: String| {
                let world = s.world().unwrap();

                let result = f.call(vec![], FunctionCallContext::new(Language::Unknown));
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
        );

    NamespaceBuilder::<GlobalNamespace>::new_unregistered(world)
        .register("global_hello_world", || Ok("hi!"))
        .register("make_hashmap", |map: HashMap<String, usize>| map)
        .register(
            "assert_str_eq",
            |s1: String, s2: String, reason: Option<String>| {
                pretty_assertions::assert_eq!(
                    s1.trim(),
                    s2.trim(),
                    "Reason Provided: {}",
                    reason.unwrap_or_default()
                )
            },
        );
}
