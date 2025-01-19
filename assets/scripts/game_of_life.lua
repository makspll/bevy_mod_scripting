LifeState = world.get_type_by_name("LifeState")
Settings = world.get_type_by_name("Settings")

world.info("Lua: The game_of_life.lua script just got loaded")

math.randomseed(os.time())

function fetch_life_state() 
    -- find the first entity with life state 
    local i,v = next(world.query():component(LifeState):build())
    return v:components()[1]
end  

function on_script_loaded()
    world.info("Lua: Hello! I am initiating the game of life simulation state with randomness!")
    world.info("Lua: Click on the screen to set cells alive after running the `gol start` command")
    
    local life_state = fetch_life_state()
    local cells = life_state.cells
    
    -- set some cells alive
    for _=1,1000 do 
        local index = math.random(#cells)
        cells[index] = 255
    end
end  

function on_click(x,y)
    -- get the settings
    world.info("Lua: Clicked at x: " .. x .. " y: " .. y)
    print(entity)
    local life_state = fetch_life_state()
    local cells = life_state.cells

    local settings = world.get_resource(Settings)
    local dimensions = settings.physical_grid_dimensions
    local screen = settings.display_grid_dimensions

    local dimension_x = dimensions._1
    local dimension_y = dimensions._2

    local screen_x = screen._1
    local screen_y = screen._2

    local cell_width = screen_x / dimension_x
    local cell_height = screen_y / dimension_y

    local cell_x = math.floor(x / cell_width)
    local cell_y = math.floor(y / cell_height)

    local index = (cell_y * dimension_x) + cell_x

    -- toggle a bunch of cells around if they exist
    local cell_offsets = {
        {0,0},
        {1,0},
        {0,1},
        {1,1},
        {-1,0},
        {0,-1},
        {-1,-1},
        {1,-1},
        {-1,1}
    }

    for _,offset in pairs(cell_offsets) do 
        local offset_x = offset[1]
        local offset_y = offset[2]
        local new_index = index + offset_x + offset_y * dimension_x
        if new_index > 0 and new_index <= (dimension_x * dimension_y) then
            cells[new_index] = 255
        end
    end
end

function on_update()
    local cells = fetch_life_state().cells
    local settings = world.get_resource(Settings)
    local dimensions = settings.physical_grid_dimensions
    local dimension_x = dimensions._1
    local dimension_y = dimensions._2

    -- primitives are passed by value to lua, keep a hold of old state but turn 255's into 1's
    local prev_state = {}
    for v in pairs(cells) do 
        prev_state[#prev_state+1] = (not(v == 0)) and 1 or 0
    end
    for i=1,(dimension_x * dimension_y) do 
        -- wrap around the north and south edges
        local north = prev_state[i - dimension_x] or prev_state[i + dimension_x * (dimension_y - 1)]
        local south = prev_state[i + dimension_x] or prev_state[i - dimension_x * (dimension_y - 1)]
        local east = prev_state[i + 1] or 0
        local west = prev_state[i - 1] or 0
        local northeast = prev_state[i - dimension_x + 1] or 0
        local southeast = prev_state[i + dimension_x + 1] or 0
        local northwest = prev_state[i - dimension_x - 1] or 0
        local southwest = prev_state[i + dimension_x - 1] or 0

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

function on_script_unloaded()
    world.info("Lua: I am being unloaded, goodbye!")

    -- set state to 0's
    local life_state = fetch_life_state()
    local cells = life_state.cells
    for i=1,#cells do 
        cells[i] = 0
    end
end