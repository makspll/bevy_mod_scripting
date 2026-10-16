-- toggles change the value stored on an entity, so store the visibility as a component first
local Vec3 = import_namespace("math").Vec3
local Visibility = import_namespace("camera").Visibility
local visibility_type = world.get_type_by_name("bevy_camera::visibility::Visibility")
local function visibility_of(value)
    local entity = world.spawn()
    world.insert_component(entity, visibility_type, value)
    return world.get_component(entity, visibility_type)
end

-- toggling inherited and visible keeps hidden entities hidden
local visibility = visibility_of(Visibility.Inherited)
visibility:toggle_inherited_visible()
assert(visibility == Visibility.Visible, "inherited toggles to visible")
visibility:toggle_inherited_visible()
assert(visibility == Visibility.Inherited, "visible toggles back to inherited")
local hidden = visibility_of(Visibility.Hidden)
hidden:toggle_inherited_visible()
assert(hidden == Visibility.Hidden, "hidden stays hidden")

-- toggling visible and hidden keeps inherited entities inherited
local shown = visibility_of(Visibility.Visible)
shown:toggle_visible_hidden()
assert(shown == Visibility.Hidden, "visible toggles to hidden")
shown:toggle_visible_hidden()
assert(shown == Visibility.Visible, "hidden toggles to visible")
local inherited = visibility_of(Visibility.Inherited)
inherited:toggle_visible_hidden()
assert(inherited == Visibility.Inherited, "inherited stays inherited")

-- values of the wrong type are an error, not a crash
assert(not pcall(function() world.insert_component(world.spawn(), visibility_type, Vec3.zero()) end), "wrong type errors")
