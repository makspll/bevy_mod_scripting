# KeyCode

### Unidentified

1. bevy\_input::keyboard::NativeKeyCode

### Backquote

### Backslash

### BracketLeft

### BracketRight

### Comma

### Digit0

### Digit1

### Digit2

### Digit3

### Digit4

### Digit5

### Digit6

### Digit7

### Digit8

### Digit9

### Equal

### IntlBackslash

### IntlRo

### IntlYen

### KeyA

### KeyB

### KeyC

### KeyD

### KeyE

### KeyF

### KeyG

### KeyH

### KeyI

### KeyJ

### KeyK

### KeyL

### KeyM

### KeyN

### KeyO

### KeyP

### KeyQ

### KeyR

### KeyS

### KeyT

### KeyU

### KeyV

### KeyW

### KeyX

### KeyY

### KeyZ

### Minus

### Period

### Quote

### Semicolon

### Slash

### AltLeft

### AltRight

### Backspace

### CapsLock

### ContextMenu

### ControlLeft

### ControlRight

### Enter

### SuperLeft

### SuperRight

### ShiftLeft

### ShiftRight

### Space

### Tab

### Convert

### KanaMode

### Lang1

### Lang2

### Lang3

### Lang4

### Lang5

### NonConvert

### Delete

### End

### Help

### Home

### Insert

### PageDown

### PageUp

### ArrowDown

### ArrowLeft

### ArrowRight

### ArrowUp

### NumLock

### Numpad0

### Numpad1

### Numpad2

### Numpad3

### Numpad4

### Numpad5

### Numpad6

### Numpad7

### Numpad8

### Numpad9

### NumpadAdd

### NumpadBackspace

### NumpadClear

### NumpadClearEntry

### NumpadComma

### NumpadDecimal

### NumpadDivide

### NumpadEnter

### NumpadEqual

### NumpadHash

### NumpadMemoryAdd

### NumpadMemoryClear

### NumpadMemoryRecall

### NumpadMemoryStore

### NumpadMemorySubtract

### NumpadMultiply

### NumpadParenLeft

### NumpadParenRight

### NumpadStar

### NumpadSubtract

### Escape

### Fn

### FnLock

### PrintScreen

### ScrollLock

### Pause

### BrowserBack

### BrowserFavorites

### BrowserForward

### BrowserHome

### BrowserRefresh

### BrowserSearch

### BrowserStop

### Eject

### LaunchApp1

### LaunchApp2

### LaunchMail

### MediaPlayPause

### MediaSelect

### MediaStop

### MediaTrackNext

### MediaTrackPrevious

### Power

### Sleep

### AudioVolumeDown

### AudioVolumeMute

### AudioVolumeUp

### WakeUp

### Meta

### Hyper

### Turbo

### Abort

### Resume

### Suspend

### Again

### Copy

### Cut

### Find

### Open

### Paste

### Props

### Select

### Undo

### Hiragana

### Katakana

### F1

### F2

### F3

### F4

### F5

### F6

### F7

### F8

### F9

### F10

### F11

### F12

### F13

### F14

### F15

### F16

### F17

### F18

### F19

### F20

### F21

### F22

### F23

### F24

### F25

### F26

### F27

### F28

### F29

### F30

### F31

### F32

### F33

### F34

### F35

## Description

>  The key code of a [`KeyboardInput`].
> 
>  ## Usage
> 
>  It is used as the generic `T` value of an [`ButtonInput`] to create a `Res<ButtonInput<KeyCode>>`.
> 
>  Code representing the location of a physical key
>  This mostly conforms to the UI Events Specification's [`KeyboardEvent.code`] with a few
>  exceptions:
>  - The keys that the specification calls `MetaLeft` and `MetaRight` are named `SuperLeft` and
>    `SuperRight` here.
>  - The key that the specification calls "Super" is reported as `Unidentified` here.
> 
>  [`KeyboardEvent.code`]: https://w3c.github.io/uievents-code/#code-value-tables
> 
>  ## Updating
> 
>  The resource is updated inside of the [`keyboard_input_system`].

## Functions

| Function | Summary |
| --- | --- |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./keycode/assert_receiver_is_total_eq.md) |
| `clone(_self)` | [No Documentation 🚧](./keycode/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./keycode/eq.md) |