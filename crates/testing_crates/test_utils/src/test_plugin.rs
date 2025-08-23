/// Creates a test plugin, but avoids the dependency on bms core
/// requires the root path of BMS core
#[macro_export]
macro_rules! make_test_plugin {
    ($ident: ident) => {
        #[derive(std::fmt::Debug)]
        struct TestPlugin($ident::ScriptingPlugin<Self>);

        impl Default for TestPlugin {
            fn default() -> Self {
                Self($ident::ScriptingPlugin::<Self>::default())
            }
        }

        impl Plugin for TestPlugin {
            fn build(&self, app: &mut App) {
                self.0.build(app);
            }
        }

        impl $ident::IntoScriptPluginParams for TestPlugin {
            type C = TestContext;
            type R = TestRuntime;

            const LANGUAGE: $ident::Language = $ident::Language::Unknown;

            fn build_runtime() -> Self::R {
                TestRuntime {
                    invocations: vec![].into(),
                }
            }

            fn handler() -> $ident::HandlerFn<Self> {
                (|args, context_key, callback, script_ctxt, pre_handling_initializers, runtime| {
                    runtime
                        .invocations
                        .lock()
                        .push((context_key.entity(), Some(context_key.script().id())));
                    Ok($ident::bindings::script_value::ScriptValue::Unit)
                }) as $ident::HandlerFn<Self>
            }

            fn context_loader() -> $ident::ContextLoadFn<Self> {
                (|attachment, content, context_initializers, pre_handling_initializers, runtime| {
                    Ok(TestContext {
                        invocations: vec![],
                    })
                })
            }

            fn context_reloader() -> $ident::ContextReloadFn<Self> {
                (|attachment,
                  content,
                  previous_context,
                  context_initializers,
                  pre_handling_initializers,
                  runtime| {
                    previous_context.invocations.clear();
                    Ok(())
                })
            }
        }

        #[derive(Default, std::fmt::Debug)]
        struct TestRuntime {
            pub invocations:
                parking_lot::Mutex<Vec<(Option<Entity>, Option<$ident::script::ScriptId>)>>,
        }

        #[derive(Default, std::fmt::Debug, Clone)]
        struct TestContext {
            pub invocations: Vec<$ident::bindings::script_value::ScriptValue>,
        }
    };
}
