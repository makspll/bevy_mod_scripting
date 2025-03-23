
reseed()

local matrix = nil
local vector = nil
function pre_bench()
    -- generate random 3x3 matrix and 3x1 vec
    vector = Vec3.new(random(1,999), random(1,999), random(1,999))
    matrix = Mat3.from_cols(
        Vec3.new(random(1,999), random(1,999), random(1,999)),
        Vec3.new(random(1,999), random(1,999), random(1,999)),
        Vec3.new(random(1,999), random(1,999), random(1,999))
    )
end

function bench()
    local mul = matrix * vector
    local add = matrix + vector
    local div = vector / matrix
end