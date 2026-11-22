//! Shared scenario config, components and helpers.

use std::{collections::HashMap, path::Path};

use bevy::{log::LogPlugin, prelude::*};
use serde_json::{Value, json};

#[derive(Clone, Copy, Debug)]
pub enum Phase {
    Lifecycle,
    Integrate,
    Behaviour,
    Callbacks,
}

impl Phase {
    pub fn name(self) -> &'static str {
        match self {
            Phase::Lifecycle => "lifecycle",
            Phase::Integrate => "integrate",
            Phase::Behaviour => "behaviour",
            Phase::Callbacks => "callbacks",
        }
    }
}

pub struct Scenario {
    pub name: String,
    pub stages: Vec<usize>,
    pub warmup_frames: u32,
    pub measure_frames: u32,
    /// the whole `scenario.json`, scripts get it as a table
    pub raw: Value,
}

impl Scenario {
    pub fn load(path: &Path, name: &str) -> Self {
        let raw: Value =
            serde_json::from_str(&std::fs::read_to_string(path).expect("read scenario.json"))
                .expect("parse scenario.json");
        assert!(raw.get(name).is_some(), "unknown scenario `{name}`");
        Self {
            name: name.to_owned(),
            stages: raw["stages"]
                .as_array()
                .expect("stages")
                .iter()
                .map(|v| v.as_u64().expect("stage") as usize)
                .collect(),
            warmup_frames: raw["warmup_frames"].as_u64().expect("warmup_frames") as u32,
            measure_frames: raw["measure_frames"].as_u64().expect("measure_frames") as u32,
            raw,
        }
    }

    pub fn phases(&self) -> Vec<Phase> {
        match self.name.as_str() {
            "s1" => vec![Phase::Lifecycle, Phase::Integrate, Phase::Callbacks],
            _ => vec![Phase::Lifecycle, Phase::Integrate, Phase::Behaviour],
        }
    }

    pub fn f(&self, key: &str) -> f32 {
        self.raw[&self.name][key].as_f64().expect(key) as f32
    }

    pub fn range(&self, key: &str) -> (f32, f32) {
        let v = &self.raw[&self.name][key];
        (
            v[0].as_f64().expect(key) as f32,
            v[1].as_f64().expect(key) as f32,
        )
    }

    pub fn global(&self, key: &str) -> &Value {
        &self.raw[key]
    }

    pub fn dt(&self) -> f32 {
        self.raw["dt"].as_f64().expect("dt") as f32
    }

    pub fn arena(&self) -> Vec2 {
        Vec2::new(
            self.raw["arena"][0].as_f64().expect("arena") as f32,
            self.raw["arena"][1].as_f64().expect("arena") as f32,
        )
    }
}

/// Park-Miller, exact in doubles so every language produces the same sequence.
#[derive(Resource)]
pub struct Rng(pub u64);

impl Rng {
    pub fn next(&mut self) -> f64 {
        self.0 = self.0 * 48271 % 2147483647;
        self.0 as f64 / 2147483647.0
    }
}

/// Order independent random number for S2.
pub fn hash01(id: u64, frame: u64, salt: u64) -> f64 {
    let mut rng = Rng((id * 7919 + frame * 104729 + salt) % 2147483646 + 1);
    rng.next();
    rng.next()
}

pub fn stats(samples: &mut [f64]) -> Value {
    samples.sort_by(f64::total_cmp);
    let at = |q: f64| samples[((samples.len() - 1) as f64 * q).round() as usize];
    json!({ "median": at(0.5), "p95": at(0.95) })
}

// S1
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct Velocity {
    pub v: Vec3,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct Lifetime {
    pub t: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct Tint {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource, Default)]
pub struct Score {
    pub total: u64,
    pub per_wall: HashMap<String, u64>,
}

pub const WALLS: [&str; 4] = ["left", "right", "floor", "ceiling"];

/// Wall hits reported during `integrate`, dispatched during `callbacks`.
#[derive(Resource, Default)]
pub struct Hits(pub Vec<(Entity, u8)>);

// S2
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct Agent {
    pub id: u32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct Health {
    pub hp: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct Infected {
    pub timer: f32,
    pub strain: String,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Target {
    pub entity: Entity,
}

pub fn base_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        AssetPlugin::default(),
        LogPlugin {
            filter: "warn".into(),
            ..Default::default()
        },
    ))
    .register_type::<Velocity>()
    .register_type::<Lifetime>()
    .register_type::<Tint>()
    .register_type::<Score>()
    .register_type::<Agent>()
    .register_type::<Health>()
    .register_type::<Infected>()
    .register_type::<Target>();
    let mut score = Score::default();
    for wall in WALLS {
        score.per_wall.insert(wall.into(), 0);
    }
    app.insert_resource(score).init_resource::<Hits>();
    app
}
