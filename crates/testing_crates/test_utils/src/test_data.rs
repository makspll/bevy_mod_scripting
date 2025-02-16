use std::alloc::Layout;

use bevy::asset::AssetPlugin;
use bevy::ecs::{component::*, world::World};
use bevy::prelude::*;
use bevy::reflect::*;

/// Test component with Reflect and ReflectComponent registered
#[derive(Component, Reflect, PartialEq, Eq, Debug)]
#[reflect(Component)]
pub struct TestComponent {
    pub strings: Vec<String>,
}

impl TestComponent {
    pub fn init() -> Self {
        Self {
            strings: vec!["Initial".to_string(), "Value".to_string()],
        }
    }
}

/// Test Resource with Reflect and ReflectResource registered
#[derive(Resource, Reflect, Default, PartialEq, Eq, Debug)]
#[reflect(Resource)]
pub struct TestResource {
    pub bytes: Vec<u8>,
}

impl TestResource {
    pub fn init() -> Self {
        Self {
            bytes: vec![0, 1, 2, 3, 4, 5],
        }
    }
}

/// Resource with Reflect and ReflectDefault registered but no ReflectResource
#[derive(Resource, Reflect, PartialEq, Eq, Debug)]
#[reflect(Default)]
pub struct ResourceWithDefault(pub String);

impl Default for ResourceWithDefault {
    fn default() -> Self {
        Self(String::from("Default"))
    }
}

impl ResourceWithDefault {
    pub fn init() -> Self {
        Self(String::from("Initial Value"))
    }
}

/// Component with Reflect and ReflectFromWorld registered but no ReflectComponent
#[derive(Reflect, Component, PartialEq, Debug)]
#[reflect(FromWorld)]
pub struct CompWithFromWorld(pub String);

impl Default for CompWithFromWorld {
    fn default() -> Self {
        Self(String::from("Default"))
    }
}

impl CompWithFromWorld {
    pub fn init() -> Self {
        Self(String::from("Initial Value"))
    }
}

/// Component with Reflect and ReflectDefault but no ReflectComponent
#[derive(Component, Reflect, PartialEq, Eq, Debug)]
#[reflect(Default)]
pub struct CompWithDefault(pub String);

impl CompWithDefault {
    pub fn init() -> Self {
        Self(String::from("Initial Value"))
    }
}

impl Default for CompWithDefault {
    fn default() -> Self {
        Self(String::from("Default"))
    }
}

#[derive(Component, Reflect, PartialEq, Eq, Debug)]
#[reflect(Component, Default)]
pub struct CompWithDefaultAndComponentData(pub String);
impl Default for CompWithDefaultAndComponentData {
    fn default() -> Self {
        Self(String::from("Default"))
    }
}

impl CompWithDefaultAndComponentData {
    pub fn init() -> Self {
        Self(String::from("Initial Value"))
    }
}

#[derive(Component, Reflect, PartialEq, Eq, Debug)]
#[reflect(Component, FromWorld)]
pub struct CompWithFromWorldAndComponentData(pub String);
impl Default for CompWithFromWorldAndComponentData {
    fn default() -> Self {
        Self(String::from("Default"))
    }
}

impl CompWithFromWorldAndComponentData {
    pub fn init() -> Self {
        Self(String::from("Initial Value"))
    }
}

#[derive(Resource, Reflect, PartialEq, Debug)]
pub struct TestResourceWithVariousFields {
    pub string: String,
    pub usize: usize,
    pub int: i32,
    pub float: f32,
    pub bool: bool,
    pub vec_usize: Vec<usize>,
}

impl TestResourceWithVariousFields {
    pub fn init() -> Self {
        Self {
            string: "Initial Value".to_string(),
            usize: 22,
            int: 42,
            float: 69.0,
            bool: true,
            vec_usize: vec![1, 2, 3, 4, 5],
        }
    }
}

pub(crate) const TEST_COMPONENT_ID_START: usize = 20;
pub(crate) const TEST_ENTITY_ID_START: u32 = 0;

pub trait GetTestComponentId {
    fn test_component_id() -> ComponentId;
}

pub trait GetTestEntityId {
    fn test_entity_id() -> Entity;
}

pub trait EnumerateTestComponents {
    fn enumerate_test_components() -> Vec<(&'static str, ComponentId, Option<Entity>)>;
}

macro_rules! impl_test_component_ids {
    ([$($comp_type:ty => $comp_id:expr),* $(,)?], [$($res_type:ty => $res_id:expr),* $(,)?]) => {
        $(
            impl GetTestComponentId for $comp_type {
                fn test_component_id() -> ComponentId {
                    ComponentId::new(TEST_COMPONENT_ID_START + $comp_id)
                }
            }

            impl GetTestEntityId for $comp_type {
                fn test_entity_id() -> Entity {
                    Entity::from_raw(TEST_ENTITY_ID_START + $comp_id)
                }
            }
        )*
        $(
            impl GetTestComponentId for $res_type {
                fn test_component_id() -> ComponentId {
                    ComponentId::new(TEST_COMPONENT_ID_START + $res_id)
                }
            }
        )*

        pub(crate) fn init_all_components(world: &mut World, registry: &mut TypeRegistry) {
            $(
                world.register_component::<$comp_type>();
                registry.register::<$comp_type>();
                let registered_id = world.component_id::<$comp_type>().unwrap().index();
                assert_eq!(registered_id, TEST_COMPONENT_ID_START + $comp_id, "Test setup failed. Did you register components before running setup_world?");
                let entity = world.spawn(<$comp_type>::init()).id();
                assert_eq!(entity.index(), TEST_ENTITY_ID_START + $comp_id, "Test setup failed. Did you spawn entities before running setup_world?");
                assert_eq!(entity.generation(), 1, "Test setup failed. Did you spawn entities before running setup_world?");
            )*
            $(
                world.insert_resource::<$res_type>(<$res_type>::init());
                registry.register::<$res_type>();
                let registered_id = world.resource_id::<$res_type>().unwrap().index();
                assert_eq!(registered_id, TEST_COMPONENT_ID_START + $res_id, "Test setup failed. Did you register components before running setup_world?");
            )*
        }

        impl EnumerateTestComponents for World {
            fn enumerate_test_components() -> Vec<(&'static str, ComponentId, Option<Entity>)> {
                vec![
                    $(
                        (std::any::type_name::<$comp_type>(), <$comp_type as GetTestComponentId>::test_component_id(), Some(<$comp_type as GetTestEntityId>::test_entity_id()))
                    ),*
                    $(
                        ,(std::any::type_name::<$res_type>(), <$res_type as GetTestComponentId>::test_component_id(), None)
                    )*

                ]
            }
        }
    };
}

impl_test_component_ids!(
    [   TestComponent => 0,
        CompWithFromWorld => 1,
        CompWithDefault => 2,
        CompWithDefaultAndComponentData => 3,
        CompWithFromWorldAndComponentData => 4
    ],
    [
        TestResource => 5,
        ResourceWithDefault => 6,
        TestResourceWithVariousFields => 7,
    ]
);

fn init_world<F: FnOnce(&mut World, &mut TypeRegistry)>(world: &mut World, init: F) {
    let type_registry = world.get_resource_or_init::<AppTypeRegistry>().clone();
    let mut type_registry_guard = type_registry.0.write();

    while world.components().len() < TEST_COMPONENT_ID_START {
        unsafe {
            world.register_component_with_descriptor(ComponentDescriptor::new_with_layout(
                format!("Filler{}", world.components().len()),
                StorageType::Table,
                Layout::new::<usize>(),
                None,
            ))
        };
    }

    init_all_components(world, &mut type_registry_guard);
    init(world, &mut type_registry_guard);
}

fn setup_app<F: FnOnce(&mut World, &mut TypeRegistry)>(init: F) -> App {
    let mut app = App::new();
    let world = app.world_mut();

    init_world(world, init);
    app
}

/// Initializes a default world with a set of test components and resources with various properties and implemantations.
pub fn setup_world<F: FnOnce(&mut World, &mut TypeRegistry)>(init: F) -> World {
    let mut world = World::default();
    init_world(&mut world, init);
    world
}

/// Initializes a headless app with the standard testing setup and the given plugin.
pub fn setup_integration_test<F: FnOnce(&mut World, &mut TypeRegistry)>(init: F) -> App {
    // first setup all normal test components and resources
    let mut app = setup_app(init);

    app.add_plugins((MinimalPlugins, AssetPlugin::default(), HierarchyPlugin));
    app
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn setup_works() {
        setup_world(|_, _| {});
    }
}
