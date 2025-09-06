function contains(table, element)
    for _, value in pairs(table) do
        if value == element then
            return true
        end
    end
    return false
end

local Resource = world.get_type_by_name("TestResource")
local resource = world.get_resource(Resource)

local functions = resource:functions()
assert(#functions > 0, "functions should not be empty")

local available_names = {}

for _, function_ref in pairs(functions) do
    table.insert(available_names, function_ref.name)
end

assert(contains(available_names, "display"),
    "functions should contain display, but got: " .. table.concat(available_names, ", "))
