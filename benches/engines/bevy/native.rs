//! The plain Rust baseline: every phase is a regular Bevy system, no scripts.

use std::collections::VecDeque;

use bevy::{
    ecs::system::{RunSystemOnce, SystemId},
    prelude::*,
};
use serde_json::{Value, json};

use crate::{
    BenchPath,
    common::{
        Agent, Health, Hits, Infected, Lifetime, Phase, Rng, Scenario, Score, Target, Tint,
        Velocity, WALLS, hash01,
    },
};

/// Script-defined in the script paths, plain components here.
#[derive(Component)]
pub struct Bouncy {
    pub restitution: f32,
    pub left: i32,
}

#[derive(Component, Default)]
pub struct Trail(pub VecDeque<Vec2>);

#[derive(Resource, Clone)]
struct S1Cfg {
    dt: f32,
    arena: Vec2,
    emitter: Vec2,
    gravity: f32,
    drag: f32,
    speed: (f32, f32),
    angle: (f32, f32),
    lifetime: (f32, f32),
    restitution: (f32, f32),
    bounces: (f32, f32),
    spawn_cap_div: usize,
    trail_every: u32,
    trail_len: usize,
}

#[derive(Default)]
pub struct NativePath {
    systems: Vec<(Phase, SystemId<In<(u32, usize)>>)>,
}

impl BenchPath for NativePath {
    fn name(&self) -> String {
        "rust".into()
    }

    fn setup(&mut self, app: &mut App, scenario: &Scenario) {
        let world = app.world_mut();
        world.insert_resource(Rng(scenario.global("seed").as_u64().expect("seed")));
        match scenario.name.as_str() {
            "s1" => {
                let (emitter_x, emitter_y) = scenario.range("emitter");
                world.insert_resource(S1Cfg {
                    dt: scenario.dt(),
                    arena: scenario.arena(),
                    emitter: Vec2::new(emitter_x, emitter_y),
                    gravity: scenario.f("gravity"),
                    drag: scenario.f("drag"),
                    speed: scenario.range("speed"),
                    angle: scenario.range("angle_deg"),
                    lifetime: scenario.range("lifetime"),
                    restitution: scenario.range("restitution"),
                    bounces: scenario.range("bounces"),
                    spawn_cap_div: scenario.f("spawn_cap_div") as usize,
                    trail_every: scenario.f("trail_every") as u32,
                    trail_len: scenario.f("trail_len") as usize,
                });
                self.systems = vec![
                    (Phase::Lifecycle, world.register_system(s1_lifecycle)),
                    (Phase::Integrate, world.register_system(s1_integrate)),
                    (Phase::Callbacks, world.register_system(s1_callbacks)),
                ];
            }
            "s2" => {
                let arena = scenario.arena();
                let cell = scenario.f("cell");
                let (cols, rows) = (
                    (arena.x / cell).ceil() as usize,
                    (arena.y / cell).ceil() as usize,
                );
                world.insert_resource(Grid {
                    cols,
                    rows,
                    cells: vec![vec![]; cols * rows],
                });
                world.init_resource::<NextId>();
                world.insert_resource(S2Cfg {
                    dt: scenario.dt(),
                    arena,
                    cell,
                    speed: scenario.f("speed"),
                    chase_speed: scenario.f("chase_speed"),
                    jitter: scenario.f("jitter"),
                    radius: scenario.f("radius"),
                    p_infect: scenario.f("p_infect") as f64,
                    infect_time: scenario.f("infect_time"),
                    damage_per_s: scenario.f("damage_per_s"),
                    hp: scenario.f("hp"),
                    seed_every: scenario.f("seed_every") as u32,
                    strains: scenario.raw["s2"]["strains"]
                        .as_array()
                        .expect("strains")
                        .iter()
                        .map(|s| s.as_str().expect("strain").to_owned())
                        .collect(),
                    memory_cap: scenario.f("memory_cap") as usize,
                    spawn_cap_div: scenario.raw["s1"]["spawn_cap_div"]
                        .as_u64()
                        .expect("spawn_cap_div") as usize,
                });
                self.systems = vec![
                    (Phase::Lifecycle, world.register_system(s2_lifecycle)),
                    (Phase::Integrate, world.register_system(s2_integrate)),
                    (Phase::Behaviour, world.register_system(s2_behaviour)),
                ];
            }
            other => panic!("unknown scenario `{other}`"),
        }
    }

    fn phase(&mut self, world: &mut World, phase: Phase, frame: u32, target: usize) {
        let (_, id) = self
            .systems
            .iter()
            .find(|(p, _)| p.name() == phase.name())
            .expect("phase");
        world
            .run_system_with(*id, (frame, target))
            .expect("run phase");
    }

    fn alive(&mut self, world: &mut World) -> usize {
        world
            .run_system_once(|q: Query<(), With<Velocity>>| q.iter().count())
            .expect("count")
    }

    fn checksum(&mut self, world: &mut World) -> Value {
        world
            .run_system_once(|q: Query<&Transform, With<Velocity>>, score: Res<Score>| {
                let sum: f64 = q
                    .iter()
                    .map(|t| (t.translation.x + t.translation.y) as f64)
                    .sum();
                json!({ "alive": q.iter().count(), "sum": sum, "score": score.total })
            })
            .expect("checksum")
    }
}

fn lerp((a, b): (f32, f32), t: f64) -> f32 {
    a + (b - a) * t as f32
}

fn s1_lifecycle(
    In((_, target)): In<(u32, usize)>,
    mut commands: Commands,
    q: Query<(Entity, &Lifetime)>,
    mut rng: ResMut<Rng>,
    cfg: Res<S1Cfg>,
) {
    let mut alive = 0;
    for (entity, life) in &q {
        if life.t <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            alive += 1;
        }
    }
    let count = target
        .saturating_sub(alive)
        .min((target / cfg.spawn_cap_div).max(1));
    for _ in 0..count {
        let angle = lerp(cfg.angle, rng.next()).to_radians();
        let speed = lerp(cfg.speed, rng.next());
        let lifetime = lerp(cfg.lifetime, rng.next());
        let restitution = lerp(cfg.restitution, rng.next());
        let left = cfg.bounces.0 as i32
            + (rng.next() * (cfg.bounces.1 - cfg.bounces.0 + 1.0) as f64).floor() as i32;
        commands.spawn((
            Transform::from_xyz(cfg.emitter.x, cfg.emitter.y, 0.0),
            Velocity {
                v: Vec3::new(angle.cos() * speed, angle.sin() * speed, 0.0),
            },
            Lifetime { t: lifetime },
            Tint {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
            Bouncy { restitution, left },
            Trail::default(),
        ));
    }
}

fn s1_integrate(
    In((frame, _)): In<(u32, usize)>,
    mut q: Query<(
        Entity,
        &mut Transform,
        &mut Velocity,
        &mut Lifetime,
        &Bouncy,
        &mut Trail,
    )>,
    mut hits: ResMut<Hits>,
    cfg: Res<S1Cfg>,
) {
    let dt = cfg.dt;
    let gravity = Vec3::new(0.0, cfg.gravity * dt, 0.0);
    let damp = 1.0 - cfg.drag * dt;
    let push_trail = frame.is_multiple_of(cfg.trail_every);
    let (w, h) = (cfg.arena.x, cfg.arena.y);
    for (entity, mut transform, mut vel, mut life, bouncy, mut trail) in &mut q {
        let mut v = (vel.v + gravity) * damp;
        let mut p = transform.translation + v * dt;
        let mut wall = 0;
        if p.x < 0.0 {
            p.x = -p.x;
            v.x = -v.x;
            wall = 1;
        } else if p.x > w {
            p.x = 2.0 * w - p.x;
            v.x = -v.x;
            wall = 2;
        }
        if p.y < 0.0 {
            p.y = -p.y;
            v.y = -v.y;
            wall = 3;
        } else if p.y > h {
            p.y = 2.0 * h - p.y;
            v.y = -v.y;
            wall = 4;
        }
        if wall != 0 {
            v *= bouncy.restitution;
            hits.0.push((entity, wall));
        }
        vel.v = v;
        transform.translation = p;
        life.t -= dt;
        if push_trail {
            trail.0.push_front(p.truncate());
            trail.0.truncate(cfg.trail_len);
        }
    }
}

fn on_bounce(
    entity: Entity,
    wall: u8,
    score: &mut Score,
    q: &mut Query<(&mut Bouncy, &mut Tint, &mut Lifetime)>,
) {
    score.total += 1;
    *score
        .per_wall
        .get_mut(WALLS[wall as usize - 1])
        .expect("wall") += 1;
    let Ok((mut bouncy, mut tint, mut life)) = q.get_mut(entity) else {
        return;
    };
    bouncy.left -= 1;
    let k = bouncy.left as f32 / 6.0;
    *tint = Tint { r: 1.0, g: k, b: k };
    if bouncy.left <= 0 {
        life.t = 0.0;
    }
}

fn s1_callbacks(
    _: In<(u32, usize)>,
    mut hits: ResMut<Hits>,
    mut score: ResMut<Score>,
    mut q: Query<(&mut Bouncy, &mut Tint, &mut Lifetime)>,
) {
    for (entity, wall) in hits.0.drain(..) {
        on_bounce(entity, wall, &mut score, &mut q);
    }
}

/// Script-defined in the script paths.
#[derive(Component, Default)]
pub struct Memory(pub Vec<u32>);

#[derive(Resource, Clone)]
struct S2Cfg {
    dt: f32,
    arena: Vec2,
    cell: f32,
    speed: f32,
    chase_speed: f32,
    jitter: f32,
    radius: f32,
    p_infect: f64,
    infect_time: f32,
    damage_per_s: f32,
    hp: f32,
    seed_every: u32,
    strains: Vec<String>,
    memory_cap: usize,
    spawn_cap_div: usize,
}

#[derive(Resource, Default)]
struct NextId(u32);

/// Agents bucketed by cell, rebuilt every frame.
#[derive(Resource, Default)]
struct Grid {
    cols: usize,
    rows: usize,
    cells: Vec<Vec<(Entity, u32, Vec2, bool)>>,
}

fn s2_lifecycle(
    In((_, target)): In<(u32, usize)>,
    mut commands: Commands,
    q: Query<(Entity, &Health)>,
    mut next_id: ResMut<NextId>,
    cfg: Res<S2Cfg>,
) {
    let mut alive = 0;
    for (entity, health) in &q {
        if health.hp <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            alive += 1;
        }
    }
    let count = target
        .saturating_sub(alive)
        .min((target / cfg.spawn_cap_div).max(1));
    for _ in 0..count {
        let id = next_id.0;
        next_id.0 += 1;
        let h = |salt| hash01(id as u64, 0, salt) as f32;
        let a = h(3) * std::f32::consts::TAU;
        let mut agent = commands.spawn((
            Transform::from_xyz(h(1) * cfg.arena.x, h(2) * cfg.arena.y, 0.0),
            Velocity {
                v: Vec3::new(a.cos(), a.sin(), 0.0) * cfg.speed,
            },
            Agent { id },
            Health { hp: cfg.hp },
            Memory::default(),
        ));
        if id.is_multiple_of(cfg.seed_every) {
            agent.insert(Infected {
                timer: cfg.infect_time,
                strain: cfg.strains[(id / cfg.seed_every) as usize % cfg.strains.len()].clone(),
            });
        }
    }
}

fn s2_integrate(
    In((frame, _)): In<(u32, usize)>,
    mut infected: Query<(&mut Health, &mut Infected)>,
    targets: Query<(Entity, &Target)>,
    mut agents: Query<(Entity, &Agent, &mut Transform, &mut Velocity)>,
    cfg: Res<S2Cfg>,
) {
    let dt = cfg.dt;
    for (mut health, mut inf) in &mut infected {
        health.hp -= cfg.damage_per_s * dt;
        inf.timer -= dt;
    }
    // random access: read every chaser's target position first
    let chase: bevy::platform::collections::HashMap<Entity, Vec3> = targets
        .iter()
        .filter_map(|(e, t)| {
            let target = agents.get(t.entity).ok()?.2.translation;
            let me = agents.get(e).ok()?.2.translation;
            Some((e, (target - me).normalize_or_zero() * cfg.chase_speed))
        })
        .collect();
    for (entity, agent, mut transform, mut vel) in &mut agents {
        if let Some(v) = chase.get(&entity) {
            vel.v = *v;
        } else {
            let a = ((hash01(agent.id as u64, frame as u64, 4) - 0.5) as f32) * cfg.jitter;
            let (s, c) = a.sin_cos();
            vel.v = Vec3::new(vel.v.x * c - vel.v.y * s, vel.v.x * s + vel.v.y * c, 0.0);
        }
        let mut p = transform.translation + vel.v * dt;
        p.x = p.x.rem_euclid(cfg.arena.x);
        p.y = p.y.rem_euclid(cfg.arena.y);
        transform.translation = p;
    }
}

fn s2_behaviour(
    In((frame, target)): In<(u32, usize)>,
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    agents: Query<(Entity, &Agent, &Transform, Has<Infected>)>,
    infected: Query<(Entity, &Agent, &Transform, &Infected, Option<&Target>)>,
    mut memory: Query<&mut Memory>,
    cfg: Res<S2Cfg>,
) {
    let (cols, rows) = (grid.cols, grid.rows);
    let p_infect = cfg.p_infect * 5000.0 / target as f64;
    let cell_of = |p: Vec2| {
        (
            ((p.x / cfg.cell) as usize).min(cols - 1),
            ((p.y / cfg.cell) as usize).min(rows - 1),
        )
    };
    grid.cells.iter_mut().for_each(Vec::clear);
    for (entity, agent, transform, is_infected) in &agents {
        let p = transform.translation.truncate();
        let (cx, cy) = cell_of(p);
        grid.cells[cy * cols + cx].push((entity, agent.id, p, is_infected));
    }

    let mut infections: bevy::platform::collections::HashMap<Entity, (u32, &str)> =
        Default::default();
    for (entity, agent, transform, inf, target) in &infected {
        if inf.timer <= 0.0 {
            commands.entity(entity).remove::<(Infected, Target)>();
            continue;
        }
        let p = transform.translation.truncate();
        let (cx, cy) = cell_of(p);
        let mut nearest: Option<(f32, u32, Entity)> = None;
        for y in cy.saturating_sub(1)..=(cy + 1).min(rows - 1) {
            for x in cx.saturating_sub(1)..=(cx + 1).min(cols - 1) {
                for &(other, id, q, other_infected) in &grid.cells[y * cols + x] {
                    if other_infected {
                        continue;
                    }
                    let d = p.distance(q);
                    if d < cfg.radius {
                        if let Ok(mut mem) = memory.get_mut(other)
                            && mem.0.len() < cfg.memory_cap
                            && !mem.0.contains(&agent.id)
                        {
                            mem.0.push(agent.id);
                        }
                        if hash01(id as u64, frame as u64, 5) < p_infect {
                            let entry = infections.entry(other).or_insert((agent.id, &inf.strain));
                            if agent.id < entry.0 {
                                *entry = (agent.id, &inf.strain);
                            }
                        }
                    }
                    if nearest.is_none_or(|(nd, nid, _)| d < nd || (d == nd && id < nid)) {
                        nearest = Some((d, id, other));
                    }
                }
            }
        }
        match target {
            None => {
                if let Some((_, _, other)) = nearest {
                    commands.entity(entity).insert(Target { entity: other });
                }
            }
            Some(t) => {
                if agents.get(t.entity).ok().is_none_or(|(_, _, _, inf)| inf) {
                    commands.entity(entity).remove::<Target>();
                }
            }
        }
    }
    for (entity, (_, strain)) in infections {
        commands.entity(entity).insert(Infected {
            timer: cfg.infect_time,
            strain: strain.to_owned(),
        });
    }
}
