register_callback("double_impl", function(x)
    return x * 2
end)

function on_test()
    export_namespace("script.reload", "double", "double_impl")
    return "exported"
end
