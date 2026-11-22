//! The BMS paths: each phase is a script callback run by the harness.

use std::{collections::VecDeque, marker::PhantomData, path::PathBuf};

use bevy::prelude::*;
use bevy_mod_scripting::{
    BMSPlugin,
    bindings::{FunctionCallContext, InteropError, ReflectReference, V, WorldExtensions},
    core::{
        IntoScriptPluginParams,
        event::{CallbackLabel, IntoCallbackLabel},
    },
    prelude::*,
};
use serde_json::{Value, json};

use crate::{
    BenchPath,
    common::{Hits, Phase, Scenario, Velocity},
};

callback_labels!(
    OnSetup => "on_setup",
    OnLifecycle => "on_lifecycle",
    OnIntegrate => "on_integrate",
    OnBehaviour => "on_behaviour",
    OnBounce => "on_bounce",
    OnChecksum => "on_checksum"
);

pub struct ScriptedPath<P> {
    dir: PathBuf,
    attachment: Option<ScriptAttachment>,
    _ph: PhantomData<fn(P)>,
}

impl<P> ScriptedPath<P> {
    pub fn new(dir: PathBuf) -> Self {
        Self {
            dir,
            attachment: None,
            _ph: PhantomData,
        }
    }
}

impl<P: IntoScriptPluginParams> ScriptedPath<P> {
    fn call(&self, world: &mut World, label: CallbackLabel, args: Vec<ScriptValue>) -> ScriptValue {
        RunScriptCallback::<P>::new(
            self.attachment.clone().expect("setup"),
            label.clone(),
            args,
            false,
        )
        .apply(world)
        .unwrap_or_else(|e| panic!("callback `{label}` failed: {e}"))
    }
}

fn to_script(value: &Value) -> ScriptValue {
    match value {
        Value::Null => ScriptValue::Unit,
        Value::Bool(b) => ScriptValue::Bool(*b),
        Value::Number(n) => match n.as_i64() {
            Some(i) => ScriptValue::Integer(i),
            None => ScriptValue::Float(n.as_f64().unwrap_or_default()),
        },
        Value::String(s) => ScriptValue::String(s.clone().into()),
        Value::Array(a) => ScriptValue::List(a.iter().map(to_script).collect::<VecDeque<_>>()),
        Value::Object(o) => {
            ScriptValue::Map(o.iter().map(|(k, v)| (k.clone(), to_script(v))).collect())
        }
    }
}

fn to_json(value: &ScriptValue) -> Value {
    match value {
        ScriptValue::Bool(b) => json!(b),
        ScriptValue::Integer(i) => json!(i),
        ScriptValue::Float(f) => json!(f),
        ScriptValue::String(s) => json!(s),
        ScriptValue::List(l) => Value::Array(l.iter().map(to_json).collect()),
        ScriptValue::Map(m) => {
            Value::Object(m.iter().map(|(k, v)| (k.clone(), to_json(v))).collect())
        }
        other => json!(format!("{other:?}")),
    }
}

impl<P: IntoScriptPluginParams> BenchPath for ScriptedPath<P> {
    fn name(&self) -> String {
        crate::LUA.into()
    }

    fn setup(&mut self, app: &mut App, scenario: &Scenario) {
        app.add_plugins(BMSPlugin);
        // `import_namespace("bench").report_hit(entity, wall)`, the script -> engine call of S1
        NamespaceBuilder::<NamespaceMembers>::in_namespace(app.world_mut(), "bench").register(
            "report_hit",
            |ctxt: FunctionCallContext, entity: V<Entity>, wall: u8| -> Result<(), InteropError> {
                ctxt.world()?
                    .with_resource_mut(|mut hits: Mut<Hits>| hits.0.push((*entity, wall)))
            },
        );
        app.finish();
        app.cleanup();

        let file = self.dir.join(format!("{}.lua", scenario.name));
        let content =
            std::fs::read(&file).unwrap_or_else(|e| panic!("read {}: {e}", file.display()));
        let world = app.world_mut();
        let handle = world
            .get_resource_or_init::<Assets<ScriptAsset>>()
            .add(ScriptAsset {
                content: content.into(),
                language: P::LANGUAGE,
            });
        let attachment = ScriptAttachment::StaticScript(handle);
        AttachScript::<P>::new(attachment.clone()).apply(world);
        app.update();
        self.attachment = Some(attachment);
        self.call(
            app.world_mut(),
            OnSetup::into_callback_label(),
            vec![to_script(&scenario.raw)],
        );
    }

    fn phase(&mut self, world: &mut World, phase: Phase, frame: u32, target: usize) {
        let frame = ScriptValue::Integer(frame as i64);
        match phase {
            Phase::Lifecycle => {
                self.call(
                    world,
                    OnLifecycle::into_callback_label(),
                    vec![frame, ScriptValue::Integer(target as i64)],
                );
            }
            Phase::Integrate => {
                self.call(world, OnIntegrate::into_callback_label(), vec![frame]);
            }
            Phase::Behaviour => {
                self.call(
                    world,
                    OnBehaviour::into_callback_label(),
                    vec![frame, ScriptValue::Integer(target as i64)],
                );
            }
            // one engine -> script call per reported hit
            Phase::Callbacks => {
                let hits = std::mem::take(&mut world.resource_mut::<Hits>().0);
                let label = OnBounce::into_callback_label();
                for (entity, wall) in hits {
                    self.call(
                        world,
                        label.clone(),
                        vec![
                            ScriptValue::Reference(ReflectReference::new_allocated(entity)),
                            ScriptValue::Integer(wall as i64),
                        ],
                    );
                }
            }
        }
    }

    fn alive(&mut self, world: &mut World) -> usize {
        world.query::<&Velocity>().iter(world).count()
    }

    fn checksum(&mut self, world: &mut World) -> Value {
        to_json(&self.call(world, OnChecksum::into_callback_label(), vec![]))
    }
}
