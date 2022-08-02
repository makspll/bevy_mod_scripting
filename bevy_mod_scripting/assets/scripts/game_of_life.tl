math.randomseed(os.time())

function init()

    local LifeState = world:get_type_by_name("LifeState")
    local life_state = world:get_component(entity,LifeState) as types.LuaLifeState
    local cells = life_state.cells as {boolean}

    -- set some cells alive
    for _=1,10000 do 
        local index = math.random(#cells)
        cells[index] = true
    end
end

function on_update()
    local LifeState = world:get_type_by_name("LifeState")
    local Settings = world:get_type_by_name("Settings")

    local life_state = world:get_component(entity,LifeState) as types.LuaLifeState
    local cells = life_state.cells as {boolean}

    -- note that here we do not make use of LuaProxyable and just go off pure reflection
    local settings = world:get_resource(Settings) as {string:any}
    local dimensions = settings.physical_grid_dimensions as {integer}

    
    -- primitives are passed by value to lua, keep a hold of old state
    local prev_state = {}
    for k,v in pairs(life_state.cells as {integer:boolean}) do 
        prev_state[k] = v
    end

    for i=1,(dimensions[0] * dimensions[1]) do 
        local north = prev_state[i - dimensions[0]] and 1 or 0
        local south = prev_state[i + dimensions[0]] and 1 or 0
        local east = prev_state[i + 1] and 1 or 0
        local west = prev_state[i - 1] and 1 or 0
        local northeast = prev_state[i - dimensions[0] + 1] and 1 or 0
        local southeast = prev_state[i + dimensions[0] + 1] and 1 or 0
        local northwest = prev_state[i - dimensions[0] - 1] and 1 or 0
        local southwest = prev_state[i + dimensions[0] - 1] and 1 or 0

        local neighbours = north + south + east + west 
            + northeast + southeast + northwest + southwest 
        
        if prev_state[i] == false then 
            -- if was dead
            cells[i] = neighbours == 3
        else 
            -- if was alive
            cells[i] = (neighbours >= 2) and (neighbours <= 3) 
        end
    end


    -- set some cells alive
    for _=1,50 do 
        local index = math.random(#cells)
        cells[index] = true
    end

end