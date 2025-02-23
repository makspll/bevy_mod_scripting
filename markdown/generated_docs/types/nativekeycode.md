# NativeKeyCode

### Unidentified

### Android

1. u32

### MacOS

1. u16

### Windows

1. u16

### Xkb

1. u32

## Description

>  Contains the platform-native physical key identifier
> 
>  The exact values vary from platform to platform (which is part of why this is a per-platform
>  enum), but the values are primarily tied to the key's physical location on the keyboard.
> 
>  This enum is primarily used to store raw keycodes when Winit doesn't map a given native
>  physical key identifier to a meaningful [`KeyCode`] variant. In the presence of identifiers we
>  haven't mapped for you yet, this lets you use [`KeyCode`] to:
> 
>  - Correctly match key press and release events.
>  - On non-web platforms, support assigning keybinds to virtually any key through a UI.

## Functions

| Function | Summary |
| --- | --- |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./nativekeycode/assert_receiver_is_total_eq.md) |
| `clone(_self)` | [No Documentation 🚧](./nativekeycode/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./nativekeycode/eq.md) |