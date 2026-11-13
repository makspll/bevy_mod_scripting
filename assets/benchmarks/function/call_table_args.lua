local test_utils = import_namespace("test_utils")

-- built once, so the bench measures converting them, not building them
local map = { name = "x", bg = "#333333", width = 10, text = "y", key = 1 }
local list = { 1, 2, 3, "a", "b" }

function bench()
    test_utils.noop_4_args(map, list, nil, nil)
end
