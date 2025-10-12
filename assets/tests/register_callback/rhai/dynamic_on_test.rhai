fn on_script_loaded(){
    register_callback("on_test", dynamic_on_test);
}

fn dynamic_on_test(){
    register_callback("on_test_last", dynamic_on_test_last);
    return "on test: I am dynamically registered from a normal callback!";
}

fn dynamic_on_test_last(){
    return "on test last: I am dynamically registered from another dynamic callback!";
}
