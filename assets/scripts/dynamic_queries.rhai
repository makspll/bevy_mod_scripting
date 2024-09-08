fn on_event() {
  let component_a = world.get_type_by_name("ComponentA");
  let component_b = world.get_type_by_name("ComponentB");
  let component_c = world.get_type_by_name("ComponentC");

  // Use with_components/without_components, as the word `with` is
  // reserved in rhai
  for results in world.query([component_a]).with_components([component_b]).without_components([component_c]) {
    print(results.Entity);
  }
}
