use crate::WorldAccessGuard;

use super::{
    AppReflectAllocator, AppScriptComponentRegistry, ReflectBase, ReflectBaseType,
    ReflectReference, ScriptComponentRegistration, ScriptResourceRegistration,
    ScriptTypeRegistration, Union,
    function::{
        namespace::Namespace,
        script_function::{AppScriptFunctionRegistry, DynamicScriptFunction, FunctionCallContext},
    },
    schedule::AppScheduleRegistry,
    script_value::ScriptValue,
};
use crate::{
    error::InteropError,
    function::{from::FromScript, from_ref::FromScriptRef},
    reflection_extensions::PartialReflectExt,
};
use ::{
    bevy_asset::{AssetServer, Handle, LoadState},
    bevy_ecs::{
        component::ComponentId,
        entity::Entity,
        reflect::{ReflectFromWorld, ReflectResource},
        world::World,
    },
    bevy_reflect::{
        DynamicEnum, DynamicStruct, DynamicTuple, DynamicTupleStruct, DynamicVariant,
        PartialReflect, TypeRegistryArc, std_traits::ReflectDefault,
    },
};
use bevy_asset::AssetPath;
use bevy_mod_scripting_asset::ScriptAsset;
use bevy_platform::collections::HashMap;
use bevy_reflect::{TypeInfo, VariantInfo};
use bevy_system_reflection::ReflectSchedule;
use std::{any::TypeId, borrow::Cow, sync::Arc};

impl GetTypeInfo for ThreadWorldContainer {
    fn get_type_info(&self, type_id: TypeId) -> Option<&TypeInfo> {
        let world = self.try_get_context().ok()?.world;
        let registry = world.type_registry();
        let registry = registry.read();
        registry.get(type_id).map(|r| r.type_info())
    }

    fn query_type_registration(
        &self,
        type_id: TypeId,
        type_data_id: TypeId,
    ) -> Option<Box<dyn bevy_reflect::TypeData>> {
        let world = self.try_get_context().ok()?.world;
        let registry = world.type_registry();
        let registry = registry.read();
        registry
            .get(type_id)
            .and_then(|r| r.data_by_id(type_data_id).map(|t| t.clone_type_data()))
    }

    fn get_component_info(
        &self,
        component_id: ComponentId,
    ) -> Option<&bevy_ecs::component::ComponentInfo> {
        let world = self.try_get_context().ok()?.world;
        let cell = world.as_unsafe_world_cell().ok()?;
        cell.components().get_info(component_id)
    }

    unsafe fn as_any_static(&self) -> &dyn Any {
        self
    }
}

impl GetTypeInfo for WorldGuard<'_> {
    fn get_type_info(&self, type_id: TypeId) -> Option<&TypeInfo> {
        let registry = self.type_registry();
        let registry = registry.read();
        registry.get(type_id).map(|r| r.type_info())
    }

    fn query_type_registration(
        &self,
        type_id: TypeId,
        type_data_id: TypeId,
    ) -> Option<Box<dyn bevy_reflect::TypeData>> {
        let registry = self.type_registry();
        let registry = registry.read();
        registry
            .get(type_id)
            .and_then(|r| r.data_by_id(type_data_id).map(|t| t.clone_type_data()))
    }

    fn get_component_info(
        &self,
        component_id: ComponentId,
    ) -> Option<&bevy_ecs::component::ComponentInfo> {
        let cell = self.as_unsafe_world_cell().ok()?;
        cell.components().get_info(component_id)
    }

    /// # Safety
    /// - TODO: should generaly be safe as the guard is invalidated once the world is out of scope
    unsafe fn as_any_static(&self) -> &dyn Any {
        let static_self: &WorldGuard<'static> = unsafe { std::mem::transmute(self) };
        static_self as &dyn Any
    }
}

pub trait WorldExtensions {
    fn remove_resource(&self, registration: ScriptResourceRegistration)
    -> Result<(), InteropError>;
    fn get_resource(
        &self,
        resource_id: ComponentId,
    ) -> Result<Option<ReflectReference>, InteropError>;
    fn remove_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
    ) -> Result<(), InteropError>;
    fn has_component(
        &self,
        entity: Entity,
        component_id: ComponentId,
    ) -> Result<bool, InteropError>;
    fn get_component(
        &self,
        entity: Entity,
        component_registration: ScriptComponentRegistration,
    ) -> Result<Option<ReflectReference>, InteropError>;
    fn insert_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
        value: ReflectReference,
    ) -> Result<(), InteropError>;
    fn add_default_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
    ) -> Result<(), InteropError>;
    fn get_resource_type(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<Result<ScriptResourceRegistration, ScriptTypeRegistration>, InteropError>;
    fn get_component_type(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<Result<ScriptComponentRegistration, ScriptTypeRegistration>, InteropError>;
    fn get_schedule_by_name(&self, schedule_name: String) -> Option<ReflectSchedule>;
    fn get_type_registration_by_name(
        &self,
        type_name: String,
    ) -> Result<
        Option<
            Union<
                ScriptTypeRegistration,
                Union<ScriptComponentRegistration, ScriptResourceRegistration>,
            >,
        >,
        InteropError,
    >;

    fn get_type_registration(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<
        Union<
            ScriptTypeRegistration,
            Union<ScriptComponentRegistration, ScriptResourceRegistration>,
        >,
        InteropError,
    >;
    fn get_type_by_name(&self, type_name: &str) -> Option<ScriptTypeRegistration>;

    fn get_script_asset_load_state(
        &self,
        script: Handle<ScriptAsset>,
    ) -> Result<LoadState, InteropError>;

    fn load_script_asset<'a>(
        &self,
        asset_path: impl Into<AssetPath<'a>>,
    ) -> Result<Handle<ScriptAsset>, InteropError>;

    fn construct(
        &self,
        type_: ScriptTypeRegistration,
        payload: HashMap<String, ScriptValue>,
        one_indexed: bool,
    ) -> Result<Box<dyn PartialReflect>, InteropError>;

    fn construct_dynamic_tuple(
        &self,
        payload: &mut HashMap<String, ScriptValue>,
        fields: Vec<TypeId>,
        one_indexed: bool,
    ) -> Result<DynamicTuple, InteropError>;
    fn construct_dynamic_tuple_struct(
        &self,
        payload: &mut HashMap<String, ScriptValue>,
        fields: Vec<TypeId>,
        one_indexed: bool,
    ) -> Result<DynamicTupleStruct, InteropError>;
    fn construct_dynamic_struct(
        &self,
        payload: &mut HashMap<String, ScriptValue>,
        fields: Vec<(&'static str, TypeId)>,
    ) -> Result<DynamicStruct, InteropError>;
    fn construct_from_script_value(
        &self,
        descriptor: impl Into<Cow<'static, str>>,
        type_id: TypeId,
        value: Option<ScriptValue>,
    ) -> Result<Box<dyn PartialReflect>, InteropError>;
    fn try_call_overloads(
        &self,
        type_id: TypeId,
        name: impl Into<Cow<'static, str>>,
        args: Vec<ScriptValue>,
        context: FunctionCallContext,
    ) -> Result<ScriptValue, InteropError>;
    fn get_functions_on_type(
        &self,
        type_id: TypeId,
    ) -> Vec<(Cow<'static, str>, DynamicScriptFunction)>;
    fn lookup_function(
        &self,
        type_ids: impl IntoIterator<Item = TypeId>,
        name: impl Into<Cow<'static, str>>,
    ) -> Result<DynamicScriptFunction, Cow<'static, str>>;
    fn script_function_registry(&self) -> AppScriptFunctionRegistry;
    fn allocator(&self) -> AppReflectAllocator;
    fn component_registry(&self) -> AppScriptComponentRegistry;
    fn schedule_registry(&self) -> AppScheduleRegistry;
    fn type_registry(&self) -> TypeRegistryArc;
}

impl<'w> WorldExtensions for WorldAccessGuard<'w> {
    /// Returns the type registry for the world
    fn type_registry(&self) -> TypeRegistryArc {
        self.inner.type_registry.clone()
    }

    /// Returns the schedule registry for the world
    fn schedule_registry(&self) -> AppScheduleRegistry {
        self.inner.schedule_registry.clone()
    }

    /// Returns the component registry for the world
    fn component_registry(&self) -> AppScriptComponentRegistry {
        self.inner.script_component_registry.clone()
    }

    /// Returns the script allocator for the world
    fn allocator(&self) -> AppReflectAllocator {
        self.inner.allocator.clone()
    }

    /// Returns the function registry for the world
    fn script_function_registry(&self) -> AppScriptFunctionRegistry {
        self.inner.function_registry.clone()
    }

    /// Try to lookup a function with the given name on the given type id's namespaces.
    ///
    /// Returns the function if found, otherwise returns the name of the function that was not found.
    fn lookup_function(
        &self,
        type_ids: impl IntoIterator<Item = TypeId>,
        name: impl Into<Cow<'static, str>>,
    ) -> Result<DynamicScriptFunction, Cow<'static, str>> {
        let registry = self.script_function_registry();
        let registry = registry.read();

        let mut name = name.into();
        for type_id in type_ids {
            name = match registry.get_function(Namespace::OnType(type_id), name) {
                Ok(func) => return Ok(func.clone()),
                Err(name) => name,
            };
        }

        Err(name)
    }

    /// Iterates over all available functions on the type id's namespace + those available on any reference if any exist.
    fn get_functions_on_type(
        &self,
        type_id: TypeId,
    ) -> Vec<(Cow<'static, str>, DynamicScriptFunction)> {
        let registry = self.script_function_registry();
        let registry = registry.read();

        registry
            .iter_namespace(Namespace::OnType(type_id))
            .chain(
                registry
                    .iter_namespace(Namespace::OnType(std::any::TypeId::of::<ReflectReference>())),
            )
            .map(|(key, func)| (key.name.clone(), func.clone()))
            .collect()
    }

    /// Tries to call a fitting overload of the function with the given name and in the type id's namespace based on the arguments provided.
    /// Currently does this by repeatedly trying each overload until one succeeds or all fail.
    fn try_call_overloads(
        &self,
        type_id: TypeId,
        name: impl Into<Cow<'static, str>>,
        args: Vec<ScriptValue>,
        context: FunctionCallContext,
    ) -> Result<ScriptValue, InteropError> {
        let registry = self.script_function_registry();
        let registry = registry.read();

        let name = name.into();
        let overload_iter = match registry.iter_overloads(Namespace::OnType(type_id), name) {
            Ok(iter) => iter,
            Err(name) => {
                return Err(InteropError::missing_function(
                    name.to_string(),
                    Namespace::OnType(type_id),
                    Some(context.clone()),
                ));
            }
        };

        let mut last_error = None;
        for overload in overload_iter {
            match overload.call(args.clone(), context.clone()) {
                Ok(out) => return Ok(out),
                Err(e) => last_error = Some(e),
            }
        }

        Err(last_error.ok_or_else(|| InteropError::invariant("invariant, iterator should always return at least one item, and if the call fails it should return an error"))?)
    }

    fn construct_from_script_value(
        &self,
        descriptor: impl Into<Cow<'static, str>>,
        type_id: TypeId,
        value: Option<ScriptValue>,
    ) -> Result<Box<dyn PartialReflect>, InteropError> {
        // if the value is missing, try to construct a default and return it
        let value = match value {
            Some(value) => value,
            None => {
                let type_registry = self.type_registry();
                let type_registry = type_registry.read();
                let default_data = type_registry
                    .get_type_data::<ReflectDefault>(type_id)
                    .ok_or_else(|| {
                        InteropError::function_interop_error(
                            "construct",
                            Namespace::OnType(TypeId::of::<World>()),
                            InteropError::string(format!(
                                "field missing and no default provided: '{}'",
                                descriptor.into()
                            )),
                            None,
                        )
                    })?;
                return Ok(default_data.default().into_partial_reflect());
            }
        };

        // otherwise we need to use from_script_ref
        <Box<dyn PartialReflect>>::from_script_ref(type_id, value, self.clone())
    }

    fn construct_dynamic_struct(
        &self,
        payload: &mut HashMap<String, ScriptValue>,
        fields: Vec<(&'static str, TypeId)>,
    ) -> Result<DynamicStruct, InteropError> {
        let mut dynamic = DynamicStruct::default();
        for (field_name, field_type_id) in fields {
            let constructed = self.construct_from_script_value(
                field_name,
                field_type_id,
                payload.remove(field_name),
            )?;

            dynamic.insert_boxed(field_name, constructed);
        }
        Ok(dynamic)
    }

    fn construct_dynamic_tuple_struct(
        &self,
        payload: &mut HashMap<String, ScriptValue>,
        fields: Vec<TypeId>,
        one_indexed: bool,
    ) -> Result<DynamicTupleStruct, InteropError> {
        let mut dynamic = DynamicTupleStruct::default();
        for (field_idx, field_type_id) in fields.into_iter().enumerate() {
            // correct for indexing
            let script_idx = if one_indexed {
                field_idx + 1
            } else {
                field_idx
            };
            let field_string = script_idx.to_string();
            dynamic.insert_boxed(self.construct_from_script_value(
                field_string.clone(),
                field_type_id,
                payload.remove(&field_string),
            )?);
        }
        Ok(dynamic)
    }

    fn construct_dynamic_tuple(
        &self,
        payload: &mut HashMap<String, ScriptValue>,
        fields: Vec<TypeId>,
        one_indexed: bool,
    ) -> Result<DynamicTuple, InteropError> {
        let mut dynamic = DynamicTuple::default();
        for (field_idx, field_type_id) in fields.into_iter().enumerate() {
            // correct for indexing
            let script_idx = if one_indexed {
                field_idx + 1
            } else {
                field_idx
            };

            let field_string = script_idx.to_string();

            dynamic.insert_boxed(self.construct_from_script_value(
                field_string.clone(),
                field_type_id,
                payload.remove(&field_string),
            )?);
        }
        Ok(dynamic)
    }

    /// An arbitrary type constructor utility.
    ///
    /// Allows the construction of arbitrary types (within limits dictated by the API) from the script directly
    fn construct(
        &self,
        type_: ScriptTypeRegistration,
        mut payload: HashMap<String, ScriptValue>,
        one_indexed: bool,
    ) -> Result<Box<dyn PartialReflect>, InteropError> {
        // figure out the kind of type we're building
        let type_info = type_.registration.type_info();
        // we just need to a) extract fields, if enum we need a "variant" field specifying the variant
        // then build the corresponding dynamic structure, whatever it may be

        let dynamic: Box<dyn PartialReflect> = match type_info {
            TypeInfo::Struct(struct_info) => {
                let fields_iter = struct_info
                    .field_names()
                    .iter()
                    .map(|f| {
                        Ok((
                            *f,
                            struct_info
                                .field(f)
                                .ok_or_else(|| {
                                    InteropError::invariant(
                                        "field in field_names should have reflection information",
                                    )
                                })?
                                .type_id(),
                        ))
                    })
                    .collect::<Result<Vec<_>, InteropError>>()?;
                let mut dynamic = self.construct_dynamic_struct(&mut payload, fields_iter)?;
                dynamic.set_represented_type(Some(type_info));
                Box::new(dynamic)
            }
            TypeInfo::TupleStruct(tuple_struct_info) => {
                let fields_iter = (0..tuple_struct_info.field_len())
                    .map(|f| {
                        Ok(tuple_struct_info
                            .field_at(f)
                            .ok_or_else(|| {
                                InteropError::invariant(
                                    "field in field_names should have reflection information",
                                )
                            })?
                            .type_id())
                    })
                    .collect::<Result<Vec<_>, InteropError>>()?;

                let mut dynamic =
                    self.construct_dynamic_tuple_struct(&mut payload, fields_iter, one_indexed)?;
                dynamic.set_represented_type(Some(type_info));
                Box::new(dynamic)
            }
            TypeInfo::Tuple(tuple_info) => {
                let fields_iter = (0..tuple_info.field_len())
                    .map(|f| {
                        Ok(tuple_info
                            .field_at(f)
                            .ok_or_else(|| {
                                InteropError::invariant(
                                    "field in field_names should have reflection information",
                                )
                            })?
                            .type_id())
                    })
                    .collect::<Result<Vec<_>, InteropError>>()?;

                let mut dynamic =
                    self.construct_dynamic_tuple(&mut payload, fields_iter, one_indexed)?;
                dynamic.set_represented_type(Some(type_info));
                Box::new(dynamic)
            }
            TypeInfo::Enum(enum_info) => {
                // extract variant from "variant"
                let variant = payload.remove("variant").ok_or_else(|| {
                    InteropError::function_interop_error(
                        "construct",
                        Namespace::OnType(TypeId::of::<World>()),
                        InteropError::str("missing 'variant' field in enum constructor payload"),
                        None,
                    )
                })?;

                let variant_name = String::from_script(variant, self.clone())?;

                let variant = enum_info.variant(&variant_name).ok_or_else(|| {
                    InteropError::function_interop_error(
                        "construct",
                        Namespace::OnType(TypeId::of::<World>()),
                        InteropError::string(format!(
                            "invalid variant name '{}' for enum '{}'",
                            variant_name,
                            enum_info.type_path()
                        )),
                        None,
                    )
                })?;

                let variant = match variant {
                    VariantInfo::Struct(struct_variant_info) => {
                        // same as above struct variant
                        let fields_iter = struct_variant_info
                            .field_names()
                            .iter()
                            .map(|f| {
                                Ok((
                                    *f,
                                    struct_variant_info
                                        .field(f)
                                        .ok_or_else(|| {
                                            InteropError::invariant(
                                                "field in field_names should have reflection information",
                                            )
                                        })?
                                        .type_id(),
                                ))
                            })
                            .collect::<Result<Vec<_>, InteropError>>()?;

                        let dynamic = self.construct_dynamic_struct(&mut payload, fields_iter)?;
                        DynamicVariant::Struct(dynamic)
                    }
                    VariantInfo::Tuple(tuple_variant_info) => {
                        // same as tuple variant
                        let fields_iter = (0..tuple_variant_info.field_len())
                            .map(|f| {
                                Ok(tuple_variant_info
                                .field_at(f)
                                .ok_or_else(|| {
                                    InteropError::invariant(
                                        "field in field_names should have reflection information",
                                    )
                                })?
                                .type_id())
                            })
                            .collect::<Result<Vec<_>, InteropError>>()?;

                        let dynamic =
                            self.construct_dynamic_tuple(&mut payload, fields_iter, one_indexed)?;
                        DynamicVariant::Tuple(dynamic)
                    }
                    VariantInfo::Unit(_) => DynamicVariant::Unit,
                };
                let mut dynamic = DynamicEnum::new(variant_name, variant);
                dynamic.set_represented_type(Some(type_info));
                Box::new(dynamic)
            }
            _ => {
                return Err(InteropError::unsupported_operation(
                    Some(type_info.type_id()),
                    Some(Box::new(payload)),
                    "Type constructor not supported",
                ));
            }
        };

        // try to construct type from reflect
        // TODO: it would be nice to have a <dyn PartialReflect>::from_reflect_with_fallback equivalent, that does exactly that
        // only using this as it's already there and convenient, the clone variant hitting will be confusing to end users
        <dyn PartialReflect>::from_reflect_or_clone(dynamic.as_ref(), self.clone())
    }

    /// Loads a script from the given asset path with default settings.
    fn load_script_asset<'a>(
        &self,
        asset_path: impl Into<AssetPath<'a>>,
    ) -> Result<Handle<ScriptAsset>, InteropError> {
        self.with_resource(|r: &AssetServer| r.load(asset_path))
    }

    /// Checks the load state of a script asset.
    fn get_script_asset_load_state(
        &self,
        script: Handle<ScriptAsset>,
    ) -> Result<LoadState, InteropError> {
        self.with_resource(|r: &AssetServer| r.load_state(script.id()))
    }

    /// get a type registration for the type, without checking if it's a component or resource
    fn get_type_by_name(&self, type_name: &str) -> Option<ScriptTypeRegistration> {
        let type_registry = self.type_registry();
        let type_registry = type_registry.read();
        type_registry
            .get_with_short_type_path(type_name)
            .or_else(|| type_registry.get_with_type_path(type_name))
            .map(|registration| ScriptTypeRegistration::new(Arc::new(registration.clone())))
    }

    /// get a type erased type registration for the type including information about whether it's a component or resource
    fn get_type_registration(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<
        Union<
            ScriptTypeRegistration,
            Union<ScriptComponentRegistration, ScriptResourceRegistration>,
        >,
        InteropError,
    > {
        let registration = match self.get_resource_type(registration)? {
            Ok(res) => {
                return Ok(Union::new_right(Union::new_right(res)));
            }
            Err(registration) => registration,
        };

        let registration = match self.get_component_type(registration)? {
            Ok(comp) => {
                return Ok(Union::new_right(Union::new_left(comp)));
            }
            Err(registration) => registration,
        };

        Ok(Union::new_left(registration))
    }

    /// Similar to [`Self::get_type_by_name`] but returns a type erased [`ScriptTypeRegistration`], [`ScriptComponentRegistration`] or [`ScriptResourceRegistration`]
    /// depending on the underlying type and state of the world.
    fn get_type_registration_by_name(
        &self,
        type_name: String,
    ) -> Result<
        Option<
            Union<
                ScriptTypeRegistration,
                Union<ScriptComponentRegistration, ScriptResourceRegistration>,
            >,
        >,
        InteropError,
    > {
        let val = self.get_type_by_name(&type_name);
        Ok(match val {
            Some(registration) => Some(self.get_type_registration(registration)?),
            None => {
                // try the component registry
                let components = self.component_registry();
                let components = components.read();
                components
                    .get(&type_name)
                    .map(|c| Union::new_right(Union::new_left(c.registration.clone())))
            }
        })
    }

    /// get a schedule by name
    fn get_schedule_by_name(&self, schedule_name: String) -> Option<ReflectSchedule> {
        let schedule_registry = self.schedule_registry();
        let schedule_registry = schedule_registry.read();

        schedule_registry
            .get_schedule_by_name(&schedule_name)
            .cloned()
    }

    /// get a component type registration for the type
    fn get_component_type(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<Result<ScriptComponentRegistration, ScriptTypeRegistration>, InteropError> {
        Ok(match self.get_component_id(registration.type_id())? {
            Some(comp_id) => Ok(ScriptComponentRegistration::new(registration, comp_id)),
            None => Err(registration),
        })
    }

    /// get a resource type registration for the type
    fn get_resource_type(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<Result<ScriptResourceRegistration, ScriptTypeRegistration>, InteropError> {
        Ok(match self.get_resource_id(registration.type_id())? {
            Some(resource_id) => Ok(ScriptResourceRegistration::new(registration, resource_id)),
            None => Err(registration),
        })
    }

    /// add a default component to an entity
    fn add_default_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
    ) -> Result<(), InteropError> {
        // we look for ReflectDefault or ReflectFromWorld data then a ReflectComponent data
        let instance = if let Some(default_td) = registration
            .type_registration()
            .type_registration()
            .data::<ReflectDefault>()
        {
            default_td.default()
        } else if let Some(from_world_td) = registration
            .type_registration()
            .type_registration()
            .data::<ReflectFromWorld>()
        {
            self.with_global_access(|world| from_world_td.from_world(world))?
        } else {
            return Err(InteropError::missing_type_data(
                registration.registration.type_id(),
                "ReflectDefault or ReflectFromWorld".to_owned(),
            ));
        };

        registration.insert_into_entity(self.clone(), entity, instance)
    }

    /// insert the component into the entity
    fn insert_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
        value: ReflectReference,
    ) -> Result<(), InteropError> {
        let instance = <Box<dyn PartialReflect>>::from_script_ref(
            registration.type_registration().type_id(),
            ScriptValue::Reference(value),
            self.clone(),
        )?;

        let reflect = instance.try_into_reflect().map_err(|v| {
            InteropError::failed_from_reflect(
                Some(registration.type_registration().type_id()),
                format!("instance produced by conversion to target type when inserting component is not a full reflect type: {v:?}"),
            )
        })?;

        registration.insert_into_entity(self.clone(), entity, reflect)
    }

    /// get the component from the entity
    fn get_component(
        &self,
        entity: Entity,
        component_registration: ScriptComponentRegistration,
    ) -> Result<Option<ReflectReference>, InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        let entity = cell
            .get_entity(entity)
            .map_err(|_| InteropError::missing_entity(entity))?;

        if entity.contains_id(component_registration.component_id) {
            Ok(Some(ReflectReference {
                base: ReflectBaseType {
                    type_id: component_registration.type_registration().type_id(),
                    base_id: ReflectBase::Component(
                        entity.id(),
                        component_registration.component_id,
                    ),
                },
                reflect_path: Default::default(),
            }))
        } else {
            Ok(None)
        }
    }

    /// check if the entity has the component
    fn has_component(
        &self,
        entity: Entity,
        component_id: ComponentId,
    ) -> Result<bool, InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        let entity = cell
            .get_entity(entity)
            .map_err(|_| InteropError::missing_entity(entity))?;

        Ok(entity.contains_id(component_id))
    }

    /// remove the component from the entity
    fn remove_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
    ) -> Result<(), InteropError> {
        registration.remove_from_entity(self.clone(), entity)
    }

    /// get the given resource
    fn get_resource(
        &self,
        resource_id: ComponentId,
    ) -> Result<Option<ReflectReference>, InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        let component_info = match cell.components().get_info(resource_id) {
            Some(info) => info,
            None => return Ok(None),
        };

        Ok(Some(ReflectReference {
            base: ReflectBaseType {
                type_id: component_info
                    .type_id()
                    .ok_or_else(|| {
                        InteropError::unsupported_operation(
                            None,
                            None,
                            format!(
                                "Resource {} does not have a type id. Such resources are not supported by BMS.",
                                component_info.name()
                            ),
                        )
                    })?,
                base_id: ReflectBase::Resource(resource_id),
            },
            reflect_path: Default::default(),
        }))
    }

    /// remove the given resource
    fn remove_resource(
        &self,
        registration: ScriptResourceRegistration,
    ) -> Result<(), InteropError> {
        let component_data = registration
            .type_registration()
            .type_registration()
            .data::<ReflectResource>()
            .ok_or_else(|| {
                InteropError::missing_type_data(
                    registration.registration.type_id(),
                    "ReflectResource".to_owned(),
                )
            })?;

        //  TODO: this shouldn't need entire world access it feels
        self.with_global_access(|world| component_data.remove(world))
    }

    /// Spawns a new entity in the world
    fn spawn(&self) -> Result<Entity, InteropError> {
        self.with_global_access(|world| {
            let mut command_queue = CommandQueue::default();
            let mut commands = Commands::new(&mut command_queue, world);
            let id = commands.spawn_empty().id();
            command_queue.apply(world);
            id
        })
    }

    /// check if the entity has the resource
    fn has_resource(&self, resource_id: ComponentId) -> Result<bool, InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        // Safety: we are not reading the value at all
        let res_ptr = unsafe { cell.get_resource_by_id(resource_id) };
        Ok(res_ptr.is_some())
    }

    /// check the given entity exists
    fn has_entity(&self, entity: Entity) -> Result<bool, InteropError> {
        self.is_valid_entity(entity)
    }

    /// get the children of the given entity
    fn get_children(&self, entity: Entity) -> Result<Vec<Entity>, InteropError> {
        if !self.is_valid_entity(entity)? {
            return Err(InteropError::missing_entity(entity));
        }

        self.with_component(entity, |c: Option<&Children>| {
            c.map(|c| c.to_vec()).unwrap_or_default()
        })
    }

    /// get the parent of the given entity
    fn get_parent(&self, entity: Entity) -> Result<Option<Entity>, InteropError> {
        if !self.is_valid_entity(entity)? {
            return Err(InteropError::missing_entity(entity));
        }

        self.with_component(entity, |c: Option<&ChildOf>| c.map(|c| c.parent()))
    }

    /// insert children into the given entity
    fn push_children(&self, parent: Entity, children: &[Entity]) -> Result<(), InteropError> {
        // verify entities exist
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }
        for c in children {
            if !self.is_valid_entity(*c)? {
                return Err(InteropError::missing_entity(*c));
            }
        }
        self.with_global_access(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).add_children(children);
            queue.apply(world);
        })
    }

    /// remove children from the given entity
    fn remove_children(&self, parent: Entity, children: &[Entity]) -> Result<(), InteropError> {
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }

        for c in children {
            if !self.is_valid_entity(*c)? {
                return Err(InteropError::missing_entity(*c));
            }
        }
        self.with_global_access(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).detach_children(children);
            queue.apply(world);
        })
    }

    /// insert children into the given entity at the given index
    fn insert_children(
        &self,
        parent: Entity,
        index: usize,
        children: &[Entity],
    ) -> Result<(), InteropError> {
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }

        for c in children {
            if !self.is_valid_entity(*c)? {
                return Err(InteropError::missing_entity(*c));
            }
        }

        self.with_global_access(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).insert_children(index, children);
            queue.apply(world);
        })
    }

    /// despawn this and all children of the given entity recursively
    fn despawn_recursive(&self, parent: Entity) -> Result<(), InteropError> {
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }
        self.with_global_access(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).despawn();
            queue.apply(world);
        })
    }

    /// despawn the given entity
    fn despawn(&self, entity: Entity) -> Result<(), InteropError> {
        if !self.is_valid_entity(entity)? {
            return Err(InteropError::missing_entity(entity));
        }

        self.with_global_access(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(entity).remove::<Children>().despawn();
            queue.apply(world);
        })
    }

    /// despawn all children of the given entity recursively
    fn despawn_descendants(&self, parent: Entity) -> Result<(), InteropError> {
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }

        self.with_global_access(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).despawn_related::<Children>();
            queue.apply(world);
        })
    }

    /// Sends AppExit event to the world with success status
    fn exit(&self) -> Result<(), InteropError> {
        self.with_global_access(|world| {
            world.write_message(AppExit::Success);
        })
    }

    /// checks if a given entity exists and is valid
    fn is_valid_entity(&self, entity: Entity) -> Result<bool, InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        Ok(cell.get_entity(entity).is_ok() && entity.index().index() != 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy_reflect::{GetTypeRegistration, ReflectFromReflect};
    use test_utils::test_data::{SimpleEnum, SimpleStruct, SimpleTupleStruct, setup_world};

    #[test]
    fn test_construct_struct() {
        let mut world = setup_world(|_, _| {});
        let world = WorldAccessGuard::new_exclusive(&mut world);

        let registry = world.type_registry();
        let registry = registry.read();

        let registration = registry.get(TypeId::of::<SimpleStruct>()).unwrap().clone();
        let type_registration = ScriptTypeRegistration::new(Arc::new(registration));

        let payload = HashMap::from_iter(vec![("foo".to_owned(), ScriptValue::Integer(1))]);

        let result = world.construct(type_registration, payload, false);
        let expected =
            Ok::<_, InteropError>(Box::new(SimpleStruct { foo: 1 }) as Box<dyn PartialReflect>);
        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));
    }

    #[test]
    fn test_construct_tuple_struct() {
        let mut world = setup_world(|_, _| {});
        let world = WorldAccessGuard::new_exclusive(&mut world);

        let registry = world.type_registry();
        let registry = registry.read();

        let registration = registry
            .get(TypeId::of::<SimpleTupleStruct>())
            .unwrap()
            .clone();
        let type_registration = ScriptTypeRegistration::new(Arc::new(registration));

        // zero indexed
        let payload = HashMap::from_iter(vec![("0".to_owned(), ScriptValue::Integer(1))]);

        let result = world.construct(type_registration.clone(), payload, false);
        let expected =
            Ok::<_, InteropError>(Box::new(SimpleTupleStruct(1)) as Box<dyn PartialReflect>);
        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));

        // one indexed
        let payload = HashMap::from_iter(vec![("1".to_owned(), ScriptValue::Integer(1))]);

        let result = world.construct(type_registration, payload, true);
        let expected =
            Ok::<_, InteropError>(Box::new(SimpleTupleStruct(1)) as Box<dyn PartialReflect>);

        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));
    }

    #[test]
    fn test_construct_tuple() {
        let mut world = setup_world(|_, registry| {
            registry.register::<(usize, usize)>();
            // TODO: does this ever get registered on normal types? I don't think so: https://github.com/bevyengine/bevy/issues/17981
            registry.register_type_data::<(usize, usize), ReflectFromReflect>();
        });

        <usize as GetTypeRegistration>::get_type_registration();
        let world = WorldAccessGuard::new_exclusive(&mut world);

        let registry = world.type_registry();
        let registry = registry.read();

        let registration = registry
            .get(TypeId::of::<(usize, usize)>())
            .unwrap()
            .clone();
        let type_registration = ScriptTypeRegistration::new(Arc::new(registration));

        // zero indexed
        let payload = HashMap::from_iter(vec![
            ("0".to_owned(), ScriptValue::Integer(1)),
            ("1".to_owned(), ScriptValue::Integer(2)),
        ]);

        let result = world.construct(type_registration.clone(), payload, false);
        let expected = Ok::<_, InteropError>(Box::new((1, 2)) as Box<dyn PartialReflect>);
        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));

        // one indexed
        let payload = HashMap::from_iter(vec![
            ("1".to_owned(), ScriptValue::Integer(1)),
            ("2".to_owned(), ScriptValue::Integer(2)),
        ]);

        let result = world.construct(type_registration.clone(), payload, true);
        let expected = Ok::<_, InteropError>(Box::new((1, 2)) as Box<dyn PartialReflect>);
        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));
    }

    #[test]
    fn test_construct_enum() {
        let mut world = setup_world(|_, _| {});
        let world = WorldAccessGuard::new_exclusive(&mut world);

        let registry = world.type_registry();
        let registry = registry.read();

        let registration = registry.get(TypeId::of::<SimpleEnum>()).unwrap().clone();
        let type_registration = ScriptTypeRegistration::new(Arc::new(registration));

        // struct version
        let payload = HashMap::from_iter(vec![
            ("foo".to_owned(), ScriptValue::Integer(1)),
            ("variant".to_owned(), ScriptValue::String("Struct".into())),
        ]);

        let result = world.construct(type_registration.clone(), payload, false);
        let expected = Ok::<_, InteropError>(
            Box::new(SimpleEnum::Struct { foo: 1 }) as Box<dyn PartialReflect>
        );
        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));

        // tuple struct version
        let payload = HashMap::from_iter(vec![
            ("0".to_owned(), ScriptValue::Integer(1)),
            (
                "variant".to_owned(),
                ScriptValue::String("TupleStruct".into()),
            ),
        ]);

        let result = world.construct(type_registration.clone(), payload, false);
        let expected =
            Ok::<_, InteropError>(Box::new(SimpleEnum::TupleStruct(1)) as Box<dyn PartialReflect>);

        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));

        // unit version
        let payload = HashMap::from_iter(vec![(
            "variant".to_owned(),
            ScriptValue::String("Unit".into()),
        )]);

        let result = world.construct(type_registration, payload, false);
        let expected = Ok::<_, InteropError>(Box::new(SimpleEnum::Unit) as Box<dyn PartialReflect>);
        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));
    }
}
