use bevy::prelude::*;
use parking_lot::RwLock;
use std::fmt::Debug;
use std::{borrow::Cow, sync::Weak};

use bevy_mod_scripting_core::world::WorldPointer;

use crate::{
    error::ReflectionError,
    sub_reflect::{ReflectBase, ReflectPath, ReflectPathElem},
};

pub enum ScriptRefBase {}

/// A reference to a rust type available from some script language.
/// References can be either to rust or script managed values (created either on the bevy or script side).
/// but also to any subfield of those values (All pointed to values must support `reflect`).
/// Each reference holds a reflection path from the root.
///
/// Automatically converts to most convenient lua representation.
/// See [`ScriptRef::into_lua`]
#[derive(Clone, Debug)]
pub struct ScriptRef {
    /// The reflection path from the root
    pub(crate) path: ReflectPath,
    pub(crate) world_ptr: WorldPointer,
}

impl ScriptRef {
    /// Safely creates a new base component reference
    pub fn new_component_ref(
        comp: ReflectComponent,
        entity: Entity,
        world_ptr: WorldPointer,
    ) -> Self {
        Self {
            path: ReflectPath::new(ReflectBase::Component { comp, entity }),
            world_ptr,
        }
    }

    pub fn new_resource_ref(res: ReflectResource, world_ptr: WorldPointer) -> Self {
        Self {
            path: ReflectPath::new(ReflectBase::Resource { res }),
            world_ptr,
        }
    }

    /// Creates a reference to a script owned value
    ///
    /// # Safety
    /// You must ensure that the following holds:
    /// - base_ptr can be dereferenced
    pub unsafe fn new_script_ref(
        ptr: ReflectPtr,
        valid: Weak<RwLock<()>>,
        world_ptr: WorldPointer,
    ) -> Self {
        Self {
            path: ReflectPath::new(ReflectBase::ScriptOwned { ptr, valid }),
            world_ptr,
        }
    }

    /// Creates a new script reference which points to a sub component of the original data,
    /// This also updates the pointer
    pub fn sub_ref(&self, elem: ReflectPathElem) -> ScriptRef {
        let path = self.path.new_sub(elem);

        Self {
            path,
            ..self.clone()
        }
    }

    /// Retrieves the underlying `dyn Reflect` reference and applies function which can retrieve a value.
    /// Panics if the reference is invalid or world is already borrowed mutably.
    #[inline(always)]
    pub fn get<O, F>(&self, f: F) -> Result<O, ReflectionError>
    where
        F: FnOnce(&dyn Reflect) -> O,
    {
        self.path.get(self.world_ptr.clone(), f)
    }

    pub fn get_typed<T, O, F>(&self, f: F) -> Result<O, ReflectionError>
    where
        F: FnOnce(&T) -> O,
        T: Reflect,
    {
        self.path.get(self.world_ptr.clone(), |reflect| {
            (f)(reflect.downcast_ref::<T>().unwrap_or_else(|| {
                panic!(
                    "Expected `{}` found `{}`",
                    ::std::any::type_name::<T>(),
                    reflect.type_name()
                )
            }))
        })
    }

    /// Retrieves the underlying `dyn Reflect` reference and applies function which can retrieve a value.
    /// If this is a component it is marked as changed.
    /// Panics if the reference is invalid or if the world/value is already borrowed or if r is not a mutable pointer.
    #[inline(always)]
    pub fn get_mut<O, F>(&mut self, f: F) -> Result<O, ReflectionError>
    where
        F: FnOnce(&mut dyn Reflect) -> O,
    {
        self.path.get_mut(self.world_ptr.clone(), f)
    }

    pub fn get_mut_typed<T, O, F>(&mut self, f: F) -> Result<O, ReflectionError>
    where
        F: FnOnce(&mut T) -> O,
        T: Reflect,
    {
        self.path.get_mut(self.world_ptr.clone(), |reflect| {
            (f)(reflect.downcast_mut().unwrap())
        })
    }

    /// applies another [`ScriptRef`] to self by carefuly acquiring locks and cloning if necessary.
    ///
    /// This is semantically equivalent to the [`Reflect::apply`] method.
    /// If you know the type of this value use [`Self::apply_luaref_typed`] since it avoids double cloning and allocating
    pub fn apply(&mut self, other: &ScriptRef) -> Result<(), ReflectionError> {
        // sadly apply already performs a clone for value types, so this incurs
        // a double clone in some cases TODO: is there another way ?
        // can we avoid the box ?
        let cloned = other.get(|s| s.clone_value())?;

        // safety: we already called `get` so reference must be valid
        self.get_mut(|s| s.apply(&*cloned))
    }

    /// Unlike apply this method expects the other type to be identical. Does not allocate so is likely to be faster than apply, uses direct assignment.
    /// If you have a concrete value use [`Self::set_val`](TypedScriptRef) unstead
    pub fn set<T>(&mut self, other: &Self) -> Result<(), ReflectionError>
    where
        T: Reflect + Clone,
    {
        let other: T = other.get_typed(|s: &T| s.clone())?;
        self.get_mut_typed(|s| *s = other)
    }

    /// Version of [`Self::set`](TypedScriptRef) which directly accepts a `T` value
    pub fn set_val<T>(&mut self, other: T) -> Result<(), ReflectionError>
    where
        T: Reflect,
    {
        self.get_mut_typed(|s| *s = other)
    }
}

/// A version of index for returning values instead of references
pub trait ValueIndex<Idx> {
    type Output;

    fn index(&self, index: Idx) -> Self::Output;
}

impl ValueIndex<usize> for ScriptRef {
    type Output = Self;

    fn index(&self, index: usize) -> Self::Output {
        self.sub_ref(ReflectPathElem::IndexAccess(index))
    }
}

impl ValueIndex<Cow<'static, str>> for ScriptRef {
    type Output = Self;

    fn index(&self, index: Cow<'static, str>) -> Self::Output {
        self.sub_ref(ReflectPathElem::FieldAccess(index))
    }
}

/// A pointer wrapper with some extra safety information about mutability.
#[derive(Clone, Copy, Debug)]
pub struct ReflectPtr {
    /// the pointer to the data
    ptr: *const dyn Reflect,
    /// a safety bit, if false, cannot cast as mutable pointer
    is_mut: bool,
}

impl From<*const dyn Reflect> for ReflectPtr {
    fn from(ptr: *const dyn Reflect) -> Self {
        Self { ptr, is_mut: false }
    }
}

impl From<*mut dyn Reflect> for ReflectPtr {
    fn from(ptr: *mut dyn Reflect) -> Self {
        Self { ptr, is_mut: true }
    }
}

impl ReflectPtr {
    /// dereference the pointer as an immutable reference.
    /// The caller must ensure the pointer is valid.
    /// # Safety
    /// pointer must point to valid non-dangling data, aliasing rules must be upheld
    #[inline(always)]
    pub unsafe fn const_ref<'a>(self) -> &'a dyn Reflect {
        &*self.ptr
    }

    /// Dereference the pointer as a mutable reference,
    ///
    /// The caller must ensure the pointer is valid. Returns None if the underlying pointer is const
    /// # Safety
    /// pointer must point to valid non-dangling data, aliasing rules must be upheld
    pub unsafe fn mut_ref<'a>(self) -> Option<&'a mut dyn Reflect> {
        if self.is_mut {
            Some(&mut *(self.ptr as *mut dyn Reflect))
        } else {
            None
        }
    }

    /// Maps this pointer to another one with one of two funtions depending on if mutable access is available
    /// # Safety
    /// pointer must point to valid non-dangling data, aliasing rules must be upheld
    pub unsafe fn map(
        self,
        get: fn(&dyn Reflect) -> &dyn Reflect,
        get_mut: fn(&mut dyn Reflect) -> &mut dyn Reflect,
    ) -> Self {
        if self.is_mut {
            (get_mut(self.mut_ref().unwrap()) as *const dyn Reflect).into()
        } else {
            (get(self.const_ref()) as *const dyn Reflect).into()
        }
    }
}

/// safe since Reflect values have to be Send
unsafe impl Send for ReflectPtr {}
/// safe since Reflect values have to be Sync
unsafe impl Sync for ReflectPtr {}

/// A value representing a type which has no special UserData implementation,
/// It exposes the much less convenient reflect interface of the underlying type.
#[derive(Clone, Debug)]
pub struct ReflectedValue {
    pub(crate) ref_: ScriptRef,
}

impl From<ReflectedValue> for ScriptRef {
    fn from(ref_: ReflectedValue) -> Self {
        ref_.ref_
    }
}

// #[cfg(test)]
// mod test {
//     use crate::{
//         api::lua::bevy::LuaEntity,
//         langs::mlu::{mlua, mlua::prelude::*},
//         ReflectPtr, ScriptRef,
//     };
//     use bevy::{prelude::*, reflect::TypeRegistryArc};
//     use parking_lot::RwLock;
//     use std::sync::Arc;

//     #[derive(Clone)]
//     struct TestArg(LuaEntity);

// //     impl<'lua> IntoLua<'lua> for TestArg {
////          fn into_lua(self, ctx: &'lua Lua) -> Result<LuaValue<'lua>, mlua::Error> {
//             self.0.into_lua(ctx)
//         }
//     }

//     #[derive(Component, Reflect, Default)]
//     #[reflect(Component)]
//     struct TestComponent {
//         mat3: Mat3,
//     }

//     #[test]
//     #[should_panic]
//     fn miri_test_components() {
//         let world_arc = Arc::new(RwLock::new(World::new()));

//         let mut component_ref1;
//         let mut component_ref2;

//         {
//             let world = &mut world_arc.write();

//             world.init_resource::<TypeRegistryArc>();
//             let registry = world.resource_mut::<TypeRegistryArc>();
//             registry.write().register::<TestComponent>();

//             let tst_comp = TestComponent {
//                 mat3: Mat3::from_cols(
//                     Vec3::new(1.0, 2.0, 3.0),
//                     Vec3::new(4.0, 5.0, 6.0),
//                     Vec3::new(7.0, 8.0, 9.0),
//                 ),
//             };

//             let refl = registry
//                 .read()
//                 .get_with_short_name("TestComponent")
//                 .and_then(|registration| registration.data::<ReflectComponent>())
//                 .unwrap()
//                 .clone();

//             let entity = world.spawn().insert(tst_comp).id();

//             let refl_ref = refl.reflect(world, entity).unwrap();
//             let _ptr: ReflectPtr = (refl_ref as *const dyn Reflect).into();

//             component_ref1 = ScriptRef::new_component_ref(refl, entity, Arc::downgrade(&world_arc));
//             component_ref2 = component_ref1.clone();
//         }
//         // TODO: reformat this test now that we return results instead of panicking
//         component_ref1
//             .get(|r1| {
//                 component_ref2
//                     .get(|r2| {
//                         let _ = r1.downcast_ref::<TestComponent>().unwrap().mat3
//                             + r2.downcast_ref::<TestComponent>().unwrap().mat3;
//                     })
//                     .unwrap()
//             })
//             .unwrap();

//         component_ref1
//             .get_mut(|r1| {
//                 let _ = r1.downcast_ref::<TestComponent>().unwrap().mat3 * 2.0;
//             })
//             .unwrap();

//         component_ref2
//             .get_mut(|r2| {
//                 let _ = r2.downcast_ref::<TestComponent>().unwrap().mat3 * 2.0;
//             })
//             .unwrap();

//         // invalid should panic here
//         component_ref1
//             .get_mut(|r1| {
//                 component_ref2
//                     .get(|r2| {
//                         r1.downcast_mut::<TestComponent>().unwrap().mat3 =
//                             r2.downcast_ref::<TestComponent>().unwrap().mat3;
//                     })
//                     .unwrap()
//             })
//             .unwrap();
//     }

// #[test]
// #[should_panic]
// fn miri_test_owned(){

//     let mut mat = Mat3::from_cols(Vec3::new(1.0,2.0,3.0),
//                             Vec3::new(4.0,5.0,6.0),
//                             Vec3::new(7.0,8.0,9.0));

//     let ptr : ReflectPtr = (mat.col_mut(0) as *mut dyn Reflect).into();
//     let valid = Arc::new(RwLock::new(()));

//     let mut ref1 = unsafe{ ScriptRef::new_script_ref(ptr, valid)
//         ScriptRefBase::ScriptOwned{valid:Arc::downgrade(&valid)},
//         None,
//         ptr.into()
//     )};
//     let mut ref2 = ref1.clone();

//     ref1.get(|r1| {
//         ref2.get(|r2|{
//             let _ = *r1.downcast_ref::<Vec3>().unwrap() + *r2.downcast_ref::<Vec3>().unwrap();
//         })
//     });

//     ref1.get_mut(|r1,_| {
//         let _ = *r1.downcast_ref::<Vec3>().unwrap() * 2.0;
//     });

//     ref2.get_mut(|r2,_|{
//         let _ = *r2.downcast_ref::<Vec3>().unwrap() * 2.0;
//     });

//     drop(valid);
//     drop(mat);

//     // should panic since original value dropped
//     ref1.get_mut(|r1,_| r1.downcast_mut::<Vec3>().unwrap()[1] = 2.0);
// }
// }
