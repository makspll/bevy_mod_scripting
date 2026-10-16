let test_utils = import_namespace.call("test_utils");
fn bench(){
    test_utils.noop_4_args.call(1,"asd",[1,2,3],#{ asd: 1});
}