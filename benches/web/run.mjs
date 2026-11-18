// Runs the scenarios of src/harness.js on each framework in headless Chromium and prints ns per operation,
// the median of `ROUNDS` interleaved rounds. Each scenario runs in a fresh page.
//
//   npm install && npm run bench          (CHROMIUM=/path/to/chromium if it isn't on the PATH)
import { chromium } from "playwright-core";
import { execSync } from "node:child_process";
import { mkdir } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const ROUNDS = 3;
const WARMUP_MS = 2000;
const MEASURE_MS = 8000;
const frameworks = ["react", "solid"];
const scenarios = ["mount", "health", "all"];

const executablePath = process.env.CHROMIUM ?? execSync("command -v chromium").toString().trim();
const browser = await chromium.launch({ executablePath, args: ["--no-sandbox"] });

// `node run.mjs --screenshots`: what each scenario shows after a few changes, into screenshots/
if (process.argv.includes("--screenshots")) {
    // "mount": hidden then shown again, "health": one change, "all": three
    const steps = { mount: 2, health: 1, all: 3 };
    await mkdir("screenshots", { recursive: true });
    for (const framework of frameworks) {
        for (const scenario of scenarios) {
            const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });
            await page.goto(pathToFileURL(resolve(`dist/${framework}.html`)).href);
            await page.evaluate(([s, n]) => window.showScenario(s, n), [scenario, steps[scenario]]);
            await page.screenshot({ path: `screenshots/${framework}-${scenario}.png` });
            await page.close();
        }
    }
    await browser.close();
    process.exit(0);
}

const results = {};
for (let round = 0; round < ROUNDS; round++) {
    for (const framework of frameworks) {
        for (const scenario of scenarios) {
            const page = await browser.newPage();
            await page.goto(pathToFileURL(resolve(`dist/${framework}.html`)).href);
            const result = await page.evaluate(
                ([s, w, m]) => window.runScenario(s, w, m),
                [scenario, WARMUP_MS, MEASURE_MS],
            );
            await page.close();
            (results[`${framework} ${scenario}`] ??= []).push(result.nsPerOp);
            console.error(`round ${round + 1} ${framework} ${scenario}: ${Math.round(result.nsPerOp)} ns`);
        }
    }
}
await browser.close();

const median = (xs) => [...xs].sort((a, b) => a - b)[xs.length >> 1];
console.log(`chromium ${browser.version()}`);
for (const [name, samples] of Object.entries(results)) {
    console.log(`${name}: ${Math.round(median(samples))} ns/op (rounds: ${samples.map(Math.round).join(", ")})`);
}
