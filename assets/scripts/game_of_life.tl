math.randomseed(os.time())

global function init()
    local LifeState = world:get_type_by_name("LifeState")
    local life_state = world:get_component(entity,LifeState) as types.LuaLifeState
    local cells = life_state.cells as {integer}

    -- set some cells alive
    for _=1,10000 do 
        local index = math.random(#cells)
        cells[index] = 255
    end
end

global function on_update()
    local LifeState = world:get_type_by_name("LifeState")
    local Settings = world:get_type_by_name("Settings")

    local life_state = world:get_component(entity,LifeState) as types.LuaLifeState
    local cells = life_state.cells as {integer}

    -- note that here we do not make use of LuaProxyable and just go off pure reflection
    local settings = world:get_resource(Settings) as {string:any}
    local dimensions = settings.physical_grid_dimensions as {integer}

    
    -- primitives are passed by value to lua, keep a hold of old state but turn 255's into 1's
    local prev_state = {}
    for k,v in pairs(life_state.cells as {integer:integer}) do 
        prev_state[k] = (not(v == 0)) and 1 or 0
    end

    for i=1,(dimensions[0] * dimensions[1]) do 
        local north = prev_state[i - dimensions[0]] or 1
        local south = prev_state[i + dimensions[0]] or 1 
        local east = prev_state[i + 1] or 1 
        local west = prev_state[i - 1] or 1
        local northeast = prev_state[i - dimensions[0] + 1] or 1
        local southeast = prev_state[i + dimensions[0] + 1] or 1
        local northwest = prev_state[i - dimensions[0] - 1] or 1
        local southwest = prev_state[i + dimensions[0] - 1] or 1

        local neighbours = north + south + east + west 
            + northeast + southeast + northwest + southwest
        
        -- was dead and got 3 neighbours now
        if prev_state[i] == 0 and neighbours == 3 then
            cells[i] = 255
        -- was alive and should die now
        elseif prev_state[i] == 1 and ((neighbours < 2) or (neighbours > 3)) then
            cells[i] = 0
        end
    end


end