use bevy_ecs::world::World;
use bevy_mod_scripting_derive::{
    ArgMeta, FromScript, GetTypeDependencies, IntoScript, TypedThrough, script_bindings,
};
use bevy_reflect::{Reflect, TypeRegistry, Typed};

#[derive(Clone, TypedThrough, GetTypeDependencies, ArgMeta, IntoScript, FromScript, Reflect)]
// this is not required in third party crates, unless they only depend on these directly
#[typed_through(bms_bindings_path = "bevy_mod_scripting_bindings")]
#[get_type_dependencies(bms_bindings_path = "bevy_mod_scripting_bindings")]
#[into_script(bms_bindings_path = "bevy_mod_scripting_bindings")]
#[from_script(bms_bindings_path = "bevy_mod_scripting_bindings")]
#[arg_meta(bms_bindings_path = "bevy_mod_scripting_bindings")]
pub struct MyThing(usize);

#[script_bindings(remote, bms_bindings_path = "bevy_mod_scripting_bindings")]
impl MyThing {
    pub fn test(thing: MyThing) -> MyThing {
        thing
    }
}
mod tests {
    use std::any::TypeId;

    use bevy_ecs::world::World;
    use bevy_mod_scripting_bindings::{
        AppScriptFunctionRegistry, FunctionCallContext, GetTypeDependencies, ReflectReference,
        ThroughTypeInfo, TypedThrough, WorldExtensions,
    };
    use bevy_mod_scripting_world::WorldAccessGuard;
    use bevy_reflect::TypeRegistry;

    use crate::derive_tests::{MyThing, register_functions};

    #[test]
    pub fn typed_through_represents_self() {
        let info = MyThing::through_type_info();
        assert!(
            matches!(info, ThroughTypeInfo::TypeInfo(info) if info.type_id() == TypeId::of::<MyThing>())
        );
    }

    #[test]
    pub fn type_dependency_registers_only_self() {
        let mut registry = TypeRegistry::new();
        let prev_count = registry.iter().count();
        <MyThing as GetTypeDependencies>::register_type_dependencies(&mut registry);
        assert!(registry.contains(TypeId::of::<MyThing>()));
        let next_count = registry.iter().count();
        assert_eq!(next_count - prev_count, 1)
    }

    #[test]
    pub fn bindings_function_works() {
        let mut world = World::new();

        world.init_resource::<AppScriptFunctionRegistry>();

        let cache = WorldAccessGuard::setup_cache(&world, Default::default());
        WorldAccessGuard::with_static_guard(&mut world, cache, |world| {
            let world = &world;
            world
                .with_world_mut_access(|w| {
                    register_functions(w);
                })
                .unwrap();
            let func = world
                .lookup_function([TypeId::of::<MyThing>()], "test")
                .unwrap();
            let allocator = world.allocator();
            let mut allocator = allocator.write();
            let allocated = ReflectReference::new_allocated(MyThing(42), &mut allocator);
            drop(allocator);
            let out = func
                .call(
                    vec![allocated.into()],
                    FunctionCallContext::new(bevy_mod_scripting_asset::Language::Lua),
                )
                .unwrap();
            let out: MyThing = match out {
                bevy_mod_scripting_bindings::ScriptValue::Reference(reflect_reference) => {
                    reflect_reference.downcast(world.clone()).unwrap()
                }
                _ => panic!("invalid return"),
            };
            assert_eq!(out.0, 42)
        });
    }
}
