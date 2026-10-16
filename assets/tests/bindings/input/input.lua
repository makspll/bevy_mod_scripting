-- the test app has no input resources, so nothing is pressed

-- keys
assert(KeyCode.pressed(KeyCode.Space) == false, "key pressed")
assert(KeyCode.just_pressed(KeyCode.Space) == false, "key just_pressed")
assert(KeyCode.just_released(KeyCode.Space) == false, "key just_released")
assert(KeyCode.any_pressed({ KeyCode.Space, KeyCode.Enter }) == false, "key any_pressed")
assert(KeyCode.any_just_pressed({ KeyCode.Space, KeyCode.Enter }) == false, "key any_just_pressed")
assert(#KeyCode.get_pressed() == 0, "key get_pressed")
assert(#KeyCode.get_just_pressed() == 0, "key get_just_pressed")

-- mouse buttons
assert(MouseButton.pressed(MouseButton.Left) == false, "mouse button pressed")
assert(MouseButton.just_pressed(MouseButton.Left) == false, "mouse button just_pressed")
assert(MouseButton.just_released(MouseButton.Left) == false, "mouse button just_released")
assert(MouseButton.any_pressed({ MouseButton.Left, MouseButton.Right }) == false, "mouse button any_pressed")
assert(MouseButton.any_just_pressed({ MouseButton.Left, MouseButton.Right }) == false, "mouse button any_just_pressed")
assert(#MouseButton.get_pressed() == 0, "mouse button get_pressed")
assert(#MouseButton.get_just_pressed() == 0, "mouse button get_just_pressed")

-- gamepads
assert(#world.gamepads() == 0, "gamepads")
