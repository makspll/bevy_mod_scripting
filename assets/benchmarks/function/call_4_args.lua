local test_utils = import_namespace("test_utils")

function bench()
    test_utils.noop_4_args(1,"asd",{1,2,3}, {asd = 1})
end