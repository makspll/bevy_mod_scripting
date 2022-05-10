
fn on_update(){

    if !("a" in state){
        state.a = 0;
    }

    if (state.a % 100) == 0 {
        let eid = entity_id(entity);
        let str = `${state.a}, entity:${eid}`;
        print_to_console(world,str);
    }

    state.a = state.a + 1;
}