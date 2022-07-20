use std::borrow::Cow;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::sync::{Weak,Arc};
use std::convert::AsRef;
use std::ops::{Deref,DerefMut};
use crate::{ScriptRef, ReflectedValue, impl_tealr_type, ScriptRefBase, ReflectPtr, ValueIndex, IdentitySubReflect, SubReflect};
use bevy::ecs::system::Command;
use bevy::hierarchy::BuildWorldChildren;
use bevy::reflect::ReflectRef;
use bevy::reflect::erased_serde::Serialize;
use bevy::{reflect::{reflect_trait, Reflect, TypeRegistry, TypeRegistration, DynamicStruct, DynamicTupleStruct, DynamicTuple, DynamicList, DynamicArray, DynamicMap}, prelude::{World, ReflectComponent, ReflectDefault, ReflectResource}, hierarchy::{Children, Parent, DespawnChildrenRecursive, DespawnRecursive}};

use parking_lot::RwLock;
use serde::Deserialize;
use tealr::TypeName;
use tealr::mlu::mlua::MetaMethod;
use tealr::mlu::{mlua::{Lua, Value,self, UserData, ToLua,FromLua}, TealData, TealDataMethods};

pub use crate::api::generated::*;


/// For internal use only.
/// 
/// Mainly necessary for separation of concerns on the [`ScriptRef`] type, but might have other uses potentially.
/// 
/// This is not the same as [`LuaProxyable`], internally this in fact will use [`LuaProxyable`] so treating it like so will cause inifnite loops.
pub(crate) trait ApplyLua {
    /// set the proxied object with the given lua value
    fn apply_lua<'lua>(&mut self, ctx: &'lua Lua, v: Value<'lua>) -> mlua::Result<()>;
}
impl ApplyLua for ScriptRef{
    /// Applies the given lua value to the proxied reflect type. Semantically equivalent to `Reflect::apply`
    fn apply_lua<'lua>(&mut self, ctx: &'lua Lua, v: Value<'lua>) -> Result<(),mlua::Error> {
        let luaworld = ctx.globals()
                        .get::<_, LuaWorld>("world")
                        .unwrap();

        // remove typedata from the world to be able to manipulate world 
        let proxyable = {
            let world = luaworld.upgrade().unwrap();
            let world = &world.read();
            let type_registry = world.resource::<TypeRegistry>().read();
            type_registry.get_type_data::<ReflectLuaProxyable>(self.get(|s| s.type_id())).cloned()
        };

        if let Some(ud) = proxyable{
            return ud.apply_lua(self,ctx, v)
        } else if let Value::UserData(v) = &v{
            if v.is::<ReflectedValue>() {
                let b = v.take::<ReflectedValue>().unwrap();
                self.apply(&b.into());
                return Ok(())
            }
        }

        Err(mlua::Error::RuntimeError(self.get(|s| 
            format!("Attempted to assign {v:?} to {:?} at path: `{}`. Did you forget to call `app.register_foreign_lua_type::<{}>`?",
                s,
                self.path.as_ref().map(|p| p.as_str()).unwrap_or("\"\""),
                s.type_name()
            )))
        )

    }
}

impl <'lua>ToLua<'lua> for ScriptRef {
    /// Converts the LuaRef to the most convenient representation
    /// checking conversions in this order:
    /// - A primitive or bevy type which has a reflect interface is converted to a custom UserData exposing its API to lua conveniently
    /// - A type implementing CustomUserData is converted with its `ref_to_lua` method
    /// - Finally the method is represented as a `ReflectedValue` which exposes the Reflect interface 
    fn to_lua(self, ctx: &'lua Lua) -> mlua::Result<Value<'lua>> {
        let luaworld = ctx.globals()
            .get::<_, LuaWorld>("world")
            .unwrap();

        let world = luaworld.upgrade().unwrap();
        let world = &mut world.read();

        let typedata = world.resource::<TypeRegistry>();
        let g = typedata.read();
        if let Some(v) = g.get_type_data::<ReflectLuaProxyable>(self.get(|s| s.type_id())) {
            Ok(v.ref_to_lua(self,ctx)?)
        } else {
            ReflectedValue{ref_: self.clone()}.to_lua(ctx)
        }
    }
}

impl_tealr_type!(ReflectedValue);
impl TealData for ReflectedValue {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |_, val, ()| {
            val.ref_.get(|s| 
                Ok(format!("{:#?}", &s)
                ))
        });

        methods.add_meta_method_mut(MetaMethod::Index, |_, val, field: Value| {
            let r = val.ref_.index(field).unwrap();
            Ok(r)
        });

        methods.add_meta_method_mut(
            MetaMethod::NewIndex,
            |ctx, val, (field, new_val): (Value, Value)| {
                val.ref_.index(field).unwrap().apply_lua(ctx, new_val).unwrap();
                Ok(())
            },
        );

        methods.add_meta_method(MetaMethod::Len, |_, val, ()| {
            val.ref_.get(|s| {
                let r = s.reflect_ref();
                if let ReflectRef::List(v) = r {
                    Ok(v.len())
                } else if let ReflectRef::Map(v) = r {
                    Ok(v.len())
                } else if let ReflectRef::Tuple(v) = r {
                    Ok(v.field_len())
                } else {
                    panic!("No length on this type");
                }
            })
        });

    }

}
/// A higher level trait for allowing types to be interpreted as custom lua proxy types (or just normal types, this interface is flexible).
/// Types implementing this trait can have [`ReflectLuaProxyable`] type data registrations inserted into the reflection API.
/// 
/// Types registered via the reflection API this way can be accessed from Lua via [`ScriptRef`] objects (via field access).
pub trait LuaProxyable : {
    /// a version of [`mlua::ToLua::to_lua`] which does not consume the object.
    /// 
    /// Note: The self reference is sourced from the given ScriptRef, attempting to get another mutable reference from the ScriptRef might
    /// cause a runtime error to prevent breaking of aliasing rules
    fn ref_to_lua<'lua>(self_ : ScriptRef, lua: &'lua Lua) -> mlua::Result<Value<'lua>>;

    /// similar to [`Reflect::apply`]
    /// 
    /// Note: 
    /// The self reference is sourced from the given ScriptRef, attempting to get another reference from the ScriptRef might
    /// cause a runtime error to prevent breaking of aliasing rules
    fn apply_lua<'lua>(self_ : &mut ScriptRef, lua: &'lua Lua, new_val: Value<'lua>) -> mlua::Result<()>;
}

/// A struct providing type data for the `LuaProxyable` trait.
/// 
/// This allows casting static methods from the `LuaProxyable trait`.
#[derive(Clone)]
pub struct ReflectLuaProxyable {
    ref_to_lua: for<'lua> fn(ref_ : ScriptRef, lua: &'lua Lua) -> mlua::Result<Value<'lua>>,
    apply_lua: for<'lua> fn(ref_ : &mut ScriptRef, lua: &'lua Lua, new_val: Value<'lua>) -> mlua::Result<()>,
}

impl ReflectLuaProxyable {
    pub fn ref_to_lua<'lua>(
        &self,
        ref_ : ScriptRef, lua: &'lua Lua
    ) -> mlua::Result<Value<'lua>> {
        (self.ref_to_lua)(ref_,lua)
    }

    pub fn apply_lua<'lua>(
        &self,
        ref_ : &mut ScriptRef, lua: &'lua Lua, new_val: Value<'lua>
    ) -> mlua::Result<()> {
        (self.apply_lua)(ref_, lua, new_val)
    }
}


impl<T: LuaProxyable + bevy::reflect::Reflect> bevy::reflect::FromType<T> for ReflectLuaProxyable {
    fn from_type() -> Self {
        Self {
            ref_to_lua: T::ref_to_lua,
            apply_lua: T::apply_lua
        }
    }
}

/// A dummy trait used to combat rust's orphan rules 
/// 
/// In the future when trait specialization is a thing, this might be a companion trait
/// to `RefLuaType` which allows non Clone types to be used
pub trait ValueLuaType {}

impl<T: Clone + UserData + Send + ValueLuaType + Reflect + 'static> LuaProxyable for T {
    fn ref_to_lua<'lua>(self_ : ScriptRef, lua: &'lua Lua) -> mlua::Result<Value<'lua>>{
        self_.get_typed(|s: &Self| s.clone().to_lua(lua))
    }

    fn apply_lua<'lua>(self_ : &mut ScriptRef, _: &'lua Lua, new_val: Value<'lua>) -> mlua::Result<()>{
        if let Value::UserData(v) = new_val {
            let o = v.borrow::<T>()?;

            self_.get_mut_typed(|s,_| {
                *s = o.clone()
            });
            
            Ok(())
        } else {
            Err(mlua::Error::RuntimeError(
                "Error in assigning to custom user data".to_owned(),
            ))
        }
    }
}


use paste::paste;


/// Implements custom user data for simple copy types which implement to and from lua
macro_rules! impl_copy_custom_user_data(
    ( $($num_ty:ty),*) => {
        paste! {
            $(
                impl LuaProxyable for $num_ty {
                    fn ref_to_lua< 'lua>(self_: crate::ScriptRef,lua: & 'lua tealr::mlu::mlua::Lua) -> tealr::mlu::mlua::Result<tealr::mlu::mlua::Value< 'lua> >  {
                        self_.get_typed(|self_ : &Self| self_.to_lua(lua))
                    }
                
                    fn apply_lua< 'lua>(self_: &mut crate::ScriptRef,lua: & 'lua tealr::mlu::mlua::Lua,new_val:tealr::mlu::mlua::Value< 'lua>) -> tealr::mlu::mlua::Result<()>  {
                        self_.set_val(Self::from_lua(new_val,lua)?);
                        Ok(())
                    }
                }
            )*
        }
    }  
);


impl_copy_custom_user_data!(bool);
impl_copy_custom_user_data!(f32,f64);
impl_copy_custom_user_data!(i8,i16,i32,i64,i128,isize);
impl_copy_custom_user_data!(u8,u16,u32,u64,u128,usize);

impl LuaProxyable for String {
    fn ref_to_lua<'lua>(self_: ScriptRef,lua: & 'lua Lua) -> mlua::Result<Value< 'lua> >  {
        self_.get_typed(|self_ : &String| self_.as_str().to_lua(lua))
    }

    fn apply_lua<'lua>(self_: &mut ScriptRef,lua: & 'lua Lua,new_val:Value< 'lua>) -> mlua::Result<()>  {
        self_.get_mut_typed(|self_,_| Ok(*self_ = Self::from_lua(new_val,lua)?))
    }
}

impl <T : LuaProxyable + Reflect + for <'a> Deserialize<'a> + serde::Serialize + Default + Clone>LuaProxyable for Option<T>{
    fn ref_to_lua< 'lua>(self_: ScriptRef,lua: & 'lua Lua) -> mlua::Result<Value< 'lua>>  {
        self_.get_typed(|s : &Option<T>| match  s {
            Some(v) => T::ref_to_lua(self_.sub_ref(
                    |ref_| 
                        ref_.downcast_ref::<Option<T>>()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                    |ref_| 
                        ref_.downcast_mut::<Option<T>>()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                    )
                ,lua ),
            None => Ok(Value::Nil),
        })
    }

    fn apply_lua< 'lua>(self_: &mut ScriptRef,lua: & 'lua Lua,new_val:Value< 'lua>) -> mlua::Result<()>  {
        if let Value::Nil = new_val {
            self_.get_mut_typed(|s : &mut Option<T>,_| Ok(*s = None))
        } else {
            // we need to do this in two passes, first 
            // ensure that the target type is the 'some' variant to allow a sub reference
            self_.get_mut_typed(|s : &mut Option<T>,_| {
                if s.is_none() {
                    *s = Some(T::default());
                }
            });

            T::apply_lua(
                &mut self_.sub_ref(
                    |ref_| 
                        ref_.downcast_ref::<Option<T>>()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                    |ref_| 
                        ref_.downcast_mut::<Option<T>>()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                    )
                ,lua, 
                new_val)
            
        }
    }
}



#[derive(Clone)]
pub struct LuaTypeRegistration(Arc<TypeRegistration>);
impl_tealr_type!(LuaTypeRegistration);


impl TealData for LuaTypeRegistration {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("An object representing an existing and registered rust type.");
        methods.document_type("Can be obtained via [`LuaWorld::get_type_by_name`].");
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.document("The [short name](https://docs.rs/bevy/latest/bevy/reflect/struct.TypeRegistration.html#method.get_short_name) of a type");
        fields.add_field_method_get("short_name", |_,s| Ok(s.0.short_name().to_string()));
        
        fields.document("The full name of the type");
        fields.add_field_method_get("type_name", |_,s| Ok(s.0.type_name()));
    }
}

impl Deref for LuaTypeRegistration {
    type Target=TypeRegistration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct LuaWorld(Weak<RwLock<World>>);

impl Deref for LuaWorld {
    type Target = Weak<RwLock<World>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Weak<RwLock<World>>> for LuaWorld {
    fn as_ref(&self) -> &Weak<RwLock<World>> {
        &self.0
    }
}

impl LuaWorld {
    pub fn new(w : Weak<RwLock<World>>) -> Self {
        Self(w)
    }
}

impl_tealr_type!(LuaWorld);

impl TealData for LuaWorld {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {

        methods.document_type("Represents the bevy world all scripts live in.");
        methods.document_type("Provides ways to interact with and modify the world.");


        methods.document("Retrieves children entities of the parent entity if it has any.");
        methods.add_method("get_children", |_,world, parent : LuaEntity| {
            let w = world.upgrade().unwrap();
            let w = &mut w.write();
            
            let children : Option<Vec<LuaEntity>> = w.get::<Children>(parent.clone())
                .map(|children| children.iter().map(|e| LuaEntity::new(*e)).collect::<Vec<_>>());

            Ok(children)
        });

        methods.document("Retrieves the parent entity of the given entity if it has any.");
        methods.add_method("get_parent", |_,world, parent : LuaEntity| {
            let w = world.upgrade().unwrap();
            let w = &mut w.write();
            
            let parent : Option<LuaEntity> = w.get::<Parent>(parent.clone())
                .map(|parent| LuaEntity::new(parent.0));

            Ok(parent)
        });

        methods.document("Attaches children entities to the given parent entity.");
        methods.add_method("push_children", |_,world, (parent,children) : (LuaEntity,Vec<LuaEntity>)| {
            let w = world.upgrade().unwrap();
            let w = &mut w.write();
            let children = children.iter().map(|e| e.clone()).collect::<Vec<_>>();
            
            w.get_entity_mut(parent.clone())
                .map(|mut entity| {entity.push_children(&children);});

            Ok(())
        });

        methods.document("Attaches child entity to the given parent entity.");
        methods.add_method("push_child", |_,world, (parent,child) : (LuaEntity,LuaEntity)| {
            let w = world.upgrade().unwrap();
            let w = &mut w.write();
            
            w.get_entity_mut(parent.clone())
                .map(|mut entity| {entity.push_children(&[child.clone()]);});

            Ok(())
        });

        methods.document("Removes children entities from the given parent entity.");
        methods.add_method("remove_children", |_,world, (parent,children) : (LuaEntity,Vec<LuaEntity>)| {
            let w = world.upgrade().unwrap();
            let w = &mut w.write();
            let children = children.iter().map(|e| e.clone()).collect::<Vec<_>>();

            w.get_entity_mut(parent.clone())
                .map(|mut entity| {entity.remove_children(&children);});

            Ok(())
        });

        methods.document("Removes child entity from the given parent entity.");
        methods.add_method("remove_child", |_,world, (parent,child) : (LuaEntity,LuaEntity)| {
            let w = world.upgrade().unwrap();
            let w = &mut w.write();

            w.get_entity_mut(parent.clone())
                .map(|mut entity| {entity.remove_children(&[child.clone()]);});

            Ok(())
        });

        methods.document("Inserts children entities to the given parent entity at the given index.");
        methods.add_method("insert_children", |_,world, (parent,index,children) : (LuaEntity,usize,Vec<LuaEntity>)| {
            let w = world.upgrade().unwrap();
            let w = &mut w.write();
            let children = children.iter().map(|e| e.clone()).collect::<Vec<_>>();

            w.get_entity_mut(parent.clone())
                .map(|mut entity| {entity.insert_children(index, &children);});

            Ok(())
        });

        methods.document("Inserts child entity to the given parent entity at the given index.");
        methods.add_method("insert_child", |_,world, (parent,index,children) : (LuaEntity,usize,LuaEntity)| {
            let w = world.upgrade().unwrap();
            let w = &mut w.write();

            w.get_entity_mut(parent.clone())
                .map(|mut entity| {entity.insert_children(index, &[children.clone()]);});

            Ok(())
        });

        methods.document("Despawns the given entity's children recursively");
        methods.add_method("despawn_children_recursive", |_,world, entity : LuaEntity| {
            let w = world.upgrade().unwrap();
            let w = &mut w.write();

            Ok(DespawnChildrenRecursive{
                entity: entity.clone(),
            }.write(w))
        });

        methods.document("Despawns the given entity and the entity's children recursively");
        methods.add_method("despawn_recursive", |_,world, entity : LuaEntity| {
            let w = world.upgrade().unwrap();
            let w = &mut w.write();

            Ok(DespawnRecursive{
                entity: entity.clone(),
            }.write(w))
        });

        methods.document("Despawns the given entity and the entity's children recursively");
        methods.add_method("insert_children", |_,world, entity : LuaEntity| {
            let w = world.upgrade().unwrap();
            let w = &mut w.write();

            Ok(DespawnRecursive{
                entity: entity.clone(),
            }.write(w))
        });
    
        methods.document("Retrieves type information given either a short (`MyType`) or fully qualified rust type name (`MyModule::MyType`).");
        methods.document("Returns `nil` if no such type exists or if one wasn't registered on the rust side.");
        methods.document("\n");
        methods.document("This is used extensively in [`LuaWorld`]");
        methods.add_method("get_type_by_name", |_,world,type_name: String| {
            let w = world.upgrade().unwrap();
            let w = &w.read();

            let registry: &TypeRegistry = w.get_resource().unwrap();

            let registry = registry.read();     
            
            Ok(registry
                .get_with_short_name(&type_name)
                .or(
                    registry.get_with_name(&type_name)   
                )
                .map(|registration| LuaTypeRegistration(Arc::new(registration.clone())))
            )
        });

        methods.document("Inserts a component of the given type to the given entity by instantiating a default version of it.");
        methods.document("The component can then be modified using field access.");
        methods.add_method("add_default_component", |_, world, (entity, comp_type): (LuaEntity, LuaTypeRegistration)| {
            // grab this entity before acquiring a lock in case it's a reference
            let entity = entity.clone();
            let w = world.upgrade().unwrap();
            let w = &mut w.write();

            let component_data = comp_type.data::<ReflectComponent>()
                .ok_or_else(|| mlua::Error::RuntimeError(format!("Not a component {}",comp_type.short_name())))?;

            // this is just a formality
            // TODO: maybe get an add_default impl added to ReflectComponent
            // this means that we don't require ReflectDefault for adding components!
            match comp_type.0.type_info(){
                bevy::reflect::TypeInfo::Struct(_) => component_data.add(w, entity, &DynamicStruct::default()),
                bevy::reflect::TypeInfo::TupleStruct(_) => component_data.add(w, entity, &DynamicTupleStruct::default()),
                bevy::reflect::TypeInfo::Tuple(_) => component_data.add(w, entity, &DynamicTuple::default()),
                bevy::reflect::TypeInfo::List(_) => component_data.add(w, entity, &DynamicList::default()),
                bevy::reflect::TypeInfo::Array(_) => component_data.add(w, entity, &DynamicArray::new(Box::new([]))),
                bevy::reflect::TypeInfo::Map(_) => component_data.add(w, entity, &DynamicMap::default()),
                bevy::reflect::TypeInfo::Value(_) | 
                bevy::reflect::TypeInfo::Dynamic(_) => component_data.add(w, entity, 
                    comp_type.data::<ReflectDefault>().ok_or_else(|| 
                        mlua::Error::RuntimeError(format!("Component {} is a value or dynamic type with no `ReflectDefault` type_data, cannot instantiate sensible value",comp_type.short_name())))?
                        .default()
                        .as_ref())
            };

            Ok(
                unsafe{ 
                    ScriptRef::new(
                        ScriptRefBase::Component{ 
                            comp: component_data.clone(), 
                            entity: entity,
                            world: world.as_ref().clone()
                        }, 
                        Some("".to_string()), 
                        (component_data.reflect(w,entity).unwrap() as *const dyn Reflect).into(),
                    )
                }
            )
        });
    
        methods.document("Retrieves a component of the given type from the given entity.");
        methods.document("If such a component does not exist returns `nil`.");
        methods.add_method("get_component", |_, world, (entity, comp_type) : (LuaEntity,LuaTypeRegistration)| {

            // grab this entity before acquiring a lock in case it's a reference
            let entity = entity.clone();
            let w = world.upgrade().unwrap();
            let w = &w.read();

            let component_data = comp_type.data::<ReflectComponent>()
                .ok_or_else(|| mlua::Error::RuntimeError(format!("Not a component {}",comp_type.short_name())))?;

            Ok(component_data
                .reflect(w, entity)
                .map(|component| 
                    unsafe{
                        ScriptRef::new(
                            ScriptRefBase::Component{ 
                                comp: component_data.clone(), 
                                entity,
                                world: world.as_ref().clone()
                            }, 
                            Some("".to_string()), 
                            (component as *const dyn Reflect).into(),
                        )
                    }
                ))
        });

        methods.document("Returns `true` if the given entity contains a component of the given type.");
        methods.add_method("has_component", |_, world, (entity, comp_type) : (LuaEntity,LuaTypeRegistration)| {

            // grab this entity before acquiring a lock in case it's a reference
            let entity = entity.clone();
            let w = world.upgrade().unwrap();
            let w = &w.read();

            let component_data = comp_type.data::<ReflectComponent>()
                .ok_or_else(|| mlua::Error::RuntimeError(format!("Not a component {}",comp_type.short_name())))?;

            Ok(component_data.reflect(w, entity).is_some())
        });


        methods.document("Removes the given component from the given entity, does nothing if it doesn't exist on the entity.");
        methods.add_method("remove_component", |_, world, (entity, comp_type) : (LuaEntity, LuaTypeRegistration)| {
            // grab this entity before acquiring a lock in case it's a reference
            let entity = entity.clone();
            let w = world.upgrade().unwrap();
            let w = &mut w.write();
            
            let component_data = comp_type.data::<ReflectComponent>()
                .ok_or_else(|| mlua::Error::RuntimeError(format!("Not a component {}",comp_type.short_name())))?;

            Ok(component_data.remove(w, entity))
        });


        methods.document("Retrieves a resource of the given type from the world.");
        methods.document("If such a resource does not exist returns `nil`.");
        methods.add_method("get_resource", |_, world, res_type : LuaTypeRegistration| {

            let w = world.upgrade().unwrap();
            let w = &mut w.write();


            let resource_data = res_type.data::<ReflectResource>()
                .ok_or_else(|| mlua::Error::RuntimeError(format!("Not a resource {}",res_type.short_name())))?;

            Ok(resource_data
                .reflect(&w)
                .map(|component| 
                    unsafe{ 
                        ScriptRef::new(
                            ScriptRefBase::Resource{ 
                                res: resource_data.clone(), 
                                world: world.as_ref().clone()
                            }, 
                            Some("".to_string()), 
                            (component as *const dyn Reflect).into(),
                        )
                    }
                ))
        });

        methods.document("Removes the given resource from the world, if one doesn't exist it does nothing.");
        methods.add_method("remove_resource", |_, world, res_type : LuaTypeRegistration| {
            let w = world.upgrade().unwrap();
            let w = &mut w.write();

            let resource_data = res_type.data::<ReflectResource>()
                .ok_or_else(|| mlua::Error::RuntimeError(format!("Not a resource {}",res_type.short_name())))?;

            Ok(resource_data.remove(w))
        });

        methods.document("Returns `true` if the world contains a resource of the given type.");
        methods.add_method("has_resource", |_, world, res_type : LuaTypeRegistration| {

            let w = world.upgrade().unwrap();
            let w = &w.read();

            let resource_data = res_type.data::<ReflectResource>()
                .ok_or_else(|| mlua::Error::RuntimeError(format!("Not a resource {}",res_type.short_name())))?;

            Ok(resource_data.reflect(w).is_some())
        });

        methods.document("Spawns a new entity and returns its Entity ID");
        methods.add_method("spawn", |_, world, ()| {
            let w = world.upgrade().unwrap();
            let w = &mut w.write();                
            
            Ok(LuaEntity::new(w.spawn().id()))
        });

        methods.document("Despawns the given entity if it exists, returns true if deletion was successfull");
        methods.add_method("despawn", |_, world, entity: LuaEntity| {
            let w = world.upgrade().unwrap();
            let w = &mut w.write();                
            
            Ok(w.despawn(entity.clone()))
        });
       
    }   
}

