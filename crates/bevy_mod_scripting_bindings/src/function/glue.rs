//! Glue traits for "unwrapping" bindings to their raw form

use std::{mem, ptr};

use variadics_please::all_tuples;

use crate::{Mut, Ref, Val};

// glue.rs (or wherever)
mod sealed {
    // Sealed relationship for a *pair* (L -> R).
    // This prevents external crates from implementing CanTransmute for arbitrary pairs.
    pub trait Sealed<Rhs> {}
}

// Public trait: L can transmute to R
pub trait CanTransmute<Rhs>: sealed::Sealed<Rhs> {
    fn safe_transmute(self) -> Rhs;
}

// === Implementations ===

// =======================
// Primitive / wrapper identity impls
// =======================

// For string slices — generic over any lifetime
impl<'a> sealed::Sealed<&'a str> for &'a str {}
impl<'a> CanTransmute<&'a str> for &'a str {
    fn safe_transmute(self) -> &'a str {
        self
    }
}

// Identity for primitives / base types
macro_rules! impl_identity {
    ($($t:ty),*) => {
        $(
            impl sealed::Sealed<$t> for $t {}
            impl CanTransmute<$t> for $t {
                fn safe_transmute(self) -> $t { self }
            }
        )*
    };
}

// Add the types you need
impl_identity!(
    (),
    usize,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    i8,
    i16,
    i32,
    i64,
    i128,
    bool,
    char,
    f32,
    f64,
    String
);

// === Wrapper -> Inner ===

// Ref<'a, T> -> &'b T
impl<'a, 'b, T> sealed::Sealed<&'b T> for Ref<'a, T> where 'a: 'b {}
impl<'a, 'b, T> CanTransmute<&'b T> for Ref<'a, T>
where
    'a: 'b,
{
    fn safe_transmute(self) -> &'b T {
        let Ref(inner) = self;
        inner
    }
}

// Mut<'a, T> -> &'b mut T
impl<'a, 'b, T> sealed::Sealed<&'b mut T> for Mut<'a, T> where 'a: 'b {}
impl<'a, 'b, T> CanTransmute<&'b mut T> for Mut<'a, T>
where
    'a: 'b,
{
    fn safe_transmute(self) -> &'b mut T {
        let Mut(inner) = self;
        inner
    }
}

// Val<T> -> T
impl<T> sealed::Sealed<T> for Val<T> {}
impl<T> CanTransmute<T> for Val<T> {
    fn safe_transmute(self) -> T {
        unsafe {
            let out = std::ptr::read(&self as *const Val<T> as *const T);
            std::mem::forget(self);
            out
        }
    }
}

// =======================
// Recursive container impls
// =======================

// Vec<T>
impl<T, U> sealed::Sealed<Vec<U>> for Vec<T> where T: CanTransmute<U> {}
impl<T, U> CanTransmute<Vec<U>> for Vec<T>
where
    T: CanTransmute<U>,
{
    fn safe_transmute(self) -> Vec<U> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T, U> sealed::Sealed<Option<U>> for Option<T> where T: CanTransmute<U> {}

// Option<T>
impl<T, U> CanTransmute<Option<U>> for Option<T>
where
    T: CanTransmute<U>,
{
    fn safe_transmute(self) -> Option<U> {
        unsafe {
            // Bitwise move the whole Option<T> into Option<U>
            let out = ptr::read(&self as *const Option<T> as *const Option<U>);
            mem::forget(self);
            out
        }
    }
}

// Result<T, E>
impl<T, E, U, F> sealed::Sealed<Result<U, F>> for Result<T, E>
where
    T: CanTransmute<U>,
    E: CanTransmute<F>,
{
}
impl<T, E, U, F> CanTransmute<Result<U, F>> for Result<T, E>
where
    T: CanTransmute<U>,
    E: CanTransmute<F>,
{
    fn safe_transmute(self) -> Result<U, F> {
        unsafe {
            let out = ptr::read(&self as *const Result<T, E> as *const Result<U, F>);
            mem::forget(self);
            out
        }
    }
}
impl<T, U, const N: usize> sealed::Sealed<[U; N]> for [T; N] where T: CanTransmute<U> {}

impl<T, U, const N: usize> CanTransmute<[U; N]> for [T; N]
where
    T: CanTransmute<U>,
{
    fn safe_transmute(self) -> [U; N] {
        use core::{mem, ptr};

        // Move `self` into a raw pointer
        let ptr = &self as *const [T; N] as *const [U; N];

        // Prevent drop of `self`
        let out = unsafe { ptr::read(ptr) };

        mem::forget(self);

        out
    }
}

pub fn safe_transmute<In, Out>(input: In) -> Out
where
    In: CanTransmute<Out> + Sized,
    Out: Sized,
{
    input.safe_transmute()
}

impl<T> sealed::Sealed<Val<T>> for T {}
impl<T> CanTransmute<Val<T>> for T {
    fn safe_transmute(self) -> Val<T> {
        Val(self)
    }
}

impl<'a, T> sealed::Sealed<Ref<'a, T>> for &'a T {}
impl<'a, T> CanTransmute<Ref<'a, T>> for &'a T {
    fn safe_transmute(self) -> Ref<'a, T> {
        Ref(self)
    }
}

impl<'a, T> sealed::Sealed<Mut<'a, T>> for &'a mut T {}
impl<'a, T> CanTransmute<Mut<'a, T>> for &'a mut T {
    fn safe_transmute(self) -> Mut<'a, T> {
        Mut(self)
    }
}

macro_rules! impl_can_transmute_tuple {
    // Each entry gives us three identifiers: T, U, t
    ($(($T:ident, $U:ident, $t:ident)),*) => {
        impl< $( $T: CanTransmute<$U>, $U ),* >
            sealed::Sealed<( $( $U ),* )>
            for ( $( $T ),* )
        {}

        impl< $( $T: CanTransmute<$U>, $U ),* >
            CanTransmute<( $( $U ),* )>
            for ( $( $T ),* )
        {
            fn safe_transmute(self) -> ( $( $U ),* ) {
                let ( $( $t ),* ) = self;
                ( $( $t.safe_transmute() ),* )
            }
        }
    }
}

all_tuples!(impl_can_transmute_tuple, 2, 15, T, U, t);

#[doc(hidden)]
pub const fn assert_layout_equiv<T, W>() {
    // SAFETY: W must be #[repr(transparent)] over T
    if std::mem::size_of::<W>() != std::mem::size_of::<T>() {
        panic!("Invariant broken, size of types is differnt.",)
    }
    if std::mem::align_of::<W>() != std::mem::align_of::<T>() {
        panic!("Invariant broken, alignment of types is differnt.",)
    }
}

#[doc(hidden)]
#[allow(dead_code)]
const SAFETY_ASSERTION: () = {
    assert_layout_equiv::<Mut<usize>, &mut usize>();
    assert_layout_equiv::<Ref<usize>, &usize>();
    assert_layout_equiv::<Val<usize>, usize>();
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_identity_transmute() {
        let x: u32 = safe_transmute::<u32, u32>(123);
        assert_eq!(x, 123);
    }

    #[test]
    fn test_val_to_inner() {
        let v = Val(vec![1, 2, 3]);
        let out: Vec<i32> = safe_transmute(v);
        assert_eq!(out, vec![1, 2, 3]);
    }

    #[test]
    fn test_ref_to_ref() {
        let x = 10;
        let r = Ref(&x);

        let out: &i32 = safe_transmute(r);
        assert_eq!(*out, 10);
    }

    #[test]
    fn test_mut_to_mut() {
        let mut x = 5;
        let m = Mut(&mut x);

        let out: &mut i32 = safe_transmute(m);
        *out = 99;
        assert_eq!(x, 99);
    }

    #[test]
    fn test_vec_of_val() {
        let v = vec![Val(1), Val(2), Val(3)];
        let out: Vec<i32> = safe_transmute(v);
        assert_eq!(out, vec![1, 2, 3]);
    }

    #[test]
    fn test_option_of_val_some() {
        let v = Some(Val(42));
        let out: Option<i32> = safe_transmute(v);
        assert_eq!(out, Some(42));
    }

    #[test]
    fn test_option_of_val_none() {
        let v: Option<Val<i32>> = None;
        let out: Option<i32> = safe_transmute(v);
        assert_eq!(out, None);
    }

    #[test]
    fn test_result_of_val_ok() {
        let r: Result<Val<i32>, Val<&str>> = Ok(Val(10));
        let out: Result<i32, &str> = safe_transmute(r);
        assert!(matches!(out, Ok(10)));
    }

    #[test]
    fn test_result_of_val_err() {
        let r: Result<Val<i32>, Val<&str>> = Err(Val("fail"));
        let out: Result<i32, &str> = safe_transmute(r);
        assert!(matches!(out, Err("fail")));
    }

    #[test]
    fn test_vec_of_option_of_val() {
        let v = vec![Some(Val(1)), None, Some(Val(3))];
        let out: Vec<Option<i32>> = safe_transmute(v);
        assert_eq!(out, vec![Some(1), None, Some(3)]);
    }

    #[test]
    fn test_nested_result_vec_val() {
        let r: Result<Vec<Val<i32>>, Val<&str>> = Ok(vec![Val(1), Val(2)]);
        let out: Result<Vec<i32>, &str> = safe_transmute(r);
        assert_eq!(out.unwrap(), vec![1, 2]);
    }
}
