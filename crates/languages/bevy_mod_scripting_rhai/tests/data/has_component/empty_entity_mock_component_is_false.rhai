let entity = world.spawn_.call();
let type = world._get_mock_component_type.call();

assert(world.has_component.call(entity, type) == false, "Entity should not have component");