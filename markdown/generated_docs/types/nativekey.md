# NativeKey

### Unidentified

### Android

1. u32

### MacOS

1. u16

### Windows

1. u16

### Xkb

1. u32

### Web

1. smol\_str::SmolStr

## Description

>  Contains the platform-native logical key identifier, known as keysym.
> 
>  Exactly what that means differs from platform to platform, but the values are to some degree
>  tied to the currently active keyboard layout. The same key on the same keyboard may also report
>  different values on different platforms, which is one of the reasons this is a per-platform
>  enum.
> 
>  This enum is primarily used to store raw keysym when Winit doesn't map a given native logical
>  key identifier to a meaningful [`Key`] variant. This lets you use [`Key`], and let the user
>  define keybinds which work in the presence of identifiers we haven't mapped for you yet.

## Functions

| Function | Summary |
| --- | --- |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./nativekey/assert_receiver_is_total_eq.md) |
| `clone(_self)` | [No Documentation 🚧](./nativekey/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./nativekey/eq.md) |