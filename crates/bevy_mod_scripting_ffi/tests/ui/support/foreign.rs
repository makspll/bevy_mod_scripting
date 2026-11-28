//! Stand-ins for foreign types (tests are separate crates, so they can't overlay glam types directly).
#![allow(dead_code)]

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mat3 {
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
}
impl Mat3 {
    pub const fn from_cols(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }
}

/// Real padding: bytes 1..4 are uninitialized.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Foo {
    pub a: u8,
    pub b: u32,
}
impl Foo {
    pub const fn new(a: u8, b: u32) -> Self {
        Self { a, b }
    }
}

/// Nested overlay with padding inside the child.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FooPair {
    pub first: Foo,
    pub second: Foo,
}
impl FooPair {
    pub const fn new(first: Foo, second: Foo) -> Self {
        Self { first, second }
    }
}

/// Private fields: unreachable from the macro call site.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Handle {
    id: u32,
    generation: u32,
}
impl Handle {
    pub const fn new(id: u32, generation: u32) -> Self {
        Self { id, generation }
    }
}

/// Fields reached only through `Deref`, like glam's SIMD `Vec3A`.
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Vec3A([f32; 4]);
#[repr(C)]
pub struct Xyz {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3A {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z, z])
    }
}
impl core::ops::Deref for Vec3A {
    type Target = Xyz;
    fn deref(&self) -> &Xyz {
        // SAFETY: Xyz is a repr(C) prefix of [f32; 4]
        unsafe { &*(self as *const Self).cast() }
    }
}

/// Contains an over-aligned field: `dir` sits at 16, the struct has align 16.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Velocity {
    pub speed: f32,
    pub dir: Vec3A,
}
impl Velocity {
    pub const fn new(speed: f32, dir: Vec3A) -> Self {
        Self { speed, dir }
    }
}

/// Bools: one non-zero value, so told apart by several probes.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Flags {
    pub a: bool,
    pub b: bool,
    pub c: bool,
}
impl Flags {
    pub const fn new(a: bool, b: bool, c: bool) -> Self {
        Self { a, b, c }
    }
}

/// Mixed sizes: `c` at 0, `a` at 2, `b` at 3.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mixed {
    pub c: u16,
    pub a: u8,
    pub b: u8,
}
impl Mixed {
    pub const fn new(a: u8, b: u8, c: u16) -> Self {
        Self { c, a, b }
    }
}

#[derive(Clone)]
pub struct Named {
    pub name: String,
}
impl Named {
    pub const fn new() -> Self {
        Self {
            name: String::new(),
        }
    }
}
