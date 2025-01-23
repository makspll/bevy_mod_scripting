info.call("Rhai: the game_of_life.rhai script just got loaded");


fn fetch_life_state() {
    let LifeState = world.get_type_by_name.call("LifeState");
    let Settings = world.get_type_by_name.call("Settings");
    for (v,i) in world.query.call().component.call(LifeState).build.call(){
        return v.components.call()[0]
    }
}


fn on_script_loaded() {
    let LifeState = world.get_type_by_name.call("LifeState");
    let Settings = world.get_type_by_name.call("Settings");

    info.call("Rhai: Hello! I am initiating the game of life simulation state with randomness!");
    info.call("Rhai: Click on the screen to set cells alive after running the `gol start` command");

    let life_state = fetch_life_state!();
    let cells = life_state.cells;
    let cells_len = cells.len.call();
    let x = 0;
    while x < 1000 {
        let index = to_int(floor(rand.call()*cells_len)) ;
        cells[index] = 255;
        x += 1;
    }
}

fn on_click(x,y) {
    let Settings = world.get_type_by_name.call("Settings");
    let LifeState = world.get_type_by_name.call("LifeState");

    info.call("Rhai: Clicked at x: "+ x + ", y: " + y );
    let life_state = fetch_life_state!();
    let cells = life_state.cells;

    let settings = world.get_resource.call(Settings);
    let dimensions = settings.physical_grid_dimensions;
    let screen = settings.display_grid_dimensions;

    let dimension_x = dimensions["_0"];
    let dimension_y = dimensions["_1"];

    let screen_x = screen["_0"];
    let screen_y = screen["_1"];

    let cell_width = screen_x / dimension_x;
    let cell_height = screen_y / dimension_y;
    
    let cell_x = to_int(x / cell_width);
    let cell_y = to_int(y / cell_height);

    let index = cell_y * dimension_x + cell_x;
    let cell_offsets_x = [0, 1, 0, 1, -1, 0, -1, 1, -1];
    let cell_offsets_y = [0, 0, 1, 1, 0, -1, -1, -1, 1];
    for (v,i) in cell_offsets_x {
        let offset_x = cell_offsets_x[i];
        let offset_y = cell_offsets_y[i];
        let new_index = index + offset_x + offset_y * dimension_x;
        if new_index >= 0 && new_index < (dimension_x * dimension_y) {
            cells[new_index] = 255;
        }
    }

}

fn on_update() {
    let LifeState = world.get_type_by_name.call("LifeState");
    let Settings = world.get_type_by_name.call("Settings");

    let life_state = fetch_life_state!();
    let cells = life_state.cells;


    // note that here we do not make use of RhaiProxyable and just go off pure reflection
    let settings = world.get_resource.call(Settings);
    let dimensions = settings.physical_grid_dimensions;
    let dimension_x = dimensions["_0"];
    let dimension_y = dimensions["_1"];
    
    // primitives are passed by value to rhai, keep a hold of old state but turn 255's into 1's
    let prev_state = [];
    for (v,k) in life_state.cells {
        prev_state.push(life_state.cells[k] != 0);
    }

    for i in 0..(dimension_x * dimension_y) {
        let north = prev_state.get(i - dimension_x);
        let south = prev_state.get(i + dimension_x);
        let east = prev_state.get(i + 1);
        let west = prev_state.get(i - 1);
        let northeast = prev_state.get(i - dimension_x + 1);
        let southeast = prev_state.get(i + dimension_x + 1);
        let northwest = prev_state.get(i - dimension_x - 1);
        let southwest = prev_state.get(i + dimension_x - 1);
        
        let neighbours = 0;
        if north == () || north {neighbours+=1}
        if south == () || south {neighbours+=1}
        if east == () || east {neighbours+=1}
        if west == () || west {neighbours+=1}
        if northeast == () || northeast {neighbours+=1}
        if southeast == () || southeast {neighbours+=1}
        if northwest == () || northwest {neighbours+=1}
        if southwest == () || southwest {neighbours+=1}

        // was dead and got 3 neighbours now
        if !prev_state[i] && neighbours == 3 {
            cells[i] = 255;
        }
        // was alive and should die now
        else if prev_state[i] && ((neighbours < 2) || (neighbours > 3)) {
            cells[i] = 0;
        }
    }
}

fn on_script_unloaded() {
    let LifeState = world.get_type_by_name.call("LifeState");
    let Settings = world.get_type_by_name.call("Settings");

    info.call("Rhai: I am being unloaded, goodbye!");

    // set state to 0's
    let life_state = fetch_life_state();
    let cells = life_state.cells;
    for i in 0..cells.len.call() {
        cells[i] = 0;
    }
}