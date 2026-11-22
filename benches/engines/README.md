# Engine benchmarks

Scenario benchmarks comparing BMS + Bevy, Godot, Unity and a plain Rust Bevy baseline.
Every path reads `scenario.json` and implements the same behaviour, the fastest way it naturally can.

Each frame runs these phases in order, and each one is timed separately:

| phase | covers |
|---|---|
| `lifecycle` | despawning and spawning |
| `integrate` | iteration and vector math |
| `behaviour` | grid building, sparse queries, structural changes, random access (S2 only) |
| `callbacks` | engine-to-script calls and resource writes (S1 only) |

For each value in `stages`, the target entity count is held for `warmup_frames` frames, then measured over `measure_frames` frames.

## Run

```sh
cargo bench --bench engines --features luajit -- --path lua --scenario s1
cargo bench --bench engines -- --path rust --scenario s2 --stages 1000,5000
```

Results are written to `results/<scenario>_<path>.json`.

## Determinism

- RNG is the Park-Miller generator, `x = x * 48271 % 2147483647`, returning `x / 2147483647`. It is exact in doubles, so every language gets the same sequence.
- S1 draws from one global RNG, seeded with `seed`, only while spawning. The draws are made in this order: angle, speed, lifetime, restitution, bounces.
- S2 draws from `hash(id, frame, salt)`, which does not depend on iteration order: `x = (id * 7919 + frame * 104729 + salt) % 2147483646 + 1`, followed by two Park-Miller steps.

## S1 Fountain

1. `lifecycle`:
   - Despawn every entity with `Lifetime.t <= 0`.
   - Then spawn `min(target - alive, max(1, floor(target / spawn_cap_div)))` balls at `emitter`. Each gets:
     - `Velocity` with an angle from `angle_deg` and a speed from `speed`.
     - `Lifetime` from `lifetime`.
     - `Tint` `(1, 1, 1)`.
     - `Bouncy`, a script component: `restitution`, plus `left` bounces drawn from the integer range in `bounces`.
     - `Trail`, a script component: an empty list.
2. `integrate`, for every ball:
   - `v = (v + (0, gravity * dt)) * (1 - drag * dt)`, then `p += v * dt`.
   - Walls reflect: `p.x < 0` gives `p.x = -p.x, v.x = -v.x` (wall 1). Wall 2 is `p.x > W`, wall 3 is `p.y < 0`, wall 4 is `p.y > H`. The y check comes after the x check, and the last wall hit wins.
   - On a hit: `v *= restitution`, and the hit `(entity, wall)` is reported to the engine.
   - `Lifetime.t -= dt`.
   - When `frame % trail_every == 0`, push `(x, y)` to the front of `Trail` and keep at most `trail_len` entries.
3. `callbacks`: the engine calls `on_bounce(entity, wall)` once per reported hit. The script then:
   - Increments `Score.total` and `Score.per_wall[left|right|floor|ceiling]`.
   - Decrements `Bouncy.left` and sets `Tint = (1, k, k)` with `k = left / 6`.
   - If `left <= 0`, sets `Lifetime.t = 0`.

Checksum: the alive count, the sum of `x + y` over all balls, and `Score.total`.

## S2 Infection

Agents wander through the arena, which wraps at the edges. The infection spreads on contact.

1. `lifecycle`:
   - Despawn every agent with `Health.hp <= 0`.
   - Spawn up to the same cap as S1. Agent ids come from a global counter starting at 0. Each new agent gets:
     - position `(hash(id,0,1) * W, hash(id,0,2) * H)`;
     - velocity `speed * (cos a, sin a)`, with `a = hash(id,0,3) * 2pi`;
     - `Health` set to `hp`;
     - `Memory`, a script component: an empty list.
   - When `id % seed_every == 0`, the agent also gets `Infected { timer: infect_time, strain: strains[(id / seed_every) % 4 + 1] }`.
2. `integrate`:
   - Infected agents: `hp -= damage_per_s * dt` and `timer -= dt`.
   - An agent with a live `Target` sets `v = normalize(target_pos - p) * chase_speed`, using random access to read the target's position.
   - Every other agent rotates `v` by `(hash(id, frame, 4) - 0.5) * jitter`.
   - Then `p += v * dt`, and `p` wraps into the arena.
3. `behaviour`:
   - Bucket every agent into a grid of `cell`-sized cells, without wrapping.
   - Recover each infected agent with `timer <= 0`: remove `Infected` and `Target`. It no longer counts as infected this frame.
   - For every other infected agent `I` (a sparse query), and every healthy agent `A` in the 3x3 cells around it:
     - If `dist < radius`, add `I.id` to `A.Memory`, unless it's already there or the memory holds `memory_cap` ids.
     - Also, if `hash(A.id, frame, 5) < p_infect * 5000 / target`, `A` becomes infected by the lowest-id infector, taking its strain. The scaling keeps the infected share similar at every density.
   - `I` with no `Target`: set `Target` to the nearest healthy agent in its 3x3 cells. Ties go to the lowest id.
   - `I` whose target is gone or infected: remove `Target`.
   - Insert the new infections: `Infected { timer: infect_time, strain }`.
   - "Healthy" and "infected" always mean the state at the start of the phase.

Checksum: the alive count, plus the infected count sampled every 60 frames.
