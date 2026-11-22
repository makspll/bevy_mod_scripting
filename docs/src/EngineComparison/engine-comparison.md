# Engine Comparison

The same two simulations run in BMS (Bevy with LuaJIT scripts), plain Rust Bevy, Godot (GDScript) and Unity (ECS with and without Burst, and a plain C# manager).
Each engine implements them the fastest way it naturally can. The rules and the code are in [`benches/engines`](https://github.com/makspll/bevy_mod_scripting/tree/main/benches/engines).

- **S1 Fountain:** spawning and despawning, vector math over every ball, and one engine-to-script callback per wall hit.
- **S2 Infection:** a spatial grid, sparse queries over infected agents, adding and removing components, and random access by entity.

Each frame is split into phases, and each phase is timed separately. Every point is the median over 60 frames at that entity count, on a GitHub hosted runner. Compare the paths with each other; the absolute numbers are noisy. A ✔ means the run's checksum matched the Rust baseline, so it did the same work.

<div id="engine-comparison">Loading results…</div>

<script src="https://cdnjs.cloudflare.com/ajax/libs/Chart.js/4.4.1/chart.umd.min.js"></script>
<script>
(async () => {
  const root = document.getElementById("engine-comparison");
  let data;
  try {
    data = await (await fetch("results.json")).json();
  } catch (e) {
    root.textContent = "No results yet.";
    return;
  }
  if (!data.runs.length) {
    root.textContent = "No results yet.";
    return;
  }
  root.innerHTML = `<p>Generated ${data.generated}</p>`;
  const label = (r) => `${r.engine}/${r.path}${r.verified === false ? " ✘" : r.verified ? " ✔" : ""}`;
  for (const scenario of ["s1", "s2"]) {
    const runs = data.runs.filter((r) => r.scenario === scenario);
    if (!runs.length) continue;
    root.insertAdjacentHTML("beforeend", `<h2>${scenario === "s1" ? "S1 Fountain" : "S2 Infection"}</h2>`);
    const phases = ["frame", ...Object.keys(runs[0].stages[0]).filter((k) => !["target", "alive", "frame"].includes(k))];
    for (const phase of phases) {
      const canvas = document.createElement("canvas");
      root.appendChild(canvas);
      new Chart(canvas, {
        type: "line",
        data: {
          datasets: runs.map((r) => ({
            label: label(r),
            data: r.stages.map((s) => ({ x: s.target, y: s[phase].median })),
          })),
        },
        options: {
          plugins: { title: { display: true, text: `${phase} (ms, median)` } },
          scales: {
            x: { type: "logarithmic", title: { display: true, text: "entities" } },
            y: { type: "logarithmic", title: { display: true, text: "ms" } },
          },
        },
      });
    }
  }
})();
</script>
