//! Scenario benchmarks for the Bevy side: the plain Rust baseline and the BMS script paths.
//! See `benches/engines/README.md`.

mod common;
mod native;
mod scripted;

use std::{path::PathBuf, time::Instant};

use bevy::prelude::*;
use clap::Parser;
use common::{Phase, Scenario};
use serde_json::{Value, json};

#[derive(Parser)]
struct Args {
    /// `rust` or `lua` (the dialect is the one enabled at build time, `luajit` in CI)
    #[arg(long)]
    path: String,
    /// `s1` or `s2`
    #[arg(long)]
    scenario: String,
    /// comma separated override of the stages in `scenario.json`
    #[arg(long, value_delimiter = ',')]
    stages: Option<Vec<usize>>,
    /// output directory
    #[arg(long, default_value = "benches/engines/results")]
    out: PathBuf,
    // `cargo bench` passes `--bench`
    #[arg(long, hide = true)]
    bench: bool,
}

/// name of the Lua path, the dialect is picked at build time
#[cfg(feature = "luajit")]
pub const LUA: &str = "luajit";
#[cfg(not(feature = "luajit"))]
pub const LUA: &str = "lua54";

/// A path through one engine: runs the phases of a frame on the app's world.
pub trait BenchPath {
    fn name(&self) -> String;
    fn setup(&mut self, app: &mut App, scenario: &Scenario);
    fn phase(&mut self, world: &mut World, phase: Phase, frame: u32, target: usize);
    fn alive(&mut self, world: &mut World) -> usize;
    fn checksum(&mut self, world: &mut World) -> Value;
}

fn main() {
    let args = Args::parse();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("benches/engines");
    let mut scenario = Scenario::load(&root.join("scenario.json"), &args.scenario);
    if let Some(stages) = args.stages {
        scenario.stages = stages;
    }

    let mut path: Box<dyn BenchPath> = match args.path.as_str() {
        "rust" => Box::new(native::NativePath::default()),
        #[cfg(feature = "lua")]
        "lua" => Box::new(scripted::ScriptedPath::<
            bevy_mod_scripting::lua::LuaScriptingPlugin,
        >::new(root.join("bevy/scripts"))),
        other => panic!("unknown or disabled path `{other}`"),
    };

    let mut app = common::base_app();
    path.setup(&mut app, &scenario);
    let world = app.world_mut();

    let phases = scenario.phases();
    let mut frame = 0u32;
    let mut stages = vec![];
    // S2 checksum, sampled every 60 frames
    let mut infected = vec![];
    for &target in &scenario.stages {
        let mut samples: Vec<Vec<f64>> = vec![vec![]; phases.len() + 1];
        let mut alive = 0;
        for i in 0..(scenario.warmup_frames + scenario.measure_frames) {
            let mut frame_ms = 0.0;
            for (p, phase) in phases.iter().enumerate() {
                let start = Instant::now();
                path.phase(world, *phase, frame, target);
                let ms = start.elapsed().as_secs_f64() * 1000.0;
                frame_ms += ms;
                if i >= scenario.warmup_frames {
                    samples[p].push(ms);
                }
            }
            if i >= scenario.warmup_frames {
                samples[phases.len()].push(frame_ms);
                alive += path.alive(world);
            }
            world.clear_trackers();
            if frame.is_multiple_of(60) {
                infected.push(world.query::<&common::Infected>().iter(world).count());
            }
            frame += 1;
        }
        let mut stage = serde_json::Map::new();
        stage.insert("target".into(), json!(target));
        stage.insert(
            "alive".into(),
            json!(alive / scenario.measure_frames as usize),
        );
        for (p, name) in phases.iter().map(|p| p.name()).chain(["frame"]).enumerate() {
            stage.insert(name.into(), common::stats(&mut samples[p]));
        }
        eprintln!(
            "{} {}: {}",
            path.name(),
            args.scenario,
            Value::Object(stage.clone())
        );
        stages.push(Value::Object(stage));
    }

    let result = json!({
        "engine": "bevy",
        "path": path.name(),
        "scenario": args.scenario,
        "frames": frame,
        "stages": stages,
        "checksum": path.checksum(world),
        "infected": infected,
    });
    std::fs::create_dir_all(&args.out).expect("create output dir");
    let file = args
        .out
        .join(format!("{}_{}.json", args.scenario, path.name()));
    std::fs::write(
        &file,
        serde_json::to_string_pretty(&result).expect("serialize"),
    )
    .expect("write results");
    eprintln!("checksum: {}", result["checksum"]);
    eprintln!("wrote {}", file.display());
}
