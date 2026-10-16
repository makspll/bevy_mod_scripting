let bms = import_namespace.call("bms");

fn on_test() {
    // the running script's entity and handle are looked up when asked for
    assert(type_of(bms.current_entity.call()) != "()", "the script is attached to an entity");
    assert(type_of(bms.current_script.call()) != "()", "the running script has a handle");
    bms.log_info.call("logged through the bms namespace");
    return true;
}
