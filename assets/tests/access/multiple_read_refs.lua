local test_utils = import_namespace("test_utils")
local entity = test_utils._make_invalid_entity();
-- does not throw
entity:eq(entity);
