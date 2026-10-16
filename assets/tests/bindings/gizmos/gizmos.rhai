// rhai can't catch errors raised by bindings, and the test app has no gizmo plugin, so drawing is covered by the lua test
let Gizmos = import_namespace.call("gizmos").Gizmos;
assert(type_of(Gizmos) != "()", "Gizmos exists");
