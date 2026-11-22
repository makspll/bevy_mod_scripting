# Engine Benchmarks

## Summary

A small set of visual, scenario-driven benchmarks comparing BMS + Bevy against Godot (GDScript), Unity (ECS and non-ECS C#) and a plain Rust Bevy baseline. They run on demand (manual workflow) and update a page in this book with charts and side-by-side videos.

Instead of many micro benchmarks, each engine implements two scenarios. Phase timers inside each frame and an entity count that ramps up over the run let one run produce every chart.

## Rules

- Every engine reads the same `scenario.json`: seed, constants, fixed `dt` and the entity count schedule.
- Each path implements the phases **the fastest way it naturally can**: its own storage, spatial grid and instanced rendering are all allowed. The behaviour and output must match.
- There is no physics engine: the physics is hand written in each path.
- Two passes:
  - **Timing pass:** headless, no rendering.
  - **Video pass:** frame-locked at 60 fps and recorded. It shows simulation time, not wall time.
- Verification:
  - S1: the sum of positions at frame 600 must match the Rust baseline within an epsilon.
  - S2: the infected count over time must match within about 5%.

## Paths

| Path | Notes |
|---|---|
| Bevy, Rust | baseline, no scripts |
| BMS: Lua54, LuaJIT, Rune | script-defined components are `DynamicComponent`s |
| Godot, GDScript | script-defined components are `RefCounted` fields |
| Unity ECS + Burst/jobs | compiled tier, next to the Rust baseline |
| Unity ECS, plain C# | |
| Unity, manager `MonoBehaviour` | one manager looping over arrays (the natural non-ECS layout) |

## Frame phases

```
t0 lifecycle   spawn / despawn
t1 integrate   iteration + value math
t2 behaviour   queries, structural changes, random access
t3 callbacks   engine -> script calls, resources
```

The entity count ramps from 1k to 50k, so every chart is plotted as ms per phase against N.

## Scenarios

### S1 Fountain

An emitter spawns balls. Gravity and drag are integrated with vector math, and balls bounce off the walls. A ball is despawned when its lifetime ends. Each wall hit calls an `on_bounce` script callback, which updates `Score` and tints the ball.

| Component | Kind | Fields |
|---|---|---|
| `Transform` | engine | position |
| `Velocity` | native | `Vec3` |
| `Lifetime` | native | `f32` |
| `Bouncy` | script-defined | restitution, bounces_left |
| `Trail` | script-defined | last 8 positions (collection) |
| `Score` | resource | total, per-wall `HashMap<String, u32>` |

S1 also runs with non-native values (Lua tables or references in BMS, `Dictionary` or `Object` in Godot), so it can compare value kinds.

### S2 Infection

Agents wander on a uniform grid. Contact adds `Infected` (an archetype move), and a timer removes it again. Infected agents chase a `Target` looked up by entity id. Only infected agents are queried.

| Component | Kind | Fields |
|---|---|---|
| `Transform`, `Velocity` | engine / native | |
| `Health` | native | hp `f32` |
| `Infected` | native | timer, strain `String` |
| `Target` | native | `Entity` |
| `Memory` | script-defined | set of recently met entity ids |
| `Grid` | resource | buckets `Vec<Vec<Entity>>` |

In the Rust baseline, script-defined components become plain components. On the Unity ECS path, `String` and maps become `FixedString64Bytes` and `NativeHashMap`.

## Workflow and page

- Trigger: `workflow_dispatch`. It runs one job per path (timing pass, then video pass), then a publish job.
- Unity builds through GameCI, which needs the `UNITY_EMAIL`, `UNITY_PASSWORD` and `UNITY_LICENSE` secrets (Personal license). Godot runs from the official binary.
- Results: `results.json` is drawn with Chart.js on the book page.
- Videos are uploaded as release assets under a fixed `bench-media` tag (`--clobber`) and embedded with `<video>`. If they don't play inline from a release, the fallback is a `gh-pages` branch.

## Open questions

- Where the engine projects live (for example `benches/engines/{bevy,bms,godot,unity}`).
- Whether the publish job pushes the updated page directly or opens a PR.
