function on_test()
    local unpacked_a, unpacked_b = unpack_args({ 1, 2 })

    local packed = pack_args(1, 2)

    assert(packed[1] == unpacked_a,
        "Expected packed[1] to be: " .. tostring(unpacked_a) .. " Got: " .. tostring(packed[1]))
    assert(packed[2] == unpacked_b,
        "Expected packed[2] to be:" .. tostring(unpacked_b) .. " Got: " .. tostring(packed[2]))
end
