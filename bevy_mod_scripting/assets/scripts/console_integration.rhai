
fn on_update(){

    if !("a" in state){
        state.a = 0;
    }

    if (state.a % 100) == 0 {
        print_to_console(world,`${state.a}, entity:${ entity_id(entity)}`);
    }

    state.a = state.a + 1;
}