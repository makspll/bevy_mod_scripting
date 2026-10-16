local test_utils = import_namespace("test_utils")
local Mat3 = import_namespace("math").Mat3
local Vec3 = import_namespace("math").Vec3

test_utils.reseed()

local matrix = nil
local vector = nil
function pre_bench()
    -- generate random 3x3 matrix and 3x1 vec
    vector = Vec3.new(test_utils.random(1,999), test_utils.random(1,999), test_utils.random(1,999))
    matrix = Mat3.from_cols(
        Vec3.new(test_utils.random(1,999), test_utils.random(1,999), test_utils.random(1,999)),
        Vec3.new(test_utils.random(1,999), test_utils.random(1,999), test_utils.random(1,999)),
        Vec3.new(test_utils.random(1,999), test_utils.random(1,999), test_utils.random(1,999))
    )
end

function bench()
    local mul = matrix * vector
    local add = matrix + matrix
    local compose = matrix * matrix
end