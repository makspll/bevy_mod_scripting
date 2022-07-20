use std::marker::PhantomData;

use bevy::reflect::Reflect;

/// A compile time recipe to access subfields of a reflect value
/// 
/// Used in the case where we might want to refer to something using a [`crate::ScriptRef`] but have that script ref
/// always refer to a sub component of the data.
pub trait SubReflect : 'static + Send + Sync {
    fn sub_ref<'a>(&self, ref_ : &'a dyn Reflect) -> &'a dyn Reflect;
    fn sub_ref_mut<'a>(&self, ref_ : &'a mut dyn Reflect) -> &'a mut dyn Reflect;
}


#[derive(Clone,Default)]
pub struct CompositeSubReflect {
    // most of these will be very short, people don't make many nested hashmaps vecs etc.
    accesses: Vec<(fn(&dyn Reflect) -> &dyn Reflect,
                        fn(&mut dyn Reflect) -> &mut dyn Reflect)>
}

impl CompositeSubReflect {
    /// pushes another sub reflect level access to the end of this access. 
    /// 
    /// The most recent sub access added will be executed last.
    pub fn push(&mut self, get: fn(&dyn Reflect) -> &dyn Reflect, get_mut: fn(&mut dyn Reflect) -> &mut dyn Reflect){
        self.accesses.push((get,get_mut));
    }

    /// Creates a new composite sub reflect, from another one and a relative sub access to be added to the end of it.
    pub fn new_sub(&self, get: fn(&dyn Reflect) -> &dyn Reflect, get_mut: fn(&mut dyn Reflect) -> &mut dyn Reflect) -> Self{
        let mut accesses = self.accesses.clone();

        accesses.push((get,get_mut));

        Self{
            accesses,
        }
    }
}

impl SubReflect for CompositeSubReflect {
    #[inline(always)]
    fn sub_ref<'a>(&self, ref_ : &'a dyn Reflect) -> &'a dyn Reflect {
        if self.accesses.is_empty() {
            return ref_
        }

        let first = self.accesses.first()
            .map(|(getter,_)| (*getter)(ref_))
            .unwrap();

        self.accesses[1..].iter().fold(first, |a,(getter,_)| {
            getter(a)
        })
    }

    #[inline(always)]
    fn sub_ref_mut<'a>(&self, ref_ : &'a mut dyn Reflect) -> &'a mut dyn Reflect {
        if self.accesses.is_empty() {
            return ref_
        }

        let first = self.accesses.first()
            .map(|(_,getter)| (*getter)(ref_))
            .unwrap();

        self.accesses[1..].iter().fold(first, |a,(_,getter)| {
            getter(a)
        })    
    }
}

#[derive(Clone,Copy)]
pub struct IdentitySubReflect;
impl SubReflect for IdentitySubReflect{
    fn sub_ref<'a>(&self, ref_ : &'a dyn Reflect) -> &'a dyn Reflect {
        ref_
    }

    fn sub_ref_mut<'a>(&self, ref_ : &'a mut dyn Reflect) -> &'a mut dyn Reflect{
        ref_
    }
}
