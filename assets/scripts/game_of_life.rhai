fn init() {
    let LifeState = world.get_type_by_name("LifeState");
    let life_state = world.get_component(entity,LifeState);
    let cells = life_state.cells;

    // set some cells alive
    for x in 1..10000 {
        let index = rand(0..cells.len());
        cells[index] = 255;
    }
}

fn on_update() {

    let LifeState = world.get_type_by_name("LifeState");
    let Settings = world.get_type_by_name("Settings");

    let life_state = world.get_component(entity,LifeState);
    let cells = life_state.cells;


    // note that here we do not make use of RhaiProxyable and just go off pure reflection
    let settings = world.get_resource(Settings);
    let dimensions = settings.physical_grid_dimensions;

    
    // primitives are passed by value to rhai, keep a hold of old state but turn 255's into 1's
    let prev_state = [];
    for (v,k) in life_state.cells {
        prev_state.push(life_state.cells[k] != 0);
    }

    for i in 0..(dimensions[0] * dimensions[1]) {
        let north = prev_state.get(i - dimensions[0]);
        let south = prev_state.get(i + dimensions[0]);
        let east = prev_state.get(i + 1);
        let west = prev_state.get(i - 1);
        let northeast = prev_state.get(i - dimensions[0] + 1);
        let southeast = prev_state.get(i + dimensions[0] + 1);
        let northwest = prev_state.get(i - dimensions[0] - 1);
        let southwest = prev_state.get(i + dimensions[0] - 1);
        
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