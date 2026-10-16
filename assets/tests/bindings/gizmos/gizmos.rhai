// rhai can't catch errors raised by bindings, and the test app has no gizmo plugin, so drawing is covered by the lua test
assert(type_of(Gizmos) != "()", "Gizmos exists");
