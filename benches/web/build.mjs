// Bundles the React and Solid benchmark pages into dist/, as production builds.
import { transformAsync } from "@babel/core";
import { build } from "esbuild";
import { mkdir, readFile, writeFile } from "node:fs/promises";

// Solid's JSX compiles to template cloning and fine-grained bindings, through its babel preset
const solidJsx = {
    name: "solid-jsx",
    setup(b) {
        b.onLoad({ filter: /solid\.jsx$/ }, async ({ path }) => {
            const source = await readFile(path, "utf8");
            const { code } = await transformAsync(source, {
                filename: path,
                presets: [["babel-preset-solid", { generate: "dom" }]],
            });
            return { contents: code, loader: "js" };
        });
    },
};

await mkdir("dist", { recursive: true });
for (const app of ["react", "solid"]) {
    await build({
        entryPoints: [`src/${app}.jsx`],
        bundle: true,
        minify: true,
        format: "iife",
        outfile: `dist/${app}.js`,
        jsx: "automatic",
        define: { "process.env.NODE_ENV": '"production"' },
        conditions: app === "solid" ? ["browser", "solid"] : [],
        plugins: app === "solid" ? [solidJsx] : [],
        logLevel: "warning",
    });
    await writeFile(
        `dist/${app}.html`,
        `<!doctype html><html><body><div id="app"></div><script src="${app}.js"></script></body></html>`,
    );
}
