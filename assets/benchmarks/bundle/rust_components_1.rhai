// spawns entities with a script registered bundle of 1 rust components
let members = [];
let values = #{};
for i in 0..1 {
    let name = "BenchComponent" + i;
    let component = types[name];
    members.push(component);
    values[name] = construct.call(component, [i]);
}

let bundle = world.register_new_bundle.call("BenchRustBundle1", members);
let value = DynamicBundle.new_.call(#{ data: values });

fn bench() {
    world.spawn_bundle.call(bundle, value);
}
