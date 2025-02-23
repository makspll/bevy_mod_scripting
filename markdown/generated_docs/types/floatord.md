# FloatOrd

### FloatOrd

1. f32

## Description

>  A wrapper for floats that implements [`Ord`], [`Eq`], and [`Hash`] traits.
> 
>  This is a work around for the fact that the IEEE 754-2008 standard,
>  implemented by Rust's [`f32`] type,
>  doesn't define an ordering for [`NaN`](f32::NAN),
>  and `NaN` is not considered equal to any other `NaN`.
> 
>  Wrapping a float with `FloatOrd` breaks conformance with the standard
>  by sorting `NaN` as less than all other numbers and equal to any other `NaN`.

## Functions

| Function | Summary |
| --- | --- |
| `clone(_self)` | [No Documentation 🚧](./floatord/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./floatord/eq.md) |
| `ge(_self, other)` | [No Documentation 🚧](./floatord/ge.md) |
| `gt(_self, other)` | [No Documentation 🚧](./floatord/gt.md) |
| `le(_self, other)` | [No Documentation 🚧](./floatord/le.md) |
| `lt(_self, other)` | [No Documentation 🚧](./floatord/lt.md) |
| `neg(_self)` | [No Documentation 🚧](./floatord/neg.md) |