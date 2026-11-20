-- 100 operations per call: the cost of one is (this - `empty loop`) / 100
-- the same operations on values living in the VM: a table with a metatable, written in Lua
local Vec = {}
Vec.__index = Vec
function Vec.new(x, y, z) return setmetatable({ x = x, y = y, z = z }, Vec) end
function Vec.length(v) return math.sqrt(v.x * v.x + v.y * v.y + v.z * v.z) end
function Vec.__add(a, b) return Vec.new(a.x + b.x, a.y + b.y, a.z + b.z) end
local v = Vec.new(1.0, 2.0, 3.0)
local w = Vec.new(4.0, 5.0, 6.0)
local t = { translation = Vec.new(1.0, 2.0, 3.0) }

function bench()
    for i = 1, 100 do
        local c = Vec.new(1.0, 2.0, 3.0)
    end
end
