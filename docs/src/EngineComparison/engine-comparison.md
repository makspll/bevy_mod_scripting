# Engine Comparison

The same two simulations run in BMS (Bevy with LuaJIT scripts), plain Rust Bevy, Godot (GDScript) and Unity (ECS with and without Burst, and a plain C# manager).
Each engine implements them the fastest way it naturally can. The rules and the code are in [`benches/engines`](https://github.com/makspll/bevy_mod_scripting/tree/main/benches/engines).

- **S1 Fountain:** spawning and despawning, vector math over every ball, and one engine-to-script callback per wall hit.
- **S2 Infection:** a spatial grid, sparse queries over infected agents, adding and removing components, and random access by entity.

Each frame is split into phases, and each phase is timed separately. Every point is the median over 60 frames at that entity count, on a GitHub hosted runner. Compare the paths with each other; the absolute numbers are noisy. A path whose checksum did not match the Rust baseline is marked "(diverged)", meaning it did different work.

The charts come from the latest manual run of the `Engine benchmarks` workflow.

## S1 Fountain

![S1 Fountain](https://github.com/makspll/bevy_mod_scripting/releases/download/bench-media/s1.png)

## S2 Infection

![S2 Infection](https://github.com/makspll/bevy_mod_scripting/releases/download/bench-media/s2.png)
