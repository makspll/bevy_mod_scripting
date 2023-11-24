use bevy::reflect::Reflect;

/// Script representable type with pass-by-value semantics
pub trait ScriptValue: Reflect + Clone {}
impl<T: Reflect + Clone> ScriptValue for T {}

#[macro_export]
macro_rules! ref_only_wrapper_methods {
    ($type_:path, $wrapper_name: ident) => {
        /// Creates a script reference pointing to the wrapped value.
        ///
        /// Depending on this value it may be a lua owned or reflect relative reference
        pub fn script_ref(
            &self,
            world_ptr: bevy_mod_scripting_core::world::WorldPointer,
        ) -> $crate::script_ref::ScriptRef {
            match self {
                Self::Owned(val) => unsafe {
                    // safety:
                    // - valid is dropped when the value goes out of scope, so won't be dangling
                    // - using the valid lock means no incorrect aliasing may occur
                    // - the pointer points to base of the reference
                    // invariants are upheld
                    $crate::script_ref::ScriptRef::new_script_ref(
                        ::std::sync::Arc::downgrade(val),
                        world_ptr,
                    )
                },
                Self::Ref(ref_) => ref_.clone(),
            }
        }

        pub fn new(b: $type_) -> Self {
            Self::Owned(::std::sync::Arc::new($crate::parking_lot::RwLock::new(b)))
        }

        pub fn new_ref(b: $crate::script_ref::ScriptRef) -> Self {
            Self::Ref(b)
        }

        /// Perform an operation on the base type and optionally retrieve something by value
        /// may require a read lock on the world in case this is a reference
        pub fn val<G, F>(&self, accessor: F) -> Result<G, $crate::error::ReflectionError>
        where
            F: FnOnce(&$type_) -> G,
        {
            match self {
                Self::Owned(v) => Ok(accessor(&v.read())),
                Self::Ref(v) => v.get(|s| accessor(s.downcast_ref::<$type_>().unwrap())),
            }
        }

        pub fn val_mut<G, F>(&mut self, accessor: F) -> Result<G, $crate::error::ReflectionError>
        where
            F: FnOnce(&mut $type_) -> G,
        {
            match self {
                Self::Owned(v) => Ok(accessor(&mut *v.write())),
                Self::Ref(v) => v.get_mut(|s| accessor(s.downcast_mut::<$type_>().unwrap())),
            }
        }

        /// Applies Self to another ScriptRef.
        /// may require a write lock on the world
        pub fn apply_self_to_base(
            &self,
            other: &mut $crate::script_ref::ScriptRef,
        ) -> Result<(), $crate::error::ReflectionError> {
            match self {
                Self::Owned(v) => {
                    other.get_mut(|other| other.apply(&mut *v.write()))?;
                    Ok(())
                }
                Self::Ref(v) => {
                    // if we are a ScriptRef, we have to be careful with borrows
                    // to avoid deadlock
                    // we take advantage of the fact we know the expected type
                    other.apply(v)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_wrapper{
    ($type_:path, $wrapper_name:ident) => {
        #[allow(clippy::large_enum_variant)]
        #[doc=concat!("A script wrapper for the type `",stringify!($type_),"`")]
        #[derive(Clone)]
        pub enum $wrapper_name{
            Owned(::std::sync::Arc<$crate::parking_lot::RwLock<$type_>>),
            Ref($crate::script_ref::ScriptRef),
        }

        /// Safety: we make this sync via RwLock<()> assuming invariants are upheld
        // unsafe impl Sync for $wrapper_name {}

        impl Drop for $wrapper_name {
            fn drop(&mut self) {
                match self {
                    Self::Owned(v) => {
                        if v.is_locked() {
                            panic!(
                                "Something is referencing a lua value and it's about to go out of scope!"
                            );
                        }
                    }
                    Self::Ref(_) => {}
                }
            }
        }
    }
}

#[macro_export]
macro_rules! make_script_wrapper {
    ($type_:path as $wrapper_name:ident with Clone) => {
        $crate::define_wrapper!($type_, $wrapper_name);
        impl $wrapper_name {
            $crate::ref_only_wrapper_methods!($type_, $wrapper_name);

            /// retrieves the underlying value by cloning it
            pub fn inner(&self) -> Result<$type_, $crate::error::ReflectionError>
            where
                $type_: Clone,
            {
                self.val(|s| s.clone())
            }
        }
    };
    ($type_:path as $wrapper_name:ident) => {
        $crate::define_wrapper!($type_, $wrapper_name);
        impl $wrapper_name {
            $crate::ref_only_wrapper_methods!($type_, $wrapper_name);
        }
    };
}

// // TODO: look at this when rust gets better
// // Oh boy, there is no way in current rust to implement this
// // We need trait specialization.
// // This isn't even possible if implemented without generics since then
// // we get a compile error from mlua about how `Clone` may be implemented on the wrapped type in the feature
// // :C
// // impl <'lua, T : ScriptReference + !Clone> FromLua<'lua> for LuaWrapper<T> {
// //     fn from_lua(lua_value: tealr::mlu::mlua::Value<'lua>, lua: &'lua tealr::mlu::mlua::Lua) -> tealr::mlu::mlua::Result<Self> {
// //         match lua_value {
// //             tealr::mlu::mlua::Value::UserData(ud) => {

// //             match ud.borrow::<LuaWrapper<T>>()?{
// //                 // here we need to move out of the value in the lua world
// //                 LuaWrapper::Owned(_, _) => ud.take(),
// //                 // we can copy fine here
// //                 LuaWrapper::Ref(ref_) => Ok(LuaWrapper::new_ref(&ref_)),
// //             }
// //         }
// //             _ => Err(tealr::mlu::mlua::Error::FromLuaConversionError {
// //                 from: lua_value.type_name(),
// //                 to: "userdata",
// //                 message: None,
// //             })
// //         }
// //     }
// // }
