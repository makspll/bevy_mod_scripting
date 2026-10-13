// spawns entities with a script registered bundle of 1 script components
let members = [];
let values = #{};
for i in 0..1 {
    let name = "BenchScriptComponent" + i;
    members.push(world.register_new_component.call(name));
    values[name] = i;
}

let bundle = world.register_new_bundle.call("BenchScriptBundle1", members);
let value = DynamicBundle.new_.call(#{ data: values });

fn bench() {
    world.spawn_bundle.call(bundle, value);
}
