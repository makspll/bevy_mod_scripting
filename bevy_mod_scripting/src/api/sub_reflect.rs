use std::marker::PhantomData;

use bevy::reflect::Reflect;

/// A compile time recipe to access subfields of a reflect value
/// 
/// Used in the case where we might want to refer to something using a [`crate::ScriptRef`] but have that script ref
/// always refer to a sub component of the data.
/// 
/// This type-level magic allows us to avoid any runtime costs of storing, allocating and combining getter closures.
pub trait SubReflect : 'static + Send + Sync + Clone {
    fn sub_ref<'a>(ref_ : &'a dyn Reflect) -> &'a dyn Reflect;
    fn sub_ref_mut<'a>(ref_ : &'a mut dyn Reflect) -> &'a mut dyn Reflect;
}

#[derive(Clone)]
pub struct NestedSubReflect<T : SubReflect> {
    ph : PhantomData<T>
}

impl <T : SubReflect>SubReflect for NestedSubReflect<T> {
    #[inline(always)]
    fn sub_ref<'a>(ref_ : &'a dyn Reflect) -> &'a dyn Reflect {
        T::sub_ref(ref_)
    }

    #[inline(always)]
    fn sub_ref_mut<'a>(ref_ : &'a mut dyn Reflect) -> &'a mut dyn Reflect {
        T::sub_ref_mut(ref_)
    }
}

#[derive(Clone)]
pub struct IdentitySubReflect;
impl SubReflect for IdentitySubReflect{
    fn sub_ref<'a>(ref_ : &'a dyn Reflect) -> &'a dyn Reflect {
        ref_
    }

    fn sub_ref_mut<'a>(ref_ : &'a mut dyn Reflect) -> &'a mut dyn Reflect{
        ref_
    }
}
