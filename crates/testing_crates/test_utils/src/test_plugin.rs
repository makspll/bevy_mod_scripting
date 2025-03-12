/// Creates a test plugin, but avoids the dependency on bms core
/// requires the root path of BMS core
#[macro_export]
macro_rules! make_test_plugin {
    ($ident: ident) => {
        // #[derive(Default)]
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

        #[derive(Default)]
        struct TestRuntime {
            pub invocations:
                parking_lot::Mutex<Vec<(bevy::prelude::Entity, $ident::script::ScriptId)>>,
        }

        #[derive(Default)]
        struct TestContext {
            pub invocations: Vec<$ident::bindings::script_value::ScriptValue>,
        }
    };
}
