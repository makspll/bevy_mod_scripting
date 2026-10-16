local world = import_namespace("bms").world
local Entity = import_namespace("ecs").Entity
local Name = import_namespace("ecs").Name
local entity = world.spawn()
local other = world.spawn()

-- entities
assert(entity == entity, "an entity equals itself")
assert(entity ~= other, "different entities are not equal")
assert(Entity.from_bits(entity:to_bits()) == entity, "bits round trip")
assert(entity:index() ~= other:index(), "live entities have different indices")
assert(entity:generation() >= 0, "generation")

-- names
local name = Name.new("player")
assert(name:as_str() == "player", "as_str")
assert(name:eq("player"), "names compare with strings")
assert(name == Name.new("player"), "names compare with names")
name:set("hero")
assert(name:as_str() == "hero", "set")

world.insert_component(entity, world.get_type_by_name("bevy_ecs::name::Name"), Name.new("player"))
assert(world.find_entity_by_name("player") == entity, "find_entity_by_name")
assert(world.find_entity_by_name("nobody") == nil, "missing names give nil")

-- hierarchy
world.push_children(entity, { other })
local child_of = world.get_component(other, world.get_type_by_name("bevy_ecs::hierarchy::ChildOf"))
assert(child_of:parent() == entity, "parent")
