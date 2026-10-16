let world = import_namespace.call("bms").world;

// toggles change the value stored on an entity, so store the visibility as a component first
let Visibility = import_namespace.call("camera").Visibility;
let visibility_type = world.get_type_by_name.call("bevy_camera::visibility::Visibility");

let entity = world.spawn_.call();
world.insert_component.call(entity, visibility_type, Visibility.Inherited);
let visibility = world.get_component.call(entity, visibility_type);
visibility.toggle_inherited_visible.call();
assert(visibility == Visibility.Visible, "inherited toggles to visible");
visibility.toggle_inherited_visible.call();
assert(visibility == Visibility.Inherited, "visible toggles back to inherited");

let shown_entity = world.spawn_.call();
world.insert_component.call(shown_entity, visibility_type, Visibility.Visible);
let shown = world.get_component.call(shown_entity, visibility_type);
shown.toggle_visible_hidden.call();
assert(shown == Visibility.Hidden, "visible toggles to hidden");
