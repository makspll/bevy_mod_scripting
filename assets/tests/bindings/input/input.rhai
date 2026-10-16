let world = import_namespace.call("bms").world;

// the test app has no input resources, so nothing is pressed
let KeyCode = import_namespace.call("input").KeyCode;
let MouseButton = import_namespace.call("input").MouseButton;

// keys
assert(KeyCode.pressed.call(KeyCode.Space) == false, "key pressed");
assert(KeyCode.just_pressed.call(KeyCode.Space) == false, "key just_pressed");
assert(KeyCode.just_released.call(KeyCode.Space) == false, "key just_released");
assert(KeyCode.any_pressed.call([KeyCode.Space, KeyCode.Enter]) == false, "key any_pressed");
assert(KeyCode.any_just_pressed.call([KeyCode.Space, KeyCode.Enter]) == false, "key any_just_pressed");
// rhai writes chained calls on a member back to it, so keep results in temporaries
let pressed = KeyCode.get_pressed.call();
assert(pressed.len() == 0, "key get_pressed");
let just_pressed = KeyCode.get_just_pressed.call();
assert(just_pressed.len() == 0, "key get_just_pressed");

// mouse buttons
assert(MouseButton.pressed.call(MouseButton.Left) == false, "mouse button pressed");
assert(MouseButton.just_pressed.call(MouseButton.Left) == false, "mouse button just_pressed");
assert(MouseButton.just_released.call(MouseButton.Left) == false, "mouse button just_released");
assert(MouseButton.any_pressed.call([MouseButton.Left, MouseButton.Right]) == false, "mouse button any_pressed");
assert(MouseButton.any_just_pressed.call([MouseButton.Left, MouseButton.Right]) == false, "mouse button any_just_pressed");
let pressed = MouseButton.get_pressed.call();
assert(pressed.len() == 0, "mouse button get_pressed");
let just_pressed = MouseButton.get_just_pressed.call();
assert(just_pressed.len() == 0, "mouse button get_just_pressed");

// gamepads
assert(world.gamepads.call().len() == 0, "gamepads");
