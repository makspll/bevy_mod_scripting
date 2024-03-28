#[macro_export]
macro_rules! ref_only_wrapper_methods {
    ($type_:path, $wrapper_name: ident) => {
        /// Creates a script reference pointing to the wrapped value.
        ///
        /// Depending on this value it may be a lua owned or reflect relative reference
        pub fn reflect_ref(
            &self,
            world_ptr: bevy_mod_scripting_core::world::WorldPointer,
        ) -> $crate::script_ref::ReflectReference {
            match self {
                Self::Owned(val) => $crate::script_ref::ReflectReference::new_script_ref(
                    ::std::sync::Arc::downgrade(val),
                    world_ptr,
                ),
                Self::Ref(ref_) => ref_.clone(),
            }
        }

        pub fn new(b: $type_) -> Self {
            Self::Owned(::std::sync::Arc::new($crate::parking_lot::RwLock::new(b)))
        }

        pub fn new_ref(b: $crate::script_ref::ReflectReference) -> Self {
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

        /// Applies Self to another ReflectReference.
        /// may require a write lock on the world
        pub fn apply_self_to_base(
            &self,
            other: &mut $crate::script_ref::ReflectReference,
        ) -> Result<(), $crate::error::ReflectionError> {
            match self {
                Self::Owned(v) => {
                    other.get_mut(|other| other.apply(&mut *v.write()))?;
                    Ok(())
                }
                Self::Ref(v) => {
                    // if we are a ReflectReference, we have to be careful with borrows
                    // to avoid deadlock
                    // we take advantage of the fact we know the expected type
                    other.apply(v)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_wrapper {
    ($type_:path, $wrapper_name:ident) => {
        #[allow(clippy::large_enum_variant)]
        #[doc=concat!("A script wrapper for the type `",stringify!($type_),"`")]
        #[derive(Clone)]
        pub enum $wrapper_name {
            Owned(::std::sync::Arc<$crate::parking_lot::RwLock<$type_>>),
            Ref($crate::script_ref::ReflectReference),
        }
    };
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
