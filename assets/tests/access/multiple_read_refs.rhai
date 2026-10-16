let test_utils = import_namespace.call("test_utils");
let entity = test_utils._make_invalid_entity.call();
// does not throw
let out = entity.eq.call(entity);