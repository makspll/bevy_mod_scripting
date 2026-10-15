let Entity = import_namespace.call("ecs").Entity;
let Name = import_namespace.call("ecs").Name;
let entity = world.spawn_.call();
let other = world.spawn_.call();

// entities
assert(entity == entity, "an entity equals itself");
assert(entity != other, "different entities are not equal");
assert(Entity.from_bits.call(entity.to_bits.call()) == entity, "bits round trip");

// names
let name = Name.new_.call("player");
assert(name.as_str.call() == "player", "as_str");
assert(name == Name.new_.call("player"), "names compare with names");

world.insert_component.call(entity, world.get_type_by_name.call("bevy_ecs::name::Name"), Name.new_.call("player"));
assert(world.find_entity_by_name.call("player") == entity, "find_entity_by_name");
assert(type_of(world.find_entity_by_name.call("nobody")) == "()", "missing names give nil");

// hierarchy
world.push_children.call(entity, [other]);
let child_of = world.get_component.call(other, world.get_type_by_name.call("bevy_ecs::hierarchy::ChildOf"));
assert(child_of.parent.call() == entity, "parent");
