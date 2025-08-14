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

        impl bevy::app::Plugin for TestPlugin {
            fn build(&self, app: &mut bevy::app::App) {
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
        }

        #[derive(Default, std::fmt::Debug)]
        struct TestRuntime {
            pub invocations: parking_lot::Mutex<
                Vec<(
                    Option<bevy::prelude::Entity>,
                    Option<$ident::script::ScriptId>,
                )>,
            >,
        }

        #[derive(Default, std::fmt::Debug, Clone)]
        struct TestContext {
            pub invocations: Vec<$ident::bindings::script_value::ScriptValue>,
        }
    };
}
