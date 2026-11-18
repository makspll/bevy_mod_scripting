// Times the reactive_ui benchmark scenarios on a web framework, the way `run_reactive_ui_benchmark` times BMS:
// preparing the change is not timed (BMS: the `bench_step` callback), applying it until the DOM is updated is.
// Nothing yields to the browser while timing, so neither layout nor paint runs: only framework work is measured,
// as the BMS benches stop at handing the tree to bevy_ui.
//
// An app implements:
//   setup(scenario)   mounts what the scenario updates ("mount" mounts nothing)
//   op(scenario, n)   prepares change `n` and returns a function applying it synchronously
//
// Scenarios: "mount" mounts or unmounts the 200 slot inventory (alternating, like BMS' "inventory mount or unmount 200"),
// "health" changes the health bar, "all" changes the count of every slot.

const MIN_BATCH_MS = 20;

export function register(app) {
    registerShow(app);
    window.runScenario = (scenario, warmupMs, measureMs) => {
        app.setup(scenario);
        let n = 0;
        // runs `k` prepared changes, returns the time per change in ms
        const batch = (k) => {
            const ops = [];
            for (let i = 0; i < k; i++) ops.push(app.op(scenario, n++));
            const start = performance.now();
            for (const op of ops) op();
            return (performance.now() - start) / k;
        };
        // even sizes, so "mount" ends every batch unmounted
        let k = 2;
        while (batch(k) * k < MIN_BATCH_MS) k *= 2;
        const until = (ms, f) => {
            const end = performance.now() + ms;
            while (performance.now() < end) f();
        };
        until(warmupMs, () => batch(k));
        const samples = [];
        until(measureMs, () => samples.push(batch(k)));
        samples.sort((a, b) => a - b);
        return { nsPerOp: samples[samples.length >> 1] * 1e6, batches: samples.length, batchSize: k };
    };
}

/// For screenshots: sets the scenario up and applies `steps` changes, untimed.
export function registerShow(app) {
    window.showScenario = (scenario, steps) => {
        app.setup(scenario);
        for (let n = 1; n <= steps; n++) app.op(scenario, n)();
    };
}

export const SLOTS = 200;

export const initialItems = () =>
    Array.from({ length: SLOTS }, (_, i) => ({ name: `item ${i + 1}`, count: 1 }));
