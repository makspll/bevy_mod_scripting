#![allow(unused_variables,unused_parens)]
use bevy::reflect::TypeData;
use bevy::reflect::TypeRegistry;
use bevy::prelude::*;
use parking_lot::RwLock;
use std::sync::Weak;
use std::{fmt,fmt::{Debug}};
use phf::{phf_map, Map};
use crate::LuaRefBase;
use crate::PrintableReflect;
use crate::LuaRef;
use crate::{ReflectPtr,lua::LuaEntity};
use crate::util::impl_tealr_type;

use bevy_mod_scripting_derive::{impl_lua_newtypes,replace};
use tealr::{mlu::{mlua,TealDataMethods,TealData,mlua::{prelude::*,Error,MetaMethod,Value}}};
use std::ops::Deref;






pub fn get_type_data<T: TypeData + ToOwned<Owned = T>>(w: &mut World, name: &str) -> Result<T,Error> {
    let registry: &TypeRegistry = w.get_resource().unwrap();

    let registry = registry.read();

    let reg = registry
        .get_with_short_name(&name)
        .or(registry.get_with_name(&name))
        .ok_or_else(|| Error::RuntimeError(format!(
            "Invalid component name {name}"
        )))
        .unwrap();

    let refl: T = reg
        .data::<T>()
        .ok_or_else(|| Error::RuntimeError(format!(
            "Invalid component name {name}"
        )))
        .unwrap()
        .to_owned();

    Ok(refl)
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
        methods.add_method("add_component", |_, world, (entity, comp_name): (LuaEntity, String)| {
            // grab this entity before acquiring a lock in case it's a reference
            let entity = entity.clone();
            let w = world.upgrade().unwrap();
            let w = &mut w.write();
            let refl: ReflectComponent = get_type_data(w, &comp_name)
                .map_err(|_| Error::RuntimeError(format!("Not a component {}",comp_name)))?;
            let def = get_type_data::<ReflectDefault>(w, &comp_name)
                .map_err(|_| Error::RuntimeError(format!("Component does not derive ReflectDefault and cannot be instantiated: {}",comp_name)))?;
            let s = def.default();
            refl.add(w, entity, s.as_ref());
            let id = w.components().get_id(s.type_id()).unwrap();

            Ok(LuaComponent {
                comp: LuaRef{
                    root: LuaRefBase::Component{ 
                        comp: refl.clone(), 
                        id,
                        entity: entity,
                        world: world.as_ref().clone()
                    }, 
                    path: Some("".to_string()), 
                    r: ReflectPtr::Const(refl.reflect(w,entity).unwrap())
                }    
            })
       });
    
        methods.add_method("get_component", |_, world, (entity, comp_name) : (LuaEntity,String)| {

            // grab this entity before acquiring a lock in case it's a reference
            let entity = entity.clone();

            let w = world.upgrade().unwrap();
            let w = &mut w.write();

            let refl: ReflectComponent = get_type_data(w, &comp_name)
                .map_err(|_| Error::RuntimeError(format!("Not a component {}",comp_name)))?;

            let dyn_comp = refl
                .reflect(&w, entity)
                .ok_or_else(|| Error::RuntimeError(format!("Could not find {comp_name} on {:?}",entity),
                ))?;

            let id = w.components().get_id(dyn_comp.type_id()).unwrap();

            Ok(
                LuaComponent {
                    comp: LuaRef{
                        root: LuaRefBase::Component{ 
                            comp: refl, 
                            id,
                            entity: entity,
                            world: world.as_ref().clone()
                        }, 
                        path: Some("".to_string()), 
                        r: ReflectPtr::Const(dyn_comp)
                    }    
                }  
            )
        });

        // "spawn" => |_, world, ()| {
        //     let w = world.upgrade().unwrap();
        //     let w = &mut w.write();                
            
        //     Ok(LuaEntity::new(w.spawn().id()))
        // };

        // "new_script_entity" => |_, world, name: String| {
        //     let w = world.upgrade().unwrap();
        //     let w = &mut w.write();

        //     w.resource_scope(|w, r: Mut<AssetServer>| {
        //         let handle = r.load::<LuaFile, _>(&name);
        //         Ok(LuaEntity::new(
        //             w.spawn()
        //                 .insert(ScriptCollection::<crate::LuaFile> {
        //                     scripts: vec![Script::<LuaFile>::new(name, handle)],
        //                 })
        //                 .id(),
        //         ))
        //     })
        // };
       
    }   
}

#[derive(Clone)]
pub struct LuaComponent {
    pub(crate) comp: LuaRef,
}

impl_tealr_type!(LuaComponent);


impl Debug for LuaComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LuaComponent")
            .field("comp", &self.comp)
            .finish()
    }
}

impl TealData for LuaComponent {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |_, val, _a: Value| {
            val.comp.get(|s,_| {
                Ok(format!("{:#?}", PrintableReflect(s)))
            })
        });

        methods.add_meta_method_mut(MetaMethod::Index, |ctx, val, field: String| {
            let r = val.comp
                .path_ref(&field)
                .map_err(|_| Error::RuntimeError(format!("The field {field} does not exist on {val:?}")))?;

            Ok(r.convert_to_lua(ctx).unwrap())
        });

        methods.add_meta_method_mut(
            MetaMethod::NewIndex,
            |ctx, val, (field, new_val): (Value, Value)| {
                val.comp
                    .path_ref_lua(field)?
                    .apply_lua(ctx, new_val).unwrap();
                
                
                Ok(())
            },
        );
    }
}

pub struct LuaResource {
    pub(crate) res: LuaRef,
}
impl_tealr_type!(LuaResource);

impl TealData for LuaResource {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(_methods: &mut T) {}
}






#[cfg(test)]

mod test {
    use crate::{langs::mlu::{mlua,mlua::prelude::*},api::lua::LuaEntity, LuaEvent, Recipients, LuaComponent, LuaRef, LuaRefBase, get_type_data, ReflectPtr};
    use bevy::{prelude::*,reflect::TypeRegistryArc};
    use std::{sync::Arc};
    use parking_lot::RwLock;

    #[derive(Clone)]
    struct TestArg(LuaEntity);

    impl <'lua>ToLua<'lua> for TestArg {
        fn to_lua(self, ctx: &'lua Lua) -> Result<LuaValue<'lua>, mlua::Error> { 
            self.0.to_lua(ctx) 
        }
    }

    #[derive(Component,Reflect,Default)]
    #[reflect(Component)]
    struct TestComponent{
        mat3: Mat3,
    }

    #[test]
    #[should_panic]
    fn miri_test_components(){
        let world_arc = Arc::new(RwLock::new(World::new()));

        let mut component_ref1;
        let mut component_ref2;

        {
            let world = &mut world_arc.write();

            world.init_resource::<TypeRegistryArc>();
            let registry = world.resource_mut::<TypeRegistryArc>();
            registry.write().register::<TestComponent>();

            let tst_comp = TestComponent{
                mat3: Mat3::from_cols(Vec3::new(1.0,2.0,3.0),
                                    Vec3::new(4.0,5.0,6.0),
                                    Vec3::new(7.0,8.0,9.0))
            };

            let entity = world.spawn()
                            .insert(tst_comp)
                            .id();

            let refl: ReflectComponent = get_type_data(world, "TestComponent").unwrap();
            let refl_ref = refl.reflect(world,entity).unwrap();
            let ptr : ReflectPtr = ReflectPtr::Const(refl_ref);
            let id = world.components().get_id(refl_ref.type_id()).unwrap();

            component_ref1 = LuaRef{
                r: ptr,
                root: LuaRefBase::Component{ 
                    comp: refl, 
                    id,
                    entity,
                    world: Arc::downgrade(&world_arc),
                }, 
                path: Some("".to_string()), 
            };
            component_ref2 = component_ref1.clone();
        }

        component_ref1.get(|r1,_| {
            component_ref2.get(|r2,_|{
                let _ = r1.downcast_ref::<TestComponent>().unwrap().mat3 + r2.downcast_ref::<TestComponent>().unwrap().mat3;
            })
        });

        component_ref1.get_mut(|r1,_| {
            let _ = r1.downcast_ref::<TestComponent>().unwrap().mat3 * 2.0;
        });

        component_ref2.get_mut(|r2,_|{
            let _ = r2.downcast_ref::<TestComponent>().unwrap().mat3 * 2.0;
        });

        // invalid should panic here
        component_ref1.get_mut(|r1,_| {
            component_ref2.get(|r2,_|{
                r1.downcast_mut::<TestComponent>().unwrap().mat3 = r2.downcast_ref::<TestComponent>().unwrap().mat3;
            })
        });    
    }

    #[test]
    #[should_panic]
    fn miri_test_owned(){
       
        let mut mat = Mat3::from_cols(Vec3::new(1.0,2.0,3.0),
                                Vec3::new(4.0,5.0,6.0),
                                Vec3::new(7.0,8.0,9.0));
        
        let ptr : ReflectPtr = ReflectPtr::Mut(mat.col_mut(0));
        let valid = Arc::new(RwLock::new(()));

        let mut ref1 = LuaRef{
            r: ptr,
            root: LuaRefBase::LuaOwned{valid:Arc::downgrade(&valid)},
            path: None, 
        };
        let mut ref2 = ref1.clone();

        ref1.get(|r1,_| {
            ref2.get(|r2,_|{
                let _ = *r1.downcast_ref::<Vec3>().unwrap() + *r2.downcast_ref::<Vec3>().unwrap();
            })
        });

        ref1.get_mut(|r1,_| {
            let _ = *r1.downcast_ref::<Vec3>().unwrap() * 2.0;
        });

        ref2.get_mut(|r2,_|{
            let _ = *r2.downcast_ref::<Vec3>().unwrap() * 2.0;
        });

        drop(valid);
        drop(mat);

        // should panic since original value dropped
        ref1.get_mut(|r1,_| r1.downcast_mut::<Vec3>().unwrap()[1] = 2.0);
    }

}